use crate::services::comparator::ComparisonResult;
use colored::Colorize;

pub struct DiscrepancyReport;

impl DiscrepancyReport {
    pub fn generate(comparison: &ComparisonResult) -> String {
        let mut report = String::new();
        
        // Cabeçalho
        report.push_str(&Self::header());
        
        // Resumo
        report.push_str(&Self::summary_section(comparison));
        
        // Discrepâncias de Status
        if !comparison.status_mismatch.is_empty() {
            report.push_str(&Self::status_mismatch_section(comparison));
        }
        
        // Estações apenas no Zabbix
        if !comparison.only_in_zabbix.is_empty() {
            report.push_str(&Self::only_in_zabbix_section(comparison));
        }
        
        // Estações apenas no SharePoint
        if !comparison.only_in_sharepoint.is_empty() {
            report.push_str(&Self::only_in_sharepoint_section(comparison));
        }
        
        // Rodapé
        report.push_str(&Self::footer());
        
        report
    }
    
    fn header() -> String {
        let mut header = String::new();
        header.push_str(&"=".repeat(80));
        header.push('\n');
        header.push_str(&format!("{:^80}", "RELATÓRIO DE DISCREPÂNCIAS"));
        header.push('\n');
        header.push_str(&format!("{:^80}", "Zabbix vs SharePoint"));
        header.push('\n');
        header.push_str(&"=".repeat(80));
        header.push_str("\n\n");
        header
    }
    
    fn summary_section(comparison: &ComparisonResult) -> String {
        let mut summary = String::new();
        summary.push_str("📊 RESUMO\n");
        summary.push_str(&"-".repeat(40));
        summary.push('\n');
        
        summary.push_str(&format!(
            "  ✅ Estações correspondentes: {}\n",
            comparison.matching.len()
        ));
        summary.push_str(&format!(
            "  ⚠️  Discrepâncias de status: {}\n",
            comparison.status_mismatch.len()
        ));
        summary.push_str(&format!(
            "  📡 Apenas no Zabbix: {}\n",
            comparison.only_in_zabbix.len()
        ));
        summary.push_str(&format!(
            "  📝 Apenas no SharePoint: {}\n",
            comparison.only_in_sharepoint.len()
        ));
        summary.push_str("\n\n");
        summary
    }
    
    fn status_mismatch_section(comparison: &ComparisonResult) -> String {
        let mut section = String::new();
        section.push_str("⚠️  DISCREPÂNCIAS DE STATUS\n");
        section.push_str(&"-".repeat(40));
        section.push('\n');
        
        for mismatch in &comparison.status_mismatch {
            section.push_str(&format!(
                "\n  ID: {}\n",
                mismatch.id.yellow()
            ));
            section.push_str(&format!(
                "  Nome: {}\n",
                mismatch.nome
            ));
            section.push_str(&format!(
                "  Status Zabbix: {} | Status SharePoint: {}\n",
                format!("{}", mismatch.zabbix_status).red(),
                format!("{}", mismatch.sharepoint_status).blue()
            ));
            
            if mismatch.zabbix_regional != mismatch.sharepoint_regional {
                section.push_str(&format!(
                    "  Regional Zabbix: {} | Regional SharePoint: {}\n",
                    mismatch.zabbix_regional
                        .as_ref()
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    mismatch.sharepoint_regional
                        .as_ref()
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                ));
            }
        }
        section.push_str("\n\n");
        section
    }
    
    fn only_in_zabbix_section(comparison: &ComparisonResult) -> String {
        let mut section = String::new();
        section.push_str("📡 APENAS NO ZABBIX\n");
        section.push_str(&"-".repeat(40));
        section.push('\n');
        
        for estacao in &comparison.only_in_zabbix {
            section.push_str(&format!(
                "\n  • {} ({})\n",
                estacao.nome.cyan(),
                estacao.id
            ));
            section.push_str(&format!(
                "    Status: {} | Regional: {}\n",
                estacao.status,
                estacao.regional
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            ));
        }
        section.push_str("\n\n");
        section
    }
    
    fn only_in_sharepoint_section(comparison: &ComparisonResult) -> String {
        let mut section = String::new();
        section.push_str("📝 APENAS NO SHAREPOINT\n");
        section.push_str(&"-".repeat(40));
        section.push('\n');
        
        for estacao in &comparison.only_in_sharepoint {
            section.push_str(&format!(
                "\n  • {} ({})\n",
                estacao.nome.green(),
                estacao.id
            ));
            section.push_str(&format!(
                "    Status: {} | Regional: {}\n",
                estacao.status,
                estacao.regional
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            ));
        }
        section.push_str("\n\n");
        section
    }
    
    fn footer() -> String {
        let mut footer = String::new();
        footer.push_str(&"=".repeat(80));
        footer.push('\n');
        footer.push_str(&format!(
            "Gerado em: {}\n",
            chrono::Local::now().format("%d/%m/%Y %H:%M:%S")
        ));
        footer
    }
}