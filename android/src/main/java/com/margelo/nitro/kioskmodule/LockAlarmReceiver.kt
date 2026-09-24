package com.margelo.nitro.kioskmodule

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

/** Fired by [AlarmScheduler] when a granted time credit is about to expire. */
class LockAlarmReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        LockEnforcer.run(context.applicationContext)
    }
}
