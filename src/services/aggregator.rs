use crate::models::common::{EstacaoInfo, Status};
use indexmap::IndexMap;
use std::collections::HashMap;

pub struct Aggregator;

#[derive(Debug, Clone)]
pub struct AggregatedData {
    pub by_status: HashMap<Status, Vec<EstacaoInfo>>,
    pub by_regional: HashMap<String, Vec<EstacaoInfo>>,
    pub by_status_and_regional: IndexMap<(Status, String), Vec<EstacaoInfo>>,
    pub totals: AggregationTotals,
}

#[derive(Debug, Clone)]
pub struct AggregationTotals {
    pub total: usize,
    pub by_status: HashMap<Status, usize>,
    pub by_regional: HashMap<String, usize>,
    pub without_regional: usize,
}

impl Aggregator {
    pub fn group_by_status_and_regional(estacoes: &[EstacaoInfo]) -> AggregatedData {
        let mut by_status: HashMap<Status, Vec<EstacaoInfo>> = HashMap::new();
        let mut by_regional: HashMap<String, Vec<EstacaoInfo>> = HashMap::new();
        let mut by_status_and_regional: IndexMap<(Status, String), Vec<EstacaoInfo>> = 
            IndexMap::new();
        
        let mut status_counts: HashMap<Status, usize> = HashMap::new();
        let mut regional_counts: HashMap<String, usize> = HashMap::new();
        let mut without_regional = 0;
        
        for estacao in estacoes {
            // Agrupar por status
            by_status
                .entry(estacao.status.clone())
                .or_insert_with(Vec::new)
                .push(estacao.clone());
            
            *status_counts.entry(estacao.status.clone()).or_insert(0) += 1;
            
            // Agrupar por regional
            if let Some(regional) = &estacao.regional {
                by_regional
                    .entry(regional.0.clone())
                    .or_insert_with(Vec::new)
                    .push(estacao.clone());
                
                *regional_counts.entry(regional.0.clone()).or_insert(0) += 1;
                
                // Agrupar por status e regional
                by_status_and_regional
                    .entry((estacao.status.clone(), regional.0.clone()))
                    .or_insert_with(Vec::new)
                    .push(estacao.clone());
            } else {
                without_regional += 1;
                
                // Adicionar ao grupo "SEM_REGIONAL"
                by_regional
                    .entry("SEM_REGIONAL".to_string())
                    .or_insert_with(Vec::new)
                    .push(estacao.clone());
                
                by_status_and_regional
                    .entry((estacao.status.clone(), "SEM_REGIONAL".to_string()))
                    .or_insert_with(Vec::new)
                    .push(estacao.clone());
            }
        }
        
        // Ordenar por status e regional
        by_status_and_regional.sort_keys();
        
        AggregatedData {
            by_status,
            by_regional,
            by_status_and_regional,
            totals: AggregationTotals {
                total: estacoes.len(),
                by_status: status_counts,
                by_regional: regional_counts,
                without_regional,
            },
        }
    }
    
    pub fn summarize(data: &AggregatedData) -> String {
        let mut summary = String::new();
        
        summary.push_str(&format!("Total de estações: {}\n", data.totals.total));
        summary.push_str("\nPor Status:\n");
        
        for (status, count) in &data.totals.by_status {
            let percentage = (*count as f64 / data.totals.total as f64) * 100.0;
            summary.push_str(&format!(
                "  {}: {} ({:.1}%)\n",
                status, count, percentage
            ));
        }
        
        summary.push_str("\nPor Regional:\n");
        let mut regional_sorted: Vec<_> = data.totals.by_regional.iter().collect();
        regional_sorted.sort_by_key(|&(k, _)| k);
        
        for (regional, count) in regional_sorted {
            let percentage = (*count as f64 / data.totals.total as f64) * 100.0;
            summary.push_str(&format!(
                "  {}: {} ({:.1}%)\n",
                regional, count, percentage
            ));
        }
        
        if data.totals.without_regional > 0 {
            summary.push_str(&format!(
                "\nEstações sem regional definida: {}\n",
                data.totals.without_regional
            ));
        }
        
        summary
    }
}