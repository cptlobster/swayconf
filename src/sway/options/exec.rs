use std::ops::Deref;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(PartialEq, Eq, Clone, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case", untagged)]
pub enum ExecParams {
    #[strum(serialize = "{0}")]
    String(String),
    #[strum(serialize = "{args}{command}")]
    Flagged {
        #[serde(default)]
        args: ExecFlags,
        command: String
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Exec {
    #[strum(serialize = "--no-startup-id")]
    NoStartupId
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecFlags(Vec<Exec>);

impl Deref for ExecFlags {
    type Target = Vec<Exec>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ExecFlags {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0
            .iter()
            .map(|a| format!("{a} "))
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl Default for ExecFlags {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl ExecFlags {
    pub fn new() -> Self {
        ExecFlags::default()
    }

    pub fn from(vec: Vec<Exec>) -> Self {
        ExecFlags(vec)
    }
}