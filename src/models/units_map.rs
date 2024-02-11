/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitsMap {
    #[serde(rename = "repo.code")]
    pub repo_code: String,
    #[serde(rename = "repo.ext_issues")]
    pub repo_ext_issues: String,
    #[serde(rename = "repo.ext_wiki")]
    pub repo_ext_wiki: String,
    #[serde(rename = "repo.issues")]
    pub repo_issues: String,
    #[serde(rename = "repo.projects")]
    pub repo_projects: String,
    #[serde(rename = "repo.pulls")]
    pub repo_pulls: String,
    #[serde(rename = "repo.releases")]
    pub repo_releases: String,
    #[serde(rename = "repo.wiki")]
    pub repo_wiki: String,
}
