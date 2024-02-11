/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-12
 */

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TrustModel {
    Default,
    Collaborator,
    Committer,
    CollaboratorCommitter,
}
impl std::str::FromStr for TrustModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(TrustModel::Default),
            "collaborator" => Ok(TrustModel::Collaborator),
            "committer" => Ok(TrustModel::Committer),
            "collaboratorcommitter" => Ok(TrustModel::CollaboratorCommitter),
            _ => Err("Invalid trust model".to_string()),
        }
    }
}

impl std::fmt::Display for TrustModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustModel::Default => {
                write!(f, "Default")
            }
            TrustModel::Collaborator => {
                write!(f, "Collaborator")
            }
            TrustModel::Committer => {
                write!(f, "Committer")
            }
            TrustModel::CollaboratorCommitter => {
                write!(f, "CollaboratorCommitter")
            }
        }
    }
}
