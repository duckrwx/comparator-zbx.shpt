use crate::{
    error::{AppError, Result},
    models::{
        common::{DataSource, EstacaoInfo, Regional, Status, TipoEstacao},
        sharepoint::SharePointRecord,
    },
    
};

pub struct CsvParser;

impl CsvParser {
    pub fn parse(csv_data: &str) -> Result<Vec<SharePointRecord>> {
        let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
        let mut records = Vec::new();
        
        for result in reader.deserialize() {
            let record: SharePointRecord = result?;
            records.push(record);
        }
        
        Ok(records)
    }
    
    pub fn to_estacao_info(records: Vec<SharePointRecord>) -> Result<Vec<EstacaoInfo>> {
        let mut estacoes = Vec::new();
        
        for record in records {
            let status = Status::from_str(&record.situacao)
                .ok_or_else(|| AppError::InvalidStatus(record.situacao.clone()))?;
            
            let regional = Self::extract_regional(&record);
            let tipo = Self::extract_tipo(&record);
            
            let estacao = EstacaoInfo::new(
                record.id_rede.clone(),
                record.id_rede.clone(), // Usando ID como nome também
                status,
                regional,
                tipo,
                DataSource::SharePoint,
            );
            
            estacoes.push(estacao);
        }
        
        Ok(estacoes)
    }
    
    fn extract_regional(record: &SharePointRecord) -> Option<Regional> {
        if let Some(detentor) = &record.detentor {
            // Procurar por padrões GR ou UO no campo detentor
            let parts: Vec<&str> = detentor.split(&[' ', ','][..]).collect();
            for part in parts {
                if part.starts_with("GR") || part.starts_with("UO") {
                    return Some(Regional::new(part.to_string()));
                }
            }
        }
        None
    }
    
    fn extract_tipo(record: &SharePointRecord) -> TipoEstacao {
        if let Some(tipo) = &record.tipo_estacao {
            TipoEstacao::from_str(tipo)
        } else {
            // Tentar inferir pelo ID
            let id = record.id_rede.to_lowercase();
            if id.contains("rfeye") {
                TipoEstacao::RFeye
            } else if id.contains("miaer") {
                TipoEstacao::MIAer
            } else if id.contains("cwsm") {
                TipoEstacao::CelWirelessRMU
            } else if id.contains("ums") {
                TipoEstacao::UMS300
            } else if id.contains("erm") {
                TipoEstacao::ERMx
            } else {
                TipoEstacao::Outro(record.id_rede.clone())
            }
        }
    }
}