package com.margelo.nitro.kioskmodule

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

/**
 * Rearms lock enforcement after a reboot: `AlarmManager` alarms don't survive a
 * reboot, but the granted-credit `SharedPreferences` do, so we re-run the lock
 * use case once to either re-lock immediately (credit already expired) or
 * reschedule the future alarm (credit still valid).
 */
class BootCompletedReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action == Intent.ACTION_BOOT_COMPLETED) {
            LockEnforcer.run(context.applicationContext)
        }
    }
}
