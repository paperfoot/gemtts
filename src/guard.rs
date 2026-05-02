use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use crate::config;
use crate::error::AppError;

pub struct GenerationGuard {
    path: PathBuf,
    active: bool,
}

impl GenerationGuard {
    pub fn acquire(force: bool) -> Result<Self, AppError> {
        let dir = config::state_dir();
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("generation.lock");

        if path.exists() && !force {
            let stale = is_stale(&path, Duration::from_secs(20 * 60));
            if !stale {
                return Err(AppError::Transient(format!(
                    "a recent generation lock exists at {}",
                    path.display()
                )));
            }
            let _ = std::fs::remove_file(&path);
        }

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .or_else(|e| {
                if force {
                    let _ = std::fs::remove_file(&path);
                    OpenOptions::new().create_new(true).write(true).open(&path)
                } else {
                    Err(e)
                }
            })?;

        writeln!(
            file,
            "pid={}\ncreated_at={:?}",
            std::process::id(),
            SystemTime::now()
        )?;

        Ok(Self { path, active: true })
    }
}

impl Drop for GenerationGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn is_stale(path: &PathBuf, max_age: Duration) -> bool {
    let Ok(metadata) = File::open(path).and_then(|f| f.metadata()) else {
        return true;
    };
    let Ok(modified) = metadata.modified() else {
        return true;
    };
    modified.elapsed().unwrap_or(max_age) >= max_age
}
