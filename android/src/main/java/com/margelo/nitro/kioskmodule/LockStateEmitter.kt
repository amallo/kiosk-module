package com.margelo.nitro.kioskmodule

/**
 * Relais process-wide vers le callback JS `onLockStateChanged` posé sur le
 * `KioskModule` HybridObject. Nécessaire car [LockEnforcer] construit sa propre
 * instance de [LockTaskBridge] pour les receivers (alarme/boot), distincte de
 * celle détenue par [KioskModule] sur laquelle JS enregistre son callback :
 * sans ce relais statique, l'instance du receiver n'aurait aucun moyen
 * d'atteindre la fonction JS enregistrée par l'autre instance.
 *
 * Purement mécanique : ne décide jamais quand notifier, se contente de
 * relayer ce que [LockTaskBridge] lui transmet.
 */
internal object LockStateEmitter {
    @Volatile
    var listener: ((Boolean) -> Unit)? = null

    fun notify(locked: Boolean) {
        listener?.invoke(locked)
    }
}
