/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-06
 */

use crate::models::external_tracker::ExternalTracker;
use crate::models::external_wiki::ExternalWiki;
use crate::models::internal_tracker::InternalTracker;
use crate::models::permissions::Permissions;
use crate::models::repo_transfer::RepoTransfer;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_rebase_update: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    pub archived_at: String,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_allow_maintainer_edit: bool,
    pub default_branch: String,
    pub default_delete_branch_after_merge: bool,
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    pub external_tracker: Option<ExternalTracker>,
    pub external_wiki: Option<ExternalWiki>,
    pub fork: bool,
    pub forks_count: i64,
    pub full_name: String,
    pub has_actions: bool,
    pub has_issues: bool,
    pub has_packages: bool,
    pub has_projects: bool,
    pub has_pull_requests: bool,
    pub has_releases: bool,
    pub has_wiki: bool,
    pub html_url: String,
    pub id: i64,
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    pub internal_tracker: Option<InternalTracker>,
    pub language: String,
    pub languages_url: String,
    pub link: String,
    pub mirror: bool,
    pub mirror_interval: String,
    pub mirror_updated: String,
    pub name: String,
    pub open_issues_count: i64,
    pub open_pr_counter: i64,
    pub original_url: String,
    pub owner: User,
    pub parent: Option<String>,
    pub permissions: Option<Permissions>,
    pub private: bool,
    pub release_counter: i64,
    pub repo_transfer: Option<RepoTransfer>,
    pub size: i64,
    pub ssh_url: String,
    pub stars_count: i64,
    pub template: bool,
    pub updated_at: String,
    pub url: String,
    pub watchers_count: i64,
    pub website: String,
}
