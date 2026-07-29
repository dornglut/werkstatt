/// Narrow storage boundary; concrete SQLite code lives in the adapter layer.
pub trait StorageDiagnostics {
    fn integrity_ok(&self) -> bool;
}
