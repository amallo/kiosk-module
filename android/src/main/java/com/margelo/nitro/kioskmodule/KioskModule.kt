package com.margelo.nitro.kioskmodule
  
import com.facebook.proguard.annotations.DoNotStrip

@DoNotStrip
class KioskModule : HybridKioskModuleSpec() {
  override fun multiply(a: Double, b: Double): Double {
    return a * b
  }
}
