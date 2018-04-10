use toml::Value;

#[derive(Serialize, Debug)]
pub struct Repository {
    pub url: String,
    pub version_control: String, 
}

//Add URL Verifcation
impl Repository {
    pub fn parse_repo_info(info: &Value) -> Repository {
        Repository {
            url: info["url"].to_string(),
            version_control: info["version_control"].to_string()
        }
    }
}