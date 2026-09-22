package com.margelo.nitro.kioskmodule

import android.content.Context

/**
 * Callback object passed to the Rust native layer at `nativeInit`. Rust holds a
 * JNI GlobalRef to an instance of this class and calls back into it to
 * persist/read the granted time-credit window, backed by SharedPreferences.
 *
 * Method signatures here are load-bearing: their JNI descriptors are hardcoded
 * as string literals in native/android/src/adapters/shared_preferences_time_credit_storage.rs.
 * Do not change a signature here without updating that file's descriptors.
 */
internal class TimeCreditStorageBridge(context: Context) {
    private val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun grant(start: Long, end: Long): Boolean =
        prefs.edit()
            .putLong(KEY_START, start)
            .putLong(KEY_END, end)
            .commit()

    fun grantedUntil(): Long = prefs.getLong(KEY_END, NO_CREDIT)

    companion object {
        private const val PREFS_NAME = "kiosk_time_credit"
        private const val KEY_START = "start"
        private const val KEY_END = "end"
        const val NO_CREDIT = -1L
    }
}
