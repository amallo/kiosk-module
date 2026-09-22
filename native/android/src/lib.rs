mod adapters;
mod app_context;
mod jni_bridge;

use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;
use std::time::Duration;

use app_context::AppContext;
use kiosk_core::usecases::grant_time_use_case::GrantTimeArgs;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong};
use jni::JNIEnv;

static LOGGER_INIT: Once = Once::new();

fn init_logger_once() {
    LOGGER_INIT.call_once(|| {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info),
        );
    });
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_margelo_nitro_kioskmodule_KioskNative_nativeInit(
    env: JNIEnv,
    _class: JClass,
    storage_bridge: JObject,
) -> jlong {
    init_logger_once();

    if storage_bridge.is_null() {
        log::error!("nativeInit: storage_bridge argument is null");
        return 0;
    }

    let vm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => {
            log::error!("nativeInit: failed to obtain JavaVM handle");
            return 0;
        }
    };

    let bridge = match env.new_global_ref(&storage_bridge) {
        Ok(global) => global,
        Err(_) => {
            log::error!("nativeInit: failed to create GlobalRef for storage_bridge");
            return 0;
        }
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| AppContext::new(vm, bridge)));
    match result {
        Ok(ctx) => Box::into_raw(Box::new(ctx)) as jlong,
        Err(_) => {
            log::error!("nativeInit: panic while building AppContext");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_margelo_nitro_kioskmodule_KioskNative_nativeLockDevice(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return jni_bridge::ERR_INTERNAL_PANIC;
    }
    let ctx = unsafe { &*(handle as *const AppContext) };

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        ctx.runtime.block_on(ctx.lock_use_case.execute())
    }));

    match result {
        Ok(use_case_result) => jni_bridge::map_lock_result(use_case_result),
        Err(_) => {
            log::error!("nativeLockDevice: panic during execute()");
            jni_bridge::ERR_INTERNAL_PANIC
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_margelo_nitro_kioskmodule_KioskNative_nativeGrantTime(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    duration_secs: jlong,
    pin: jint,
) -> jint {
    if handle == 0 {
        return jni_bridge::ERR_INTERNAL_PANIC;
    }
    let ctx = unsafe { &*(handle as *const AppContext) };

    let args = GrantTimeArgs {
        duration: Duration::from_secs(duration_secs.max(0) as u64),
        pin: pin as u8,
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        ctx.runtime.block_on(ctx.grant_use_case.execute(args))
    }));

    match result {
        Ok(use_case_result) => jni_bridge::map_use_case_result(use_case_result),
        Err(_) => {
            log::error!("nativeGrantTime: panic during execute()");
            jni_bridge::ERR_INTERNAL_PANIC
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_margelo_nitro_kioskmodule_KioskNative_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle == 0 {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle as *mut AppContext));
    }
}
