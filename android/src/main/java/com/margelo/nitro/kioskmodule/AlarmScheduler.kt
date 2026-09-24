package com.margelo.nitro.kioskmodule

import android.app.AlarmManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent

/**
 * Wraps a single one-shot inexact alarm (`setAndAllowWhileIdle`) used to wake
 * [LockAlarmReceiver] when a granted time credit expires. Always uses the same
 * `PendingIntent` request code with `FLAG_UPDATE_CURRENT`, so scheduling a new
 * alarm before the previous one fires transparently replaces it — only one
 * lock alarm is ever pending at a time.
 */
internal object AlarmScheduler {
    private const val REQUEST_CODE = 1001

    fun schedule(context: Context, atEpochSeconds: Long) {
        val alarmManager = context.getSystemService(Context.ALARM_SERVICE) as AlarmManager
        alarmManager.setAndAllowWhileIdle(
            AlarmManager.RTC_WAKEUP,
            atEpochSeconds * 1000,
            pendingIntent(context),
        )
    }

    private fun pendingIntent(context: Context): PendingIntent =
        PendingIntent.getBroadcast(
            context,
            REQUEST_CODE,
            Intent(context, LockAlarmReceiver::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
        )
}
