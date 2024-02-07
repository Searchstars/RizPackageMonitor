use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config{
    pub token: String, // 机器人Token
    pub guild_id_list: Vec<u64>, // 机器人需要工作的服务器ID列表
    pub enable_dm_command: bool, // 是否启用私聊指令
    pub assetripper_server_url: String // AssetRipper服务器地址（注意，端口号需要在每次启动AssetRipper时更新）
}

#[derive(Serialize, Deserialize)]
pub struct GameConfig_Config {
    pub version: String,
    pub resourceUrl: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub configs: Vec<GameConfig_Config>,
    pub minimalVersion: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog_ProviderData {
    pub m_Id: String,
    pub m_ObjectType: Catalog_ObjectType,
    pub m_Data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog_ObjectType {
    pub m_AssemblyName: String,
    pub m_ClassName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog_ResourceType {
    pub m_AssemblyName: String,
    pub m_ClassName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub m_LocatorId: String,
    pub m_InstanceProviderData: Catalog_ProviderData,
    pub m_SceneProviderData: Catalog_ProviderData,
    pub m_ResourceProviderData: Vec<Catalog_ProviderData>,
    pub m_ProviderIds: Vec<String>,
    pub m_InternalIds: Vec<String>,
    pub m_KeyDataString: String,
    pub m_BucketDataString: String,
    pub m_EntryDataString: String,
    pub m_ExtraDataString: String,
    pub m_resourceTypes: Vec<Catalog_ResourceType>,
    pub m_InternalIdPrefixes: Vec<String>,
}