use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SharePointRecord {
    #[serde(rename = "Local:UF")]
    pub uf: Option<String>,
    
    #[serde(rename = "Local:Município")]
    pub municipio: Option<String>,
    
    #[serde(rename = "Local")]
    pub local: Option<String>,
    
    #[serde(rename = "Detentor")]
    pub detentor: Option<String>,
    
    #[serde(rename = "Tipo de Estação")]
    pub tipo_estacao: Option<String>,
    
    #[serde(rename = "ID de rede")]
    pub id_rede: String,
    
    #[serde(rename = "Situação do Equipamento")]
    pub situacao: String,
    
    #[serde(rename = "Situação Litígio")]
    pub situacao_litigio: Option<String>,
    
    #[serde(rename = "Instrumento Fiscaliza")]
    pub instrumento_fiscaliza: Option<String>,
    
    #[serde(rename = "Link Zabbix")]
    pub link_zabbix: Option<String>,
    
    #[serde(rename = "IP OpenVPN")]
    pub ip_openvpn: Option<String>,
    
    #[serde(rename = "Patrimônio")]
    pub patrimonio: Option<String>,
    
    #[serde(rename = "Observações")]
    pub observacoes: Option<String>,
    
    #[serde(rename = "Pendência")]
    pub pendencia: Option<String>,
    
    #[serde(rename = "Ações a serem adotadas")]
    pub acoes: Option<String>,
    
    #[serde(rename = "Responsável na Anatel pela ação")]
    pub responsavel: Option<String>,
    
    #[serde(rename = "Modificado")]
    pub modificado: Option<String>,
    
    #[serde(rename = "Modificado por")]
    pub modificado_por: Option<String>,
}