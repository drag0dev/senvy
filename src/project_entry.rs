use serde_derive::{Serialize, Deserialize};

/// name of the project is based on the file name
/// timestamp of request arrival to avoid packages that come later but are faster to process
#[derive(Serialize, Deserialize)]
struct ProjectEntry {
    /// timestamp of the last change
    timestamp: u128,
    vars: Vec<Var>,
}

/// name value pair - env var
#[derive(Serialize, Deserialize)]
struct Var {
    name: String,
    value: String,
}
