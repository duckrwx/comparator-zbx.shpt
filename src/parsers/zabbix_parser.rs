use crate::{
    config::{REGIONAL_PREFIXES, STATUS_ATIVO, STATUS_DEFEITO, STATUS_DISPONIVEL, 
             STATUS_LITIGIO, STATUS_NOMADICO},
    error::Result,
    models::{
        common::{DataSource, EstacaoInfo, Regional, Status, TipoEstacao},
        zabbix::{ZabbixHost, ZabbixResponse},
    },
    utils::normalizer::Normalizer,
};

pub struct ZabbixParser;

impl ZabbixParser {
    pub fn parse(json_data: &str) -> Result<Vec<ZabbixHost>> {
        let response: ZabbixResponse = serde_json::from_str(json_data)?;
        Ok(response.result)
    }
    
    pub fn to_estacao_info(hosts: Vec<ZabbixHost>) -> Result<Vec<EstacaoInfo>> {
        let mut estacoes = Vec::new();
        
        for host in hosts {
            let status = Self::extract_status(&host).unwrap_or(crate::models::common::Status::Desconhecido);
            let regional = Self::extract_regional(&host);
            let tipo = Self::extract_tipo(&host);

            let estacao = EstacaoInfo::new(
                host.hostid.clone(),
                host.name.clone(),
                status,
                regional,
                tipo,
                DataSource::Zabbix,
            );

            estacoes.push(estacao);
        }
        
        Ok(estacoes)
    }
    
    fn extract_status(host: &ZabbixHost) -> Option<Status> {
        for group in &host.groups {
            // Primeiro, tentar por groupid (configurado)
            match group.groupid.as_str() {
                STATUS_ATIVO => return Some(Status::Ativo),
                STATUS_DEFEITO => return Some(Status::Defeito),
                STATUS_DISPONIVEL => return Some(Status::Disponivel),
                STATUS_NOMADICO => return Some(Status::Nomadico),
                STATUS_LITIGIO => return Some(Status::Litigio),
                _ => {}
            }

            // Se não encontrou por ID, tentar inferir pelo nome do grupo (mais flexível)
            let name_norm = Normalizer::normalize_text(&group.name).to_lowercase();
            match name_norm.as_str() {
                "ativo" => return Some(Status::Ativo),
                "defeito" => return Some(Status::Defeito),
                "disponivel" | "disponível" => return Some(Status::Disponivel),
                "nomadico" | "nomádico" => return Some(Status::Nomadico),
                "litigio" | "litígio" => return Some(Status::Litigio),
                "manutencao" | "manutenção" => return Some(Status::Manutencao),
                "triagem" => return Some(Status::Triagem),
                "baixa" => return Some(Status::Baixa),
                _ => continue,
            }
        }
        None
    }
    
    fn extract_regional(host: &ZabbixHost) -> Option<Regional> {
        for group in &host.groups {
            for prefix in REGIONAL_PREFIXES {
                if group.name.starts_with(prefix) {
                    return Some(Regional::new(group.name.clone()));
                }
            }
        }
        None
    }
    
    fn extract_tipo(host: &ZabbixHost) -> TipoEstacao {
        let nome = &host.name;
        
        if nome.to_lowercase().contains("rfeye") {
            TipoEstacao::RFeye
        } else if nome.to_lowercase().contains("miaer") {
            TipoEstacao::MIAer
        } else if nome.to_lowercase().contains("cwsm") {
            TipoEstacao::CelWirelessRMU
        } else if nome.to_lowercase().contains("ums") {
            TipoEstacao::UMS300
        } else if nome.to_lowercase().contains("erm") {
            TipoEstacao::ERMx
        } else {
            // Tentar identificar pelo grupo
            for group in &host.groups {
                let group_name = group.name.to_lowercase();
                if group_name.contains("rfeye") {
                    return TipoEstacao::RFeye;
                } else if group_name.contains("miaer") {
                    return TipoEstacao::MIAer;
                } else if group_name.contains("erm") {
                    return TipoEstacao::ERMx;
                }
            }
            TipoEstacao::Outro(nome.clone())
        }
    }
}