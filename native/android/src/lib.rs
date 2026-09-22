mod adapters;
mod app_context;
mod jni_bridge;

use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;
use std::time::Duration;

use app_context::AppContext;
use kiosk_core::usecases::grant_time_use_case::GrantTimeArgs;
use jni::objects::{JClass, JString};
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
    mut env: JNIEnv,
    _class: JClass,
    storage_path: JString,
) -> jlong {
    init_logger_once();

    let path: String = match env.get_string(&storage_path) {
        Ok(s) => s.into(),
        Err(_) => {
            log::error!("nativeInit: invalid storage_path argument");
            return 0;
        }
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| AppContext::new(path)));
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
