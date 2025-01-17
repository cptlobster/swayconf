use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecFlags {
    #[strum(serialize = "--no-startup-id")]
    NoStartupId
}