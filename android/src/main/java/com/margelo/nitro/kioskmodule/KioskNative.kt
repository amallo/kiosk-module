package com.margelo.nitro.kioskmodule

/**
 * Pont JNI vers le crate Rust `kiosk-android` (native/android), lui-même
 * dépendant de la logique métier pure du crate `core` (native/core).
 *
 * Codes de retour (voir native/android/src/jni_bridge.rs) :
 *   0   RESULT_OK
 *   1   RESULT_GRANTED
 *   2   RESULT_DENIED
 *  -1   ERR_TIME_CREDIT_STORAGE
 *  -2   ERR_PIN_VALIDATION
 *  -3   ERR_LOCK_DEVICE
 * -100  ERR_INTERNAL_PANIC
 */
internal object KioskNative {
    init {
        System.loadLibrary("kiosk_android")
    }

    external fun nativeInit(storageBridge: TimeCreditStorageBridge, lockBridge: LockTaskBridge): Long
    external fun nativeLockDevice(handle: Long): Int
    external fun nativeGrantTime(handle: Long, durationSecs: Long, pin: Int): Int
    external fun nativeDestroy(handle: Long)
}
