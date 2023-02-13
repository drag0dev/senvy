use serde_derive::{Serialize, Deserialize};

/// data about a project when creating a new one or updating already existing
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Project{
    /// project name
    pub name: String,
    pub path: String,
    pub vars: Vec<Var>,
}

/// name of the project is based on the file name
/// timestamp of request arrival to avoid packages that come later but are faster to process
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProjectEntry {
    /// timestamp of the last change
    pub timestamp: u128,
    pub path: String,
    pub vars: Vec<Var>,
}

/// name value pair - env var
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    pub value: String,
}
