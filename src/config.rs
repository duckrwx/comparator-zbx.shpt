// Constantes para os GroupIDs do Zabbix
pub const STATUS_ATIVO: &str = "54";
pub const STATUS_DEFEITO: &str = "48";
pub const STATUS_DISPONIVEL: &str = "122";
pub const STATUS_NOMADICO: &str = "58";
pub const STATUS_LITIGIO: &str = "120";

// Prefixos para identificar regionais
pub const REGIONAL_PREFIXES: &[&str] = &["GR", "UO"];

// Mapeamento de status do SharePoint para o enum Status
pub const SHAREPOINT_STATUS_MAP: &[(&str, &str)] = &[
    ("Ativo", "Ativo"),
    ("Defeito", "Defeito"),
    ("Disponível", "Disponivel"),
    ("Nomádico", "Nomadico"),
    ("Baixa", "Baixa"),
];

// Tipos de estação conhecidos
pub const STATION_TYPES: &[&str] = &[
    "RFeye",
    "MIAer",
    "CelWireless RMU",
    "UMS300",
    "ERM-x",
    "ERMx",
];