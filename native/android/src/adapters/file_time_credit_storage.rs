use async_trait::async_trait;
use kiosk_core::adapters::time_credit_storage::{
    GrantedTimeCredit, StorageError, TimeCredit, TimeCreditStorage,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StoredCredit {
    start: u64,
    end: u64,
}

/// Persiste le crédit de temps accordé dans un fichier JSON. Choisi (plutôt qu'un
/// stockage en RAM) pour que le crédit survive à un reload/redémarrage de l'app.
/// Écriture atomique (fichier temporaire + rename) pour éviter un JSON tronqué
/// si le process est tué en plein milieu d'une écriture.
pub struct FileTimeCreditStorage {
    path: String,
}

impl FileTimeCreditStorage {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn tmp_path(&self) -> String {
        format!("{}.tmp", self.path)
    }
}

#[async_trait]
impl TimeCreditStorage for FileTimeCreditStorage {
    async fn grant(&self, credit: TimeCredit) -> Result<(), StorageError> {
        let data = StoredCredit {
            start: credit.start,
            end: credit.end,
        };
        let json = serde_json::to_vec(&data).map_err(|_| StorageError::WriteError)?;

        let tmp_path = self.tmp_path();
        tokio::fs::write(&tmp_path, json)
            .await
            .map_err(|_| StorageError::WriteError)?;
        tokio::fs::rename(&tmp_path, &self.path)
            .await
            .map_err(|_| StorageError::WriteError)?;
        Ok(())
    }

    async fn granted_until(&self) -> Result<GrantedTimeCredit, StorageError> {
        match tokio::fs::read(&self.path).await {
            Ok(bytes) => {
                let stored: StoredCredit =
                    serde_json::from_slice(&bytes).map_err(|_| StorageError::WriteError)?;
                Ok(GrantedTimeCredit::Until { time: stored.end })
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(GrantedTimeCredit::Denied),
            Err(_) => Err(StorageError::WriteError),
        }
    }
}
