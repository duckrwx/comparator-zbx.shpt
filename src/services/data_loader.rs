use crate::error::Result;
use std::fs;
use std::path::Path;

pub struct DataLoader;

impl DataLoader {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    
    pub fn save_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
        fs::write(path, content)?;
        Ok(())
    }
    
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
    
    pub fn create_backup<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        if path.exists() {
            let backup_name = format!(
                "{}.backup.{}",
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );
            let backup_path = path.parent()
                .unwrap()
                .join(backup_name);
            fs::copy(path, &backup_path)?;
            Ok(backup_path.to_str().unwrap().to_string())
        } else {
            Err(crate::error::AppError::FileRead(
                std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")
            ))
        }
    }
}