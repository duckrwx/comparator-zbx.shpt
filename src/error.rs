use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro ao ler arquivo: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("Erro ao parsear JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("Erro ao parsear CSV: {0}")]
    CsvParse(#[from] csv::Error),
    
    #[error("Status inválido: {0}")]
    InvalidStatus(String),
    
    #[error("Regional não encontrada para host: {0}")]
    RegionalNotFound(String),
    
    #[error("Discrepância encontrada: {0}")]
    Discrepancy(String),
    
    #[error("Erro genérico: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Generic(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Generic(s.to_string())
    }
}