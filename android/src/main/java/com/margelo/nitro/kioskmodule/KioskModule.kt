package com.margelo.nitro.kioskmodule

import com.facebook.proguard.annotations.DoNotStrip
import com.margelo.nitro.NitroModules

@DoNotStrip
class KioskModule : HybridKioskModuleSpec() {
  @Volatile
  private var nativeHandle: Long = 0

  private fun ensureNativeHandle(): Long {
    var handle = nativeHandle
    if (handle == 0L) {
      synchronized(this) {
        handle = nativeHandle
        if (handle == 0L) {
          val context = requireNotNull(NitroModules.applicationContext) {
            "NitroModules.applicationContext is not available yet"
          }
          val storageBridge = TimeCreditStorageBridge(context)
          val lockBridge = LockTaskBridge()
          handle = KioskNative.nativeInit(storageBridge, lockBridge)
          nativeHandle = handle
        }
      }
    }
    return handle
  }

  override var onLockStateChanged: (Boolean) -> Unit
    get() = LockStateEmitter.listener ?: {}
    set(value) { LockStateEmitter.listener = value }

  override fun enforceTimeCredit(): Double =
    KioskNative.nativeLockDevice(ensureNativeHandle()).toDouble()

  override fun grantTime(durationSecs: Double, pin: Double): Double =
    KioskNative.nativeGrantTime(ensureNativeHandle(), durationSecs.toLong(), pin.toInt()).toDouble()

  override fun dispose() {
    synchronized(this) {
      if (nativeHandle != 0L) {
        KioskNative.nativeDestroy(nativeHandle)
        nativeHandle = 0
      }
    }
    super.dispose()
  }
}
