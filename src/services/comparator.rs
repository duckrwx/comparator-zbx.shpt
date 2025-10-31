use crate::models::common::{EstacaoInfo, Status};
use crate::utils::normalizer::Normalizer;
use std::collections::{HashMap, HashSet};

pub struct Comparator;

impl Comparator {
    pub fn compare(
        zabbix_data: Vec<EstacaoInfo>,
        sharepoint_data: Vec<EstacaoInfo>,
    ) -> ComparisonResult {
        let mut zabbix_map: HashMap<String, EstacaoInfo> = HashMap::new();
        let mut sharepoint_map: HashMap<String, EstacaoInfo> = HashMap::new();
        
        // Normalizar e mapear dados do Zabbix
        for estacao in zabbix_data {
            let normalized_id = Normalizer::normalize_id(&estacao.nome);
            zabbix_map.insert(normalized_id, estacao);
        }
        
        // Normalizar e mapear dados do SharePoint
        for estacao in sharepoint_data {
            let normalized_id = Normalizer::normalize_id(&estacao.id);
            sharepoint_map.insert(normalized_id, estacao);
        }
        
        let zabbix_keys: HashSet<_> = zabbix_map.keys().cloned().collect();
        let sharepoint_keys: HashSet<_> = sharepoint_map.keys().cloned().collect();
        
        // Encontrar correspondências
        let common_keys: HashSet<_> = zabbix_keys
            .intersection(&sharepoint_keys)
            .cloned()
            .collect();
        
        let mut matching = Vec::new();
        let mut status_mismatch = Vec::new();
        
        for key in &common_keys {
            let zabbix_estacao = &zabbix_map[key];
            let sp_estacao = &sharepoint_map[key];
            
            if zabbix_estacao.status == sp_estacao.status {
                matching.push((zabbix_estacao.clone(), sp_estacao.clone()));
            } else {
                status_mismatch.push(StatusMismatch {
                    id: key.clone(),
                    nome: zabbix_estacao.nome.clone(),
                    zabbix_status: zabbix_estacao.status.clone(),
                    sharepoint_status: sp_estacao.status.clone(),
                    zabbix_regional: zabbix_estacao.regional.clone(),
                    sharepoint_regional: sp_estacao.regional.clone(),
                });
            }
        }
        
        // Encontrar estações únicas
        let only_zabbix_keys: HashSet<_> = zabbix_keys
            .difference(&sharepoint_keys)
            .cloned()
            .collect();
        let only_sharepoint_keys: HashSet<_> = sharepoint_keys
            .difference(&zabbix_keys)
            .cloned()
            .collect();
        
        let only_in_zabbix: Vec<_> = only_zabbix_keys
            .iter()
            .map(|k| zabbix_map[k].clone())
            .collect();
        
        let only_in_sharepoint: Vec<_> = only_sharepoint_keys
            .iter()
            .map(|k| sharepoint_map[k].clone())
            .collect();
        
        ComparisonResult {
            matching,
            only_in_zabbix,
            only_in_sharepoint,
            status_mismatch,
        }
    }
}

pub struct ComparisonResult {
    pub matching: Vec<(EstacaoInfo, EstacaoInfo)>,
    pub only_in_zabbix: Vec<EstacaoInfo>,
    pub only_in_sharepoint: Vec<EstacaoInfo>,
    pub status_mismatch: Vec<StatusMismatch>,
}

pub struct StatusMismatch {
    pub id: String,
    pub nome: String,
    pub zabbix_status: Status,
    pub sharepoint_status: Status,
    pub zabbix_regional: Option<crate::models::common::Regional>,
    pub sharepoint_regional: Option<crate::models::common::Regional>,
}