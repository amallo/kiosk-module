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
          val storagePath = context.filesDir.absolutePath + "/kiosk_time_credit.json"
          handle = KioskNative.nativeInit(storagePath)
          nativeHandle = handle
        }
      }
    }
    return handle
  }

  // TODO: exposer lockDevice dans src/KioskModule.nitro.ts puis régénérer
  // `yarn nitrogen` pour la rendre appelable depuis JS. Pour l'instant, c'est
  // une simple méthode Kotlin utilisable pour valider le pipeline JNI natif.
  fun lockDevice(): Int = KioskNative.nativeLockDevice(ensureNativeHandle())

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
