use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ZabbixResponse {
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: Option<String>,
    pub result: Vec<ZabbixHost>,
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZabbixHost {
    pub hostid: String,
    pub host: String,
    pub name: String,
    pub groups: Vec<ZabbixGroup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZabbixGroup {
    pub groupid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl ZabbixHost {
    pub fn get_group_by_id(&self, id: &str) -> Option<&ZabbixGroup> {
        self.groups.iter().find(|g| g.groupid == id)
    }
    
    pub fn get_groups_by_name_prefix(&self, prefix: &str) -> Vec<&ZabbixGroup> {
        self.groups
            .iter()
            .filter(|g| g.name.starts_with(prefix))
            .collect()
    }
}