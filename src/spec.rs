use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub certs: Vec<Certificate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    pub subject: Subject,
    pub validity: Validity,
    pub key: Key,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subject {
    pub country: String,
    pub common: String,
    pub org: String,
    pub org_unit: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

impl Subject {
    pub fn full_name(&self) -> String {
        let mut res = "CN=".to_string();
        res.push_str(&self.common);
        res.push_str(",C=");
        res.push_str(&self.country);
        res.push_str(",O=");
        res.push_str(&self.org);
        if let Some(org_unit) = &self.org_unit {
            res.push_str(",OU=");
            res.push_str(org_unit);
        }
        if let Some(state) = &self.state {
            res.push_str(",ST=");
            res.push_str(state);
        }
        if let Some(locality) = &self.locality {
            res.push_str(",L=");
            res.push_str(locality);
        }
        res
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Validity {
    pub days: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub bits: u32,
}
