class KioskModule: HybridKioskModuleSpec {
    public func grantTime(durationSecs: Double, pin: Double) throws -> Double {
        throw NSError(domain: "KioskModule", code: -1, userInfo: [NSLocalizedDescriptionKey: "grantTime not implemented on iOS yet"])
    }
}
