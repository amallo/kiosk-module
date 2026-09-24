package com.margelo.nitro.kioskmodule

import android.content.Context

/**
 * Short-lived native cycle used by [LockAlarmReceiver] and [BootCompletedReceiver].
 * A receiver invoked in a cold-restarted process cannot reuse KioskModule's
 * long-lived native handle, so it builds its own handle, runs the lock use case
 * once, and destroys it immediately. `EnforceTimeCreditUseCase` decides
 * everything (grant + reschedule, or lock immediately) — this is a plain,
 * unconditional passthrough with no branching of its own.
 */
internal object LockEnforcer {
    fun run(context: Context) {
        val handle = KioskNative.nativeInit(TimeCreditStorageBridge(context), LockTaskBridge())
        if (handle == 0L) return
        try {
            KioskNative.nativeLockDevice(handle)
        } finally {
            KioskNative.nativeDestroy(handle)
        }
    }
}
