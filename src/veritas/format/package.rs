use toml::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String, // <- Should this be a Version?
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl Package {
    pub fn parse_package_info(info: &Value) -> Package {
        Package {
            name: info["name"].to_string(),
            //TODO: SEMANTIC VERSION VERIFIER
            version: info["version"].to_string(),
            description: info["description"].to_string(),
            author: Some(info["author"].to_string()),
            license: Some(info["license"].to_string())
        }
    }
}