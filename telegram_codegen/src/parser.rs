use std::str::FromStr;
use serde_json;

#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Constructor {
    pub id: i32,
    pub predicate: String,
    pub params: Vec<Parameter>,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub id: i32,
    pub method: String,
    pub params: Vec<Parameter>,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize)]
pub struct Schema {
    pub constructors: Vec<Constructor>,
    pub methods: Vec<Method>,
}

impl FromStr for Schema {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
