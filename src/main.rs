use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use estacoes_manager::{
    parsers::{csv_parser::CsvParser, zabbix_parser::ZabbixParser},
    reports::{discrepancy::DiscrepancyReport, summary::SummaryReport},
    services::{
        aggregator::Aggregator, comparator::Comparator, data_loader::DataLoader,
    },
};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "estacoes-manager")]
#[command(about = "Gerenciador de Estações - Comparação Zabbix/SharePoint", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analisa dados do Zabbix
    Zabbix {
        /// Arquivo JSON do Zabbix
        #[arg(short, long)]
        file: PathBuf,
        
        /// Gera relatório resumido
        #[arg(short, long)]
        summary: bool,
    },
    
    /// Analisa dados do SharePoint
    SharePoint {
        /// Arquivo CSV do SharePoint
        #[arg(short, long)]
        file: PathBuf,
        
        /// Gera relatório resumido
        #[arg(short, long)]
        summary: bool,
    },
    
    /// Compara dados do Zabbix com SharePoint
    Compare {
        /// Arquivo JSON do Zabbix
        #[arg(short = 'z', long)]
        zabbix: PathBuf,
        
        /// Arquivo CSV do SharePoint
        #[arg(short = 's', long)]
        sharepoint: PathBuf,
        
        /// Arquivo de saída para o relatório
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    // Configurar logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Zabbix { file, summary } => {
            info!("Analisando arquivo Zabbix: {:?}", file);
            
            let data = DataLoader::load_file(&file)?;
            let hosts = ZabbixParser::parse(&data)?;
            let estacoes = ZabbixParser::to_estacao_info(hosts)?;
            
            println!("\n{}", "=== ANÁLISE ZABBIX ===".green().bold());
            println!("Total de estações: {}", estacoes.len());
            
            if summary {
                let aggregated = Aggregator::group_by_status_and_regional(&estacoes);
                let report = SummaryReport::generate(&aggregated);
                println!("{}", report);
            }
        }
        
        Commands::SharePoint { file, summary } => {
            info!("Analisando arquivo SharePoint: {:?}", file);
            
            let data = DataLoader::load_file(&file)?;
            let records = CsvParser::parse(&data)?;
            let estacoes = CsvParser::to_estacao_info(records)?;
            
            println!("\n{}", "=== ANÁLISE SHAREPOINT ===".blue().bold());
            println!("Total de estações: {}", estacoes.len());
            
            if summary {
                let aggregated = Aggregator::group_by_status_and_regional(&estacoes);
                let report = SummaryReport::generate(&aggregated);
                println!("{}", report);
            }
        }
        
        Commands::Compare { zabbix, sharepoint, output } => {
            info!("Comparando Zabbix com SharePoint");
            
            // Carregar dados do Zabbix
            let zabbix_data = DataLoader::load_file(&zabbix)?;
            let zabbix_hosts = ZabbixParser::parse(&zabbix_data)?;
            let zabbix_estacoes = ZabbixParser::to_estacao_info(zabbix_hosts)?;
            
            // Carregar dados do SharePoint
            let sp_data = DataLoader::load_file(&sharepoint)?;
            let sp_records = CsvParser::parse(&sp_data)?;
            let sp_estacoes = CsvParser::to_estacao_info(sp_records)?;
            
            // Comparar
            let comparison = Comparator::compare(zabbix_estacoes, sp_estacoes);
            let report = DiscrepancyReport::generate(&comparison);
            
            // Salvar ou imprimir relatório
            if let Some(output_path) = output {
                DataLoader::save_file(&output_path, &report)?;
                println!("{}", format!("Relatório salvo em: {:?}", output_path).green());
            } else {
                println!("{}", report);
            }
        }
    }

    Ok(())
}