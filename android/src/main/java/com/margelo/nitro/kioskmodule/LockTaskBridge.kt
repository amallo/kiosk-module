package com.margelo.nitro.kioskmodule

import android.app.Activity
import android.content.Intent
import com.margelo.nitro.NitroModules

/**
 * Callback object passed to the Rust native layer at `nativeInit`. Rust holds a
 * JNI GlobalRef to an instance of this class and calls back into it to lock/unlock
 * the device via Android Lock Task Mode (`startLockTask`/`stopLockTask`), without
 * requiring Device Owner provisioning.
 *
 * The current activity is looked up via `NitroModules.applicationContext` at call
 * time (never stored), since this class is also instantiated with no live RN
 * context by the short-lived native cycle in [LockEnforcer] (alarm/boot receivers).
 *
 * Method signatures here are load-bearing: their JNI descriptors are hardcoded
 * as string literals in native/android/src/adapters/lock_task_device_locker.rs.
 * Do not change a signature here without updating that file's descriptors.
 *
 */
internal class LockTaskBridge {
    fun lockNow(): Boolean {
        bringAppToForeground()
        return withCurrentActivity {
            // Ne pas rappeler startLockTask() si l'activité est déjà épinglée :
            // EnforceTimeCreditUseCase peut retomber sur lock_now() à chaque
            // ré-évaluation (poll JS, alarme) même quand l'appareil est déjà
            // verrouillé, et ré-épingler à répétition perturbe le focus clavier.
            if (it.isInLockTaskModeSafe()) return@withCurrentActivity
            it.startLockTask()
        }
    }

    private fun Activity.isInLockTaskModeSafe(): Boolean {
        val activityManager = getSystemService(android.content.Context.ACTIVITY_SERVICE) as? android.app.ActivityManager
            ?: return false
        return activityManager.lockTaskModeState != android.app.ActivityManager.LOCK_TASK_MODE_NONE
    }

    fun unlockNow(): Boolean = withCurrentActivity {
        // Symétrique de lockNow() : GrantTimeUseCase/EnforceTimeCreditUseCase
        // rappellent unlock_now() à chaque ré-évaluation tant que le crédit est
        // valide (poll JS, alarme), même quand l'appareil est déjà dépinglé.
        if (!it.isInLockTaskModeSafe()) return@withCurrentActivity
        it.stopLockTask()
    }

    fun scheduleLock(atEpochSeconds: Long): Boolean {
        val context = NitroModules.applicationContext ?: return false
        AlarmScheduler.schedule(context, atEpochSeconds)
        return true
    }

    // Relais mécanique vers le callback JS : c'est EnforceTimeCreditUseCase/
    // GrantTimeUseCase (Rust) qui décident explicitement quand appeler ces
    // méthodes, jamais lockNow()/unlockNow() elles-mêmes.
    fun notifyLocked(): Boolean {
        LockStateEmitter.notify(true)
        return true
    }

    fun notifyUnlocked(): Boolean {
        LockStateEmitter.notify(false)
        return true
    }

    private fun bringAppToForeground() {
        val context = NitroModules.applicationContext ?: return
        // Rien à ramener au premier plan si une Activity est déjà là (cas
        // courant : l'app tourne déjà et vient d'être verrouillée). Relancer
        // l'intent à chaque appel provoquerait un restart/flicker inutile de
        // l'activité déjà au premier plan à chaque re-déclenchement de
        // EnforceTimeCreditUseCase (poll JS, alarme, boot).
        if (context.currentActivity != null) return
        val launchIntent = context.packageManager.getLaunchIntentForPackage(context.packageName) ?: return
        launchIntent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_REORDER_TO_FRONT)
        context.startActivity(launchIntent)
    }

    private fun withCurrentActivity(block: (Activity) -> Unit): Boolean {
        val activity = NitroModules.applicationContext?.currentActivity ?: return false
        return try {
            block(activity)
            true
        } catch (e: Exception) {
            false
        }
    }
}
