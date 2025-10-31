use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    Ativo,
    Defeito,
    Disponivel,
    Triagem,
    Desconhecido,
    Manutencao,
    Nomadico,
    Litigio,
    Baixa,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Ativo => write!(f, "Ativo"),
            Status::Defeito => write!(f, "Defeito"),
            Status::Disponivel => write!(f, "Disponível"),
            Status::Triagem => write!(f, "Triagem"),
            Status::Desconhecido => write!(f, "Desconhecido"),
            Status::Manutencao => write!(f, "Manutenção"),
            Status::Nomadico => write!(f, "Nomádico"),
            Status::Litigio => write!(f, "Litígio"),
            Status::Baixa => write!(f, "Baixa"),
        }
    }
}

impl Status {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ativo" => Some(Status::Ativo),
            "defeito" => Some(Status::Defeito),
            "disponível" | "disponivel" => Some(Status::Disponivel),
            "triagem" => Some(Status::Triagem),
            "manutenção" | "manutencao" => Some(Status::Manutencao),
            "nomádico" | "nomadico" => Some(Status::Nomadico),
            "litígio" | "litigio" => Some(Status::Litigio),
            "baixa" => Some(Status::Baixa),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Regional(pub String);

impl fmt::Display for Regional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Regional {
    pub fn new(s: String) -> Self {
        Regional(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstacaoInfo {
    pub id: String,
    pub nome: String,
    pub status: Status,
    pub regional: Option<Regional>,
    pub tipo: TipoEstacao,
    pub source: DataSource,
}

impl EstacaoInfo {
    pub fn new(
        id: String,
        nome: String,
        status: Status,
        regional: Option<Regional>,
        tipo: TipoEstacao,
        source: DataSource,
    ) -> Self {
        Self {
            id,
            nome,
            status,
            regional,
            tipo,
            source,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipoEstacao {
    RFeye,
    MIAer,
    CelWirelessRMU,
    UMS300,
    ERMx,
    Outro(String),
}

impl fmt::Display for TipoEstacao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoEstacao::RFeye => write!(f, "RFeye"),
            TipoEstacao::MIAer => write!(f, "MIAer"),
            TipoEstacao::CelWirelessRMU => write!(f, "CelWireless RMU"),
            TipoEstacao::UMS300 => write!(f, "UMS300"),
            TipoEstacao::ERMx => write!(f, "ERM-x"),
            TipoEstacao::Outro(s) => write!(f, "{}", s),
        }
    }
}

impl TipoEstacao {
    pub fn from_str(s: &str) -> Self {
        match s {
            "RFeye" => TipoEstacao::RFeye,
            "MIAer" => TipoEstacao::MIAer,
            "CelWireless RMU" => TipoEstacao::CelWirelessRMU,
            "UMS300" => TipoEstacao::UMS300,
            "ERM-x" | "ERMx" => TipoEstacao::ERMx,
            other => TipoEstacao::Outro(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Zabbix,
    SharePoint,
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSource::Zabbix => write!(f, "Zabbix"),
            DataSource::SharePoint => write!(f, "SharePoint"),
        }
    }
}