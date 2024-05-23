
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AurPackage {
    pub(crate) Name: String,
    pub(crate) Version: String,
    pub(crate) Description: Option<String>,
    pub(crate) URLPath: String,
}

#[derive(Deserialize, Debug)]
pub struct AurResponse {
    pub(crate) results: Vec<AurPackage>,
}

const AUR_BASE_URL: &str = "https://aur.archlinux.org/rpc/?v=5&type=search&arg=";

pub fn search_aur(query: &str) -> Result<AurResponse, Box<dyn std::error::Error>> {
    let url = format!("{}{}", AUR_BASE_URL, query);
    let response: AurResponse = get(&url)?.json()?;
    Ok(response)
}