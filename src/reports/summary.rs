use crate::services::aggregator::AggregatedData;
use colored::Colorize;

pub struct SummaryReport;

impl SummaryReport {
    pub fn generate(data: &AggregatedData) -> String {
        let mut report = String::new();
        
        // Cabeçalho
        report.push_str(&Self::header());
        
        // Estatísticas gerais
        report.push_str(&Self::general_stats(data));
        
        // Por Status
        report.push_str(&Self::by_status(data));
        
        // Por Regional
        report.push_str(&Self::by_regional(data));
        
        // Matriz Status x Regional
        report.push_str(&Self::status_regional_matrix(data));
        
        report
    }
    
    fn header() -> String {
        let mut header = String::new();
        header.push_str(&"═".repeat(80));
        header.push('\n');
        header.push_str(&format!("{:^80}", "RELATÓRIO RESUMIDO"));
        header.push('\n');
        header.push_str(&"═".repeat(80));
        header.push_str("\n\n");
        header
    }
    
    fn general_stats(data: &AggregatedData) -> String {
        let mut stats = String::new();
        stats.push_str("📊 ESTATÍSTICAS GERAIS\n");
        stats.push_str(&"─".repeat(40));
        stats.push('\n');
        stats.push_str(&format!(
            "  Total de estações: {}\n",
            data.totals.total.to_string().bold()
        ));
        if data.totals.without_regional > 0 {
            stats.push_str(&format!(
                "  Sem regional definida: {}\n",
                data.totals.without_regional.to_string().yellow()
            ));
        }
        stats.push_str("\n\n");
        stats
    }
    
    fn by_status(data: &AggregatedData) -> String {
        let mut section = String::new();
        section.push_str("📈 DISTRIBUIÇÃO POR STATUS\n");
        section.push_str(&"─".repeat(40));
        section.push('\n');
        
        let mut status_sorted: Vec<_> = data.totals.by_status.iter().collect();
        status_sorted.sort_by_key(|&(k, _)| format!("{:?}", k));
        
        for (status, count) in status_sorted {
            let percentage = (*count as f64 / data.totals.total as f64) * 100.0;
            let bar_length = (percentage / 2.0) as usize;
            let bar = "█".repeat(bar_length);
            
            let status_str = format!("{}", status);
            let colored_status = match status {
                crate::models::common::Status::Ativo => status_str.green(),
                crate::models::common::Status::Defeito => status_str.red(),
                crate::models::common::Status::Disponivel => status_str.cyan(),
                crate::models::common::Status::Nomadico => status_str.yellow(),
                crate::models::common::Status::Litigio => status_str.magenta(),
                crate::models::common::Status::Baixa => status_str.white(),
            };
            
            section.push_str(&format!(
                "  {:<12} {:>4} ({:>5.1}%) {}\n",
                colored_status,
                count,
                percentage,
                bar.blue()
            ));
        }
        section.push_str("\n\n");
        section
    }
    
    fn by_regional(data: &AggregatedData) -> String {
        let mut section = String::new();
        section.push_str("🌍 DISTRIBUIÇÃO POR REGIONAL\n");
        section.push_str(&"─".repeat(40));
        section.push('\n');
        
        let mut regional_sorted: Vec<_> = data.totals.by_regional.iter().collect();
        regional_sorted.sort_by_key(|&(k, _)| k);
        
        for (regional, count) in regional_sorted {
            let percentage = (*count as f64 / data.totals.total as f64) * 100.0;
            section.push_str(&format!(
                "  {:<12} {:>4} ({:>5.1}%)\n",
                regional.cyan(),
                count,
                percentage
            ));
        }
        section.push_str("\n\n");
        section
    }
    
    fn status_regional_matrix(data: &AggregatedData) -> String {
        let mut section = String::new();
        section.push_str("🔀 MATRIZ STATUS × REGIONAL\n");
        section.push_str(&"─".repeat(40));
        section.push('\n');
        
        // Coletar todas as regionais únicas
        let mut all_regionals: Vec<String> = data.by_status_and_regional
            .keys()
            .map(|(_, r)| r.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        all_regionals.sort();
        
        // Coletar todos os status únicos
        let mut all_statuses: Vec<_> = data.by_status_and_regional
            .keys()
            .map(|(s, _)| s.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        all_statuses.sort_by_key(|s| format!("{:?}", s));
        
        // Criar matriz
        section.push_str("\n");
        section.push_str(&format!("{:<15}", ""));
        for regional in &all_regionals {
            section.push_str(&format!("{:<10}", &regional[..regional.len().min(9)]));
        }
        section.push('\n');
        
        for status in &all_statuses {
            section.push_str(&format!("{:<15}", format!("{}", status)));
            for regional in &all_regionals {
                let count = data.by_status_and_regional
                    .get(&(status.clone(), regional.clone()))
                    .map(|v| v.len())
                    .unwrap_or(0);
                if count > 0 {
                    section.push_str(&format!("{:<10}", count));
                } else {
                    section.push_str(&format!("{:<10}", "-"));
                }
            }
            section.push('\n');
        }
        
        section.push_str("\n\n");
        section
    }
}