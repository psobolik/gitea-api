/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-06
 */
use crate::TrustModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateRepoOptions {
    pub name: String,
    pub default_branch: String,
    pub trust_model: TrustModel,
    pub auto_init: bool,
    pub private: bool,
    pub template: bool,
    pub description: Option<String>,
    pub gitignores: Option<String>,
    pub issue_labels: Option<String>,
    pub license: Option<String>,
    pub readme: Option<String>,
}

impl CreateRepoOptions {
    pub fn default(name: &str) -> CreateRepoOptions {
        CreateRepoOptions {
            name: name.to_string(),
            default_branch: "main".to_string(),
            trust_model: TrustModel::Default,
            auto_init: false,
            private: false,
            template: false,
            description: None,
            gitignores: None,
            issue_labels: None,
            license: None,
            readme: None,
        }
    }

    pub fn new(
        name: String,
        default_branch: String,
        trust_model: TrustModel,
        auto_init: bool,
        private: bool,
        template: bool,
        description: Option<String>,
        gitignores: Option<String>,
        issue_labels: Option<String>,
        license: Option<String>,
        readme: Option<String>,
    ) -> CreateRepoOptions {
        CreateRepoOptions {
            name,
            default_branch,
            trust_model,
            auto_init,
            private,
            template,
            description,
            gitignores,
            issue_labels,
            license,
            readme,
        }
    }
}
