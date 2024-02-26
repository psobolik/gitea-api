/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use crate::Organization;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Permission {
    None,
    Read,
    Write,
    Admin,
    Owner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub can_create_org_repo: bool,
    pub description: String,
    pub id: i64,
    pub includes_all_repositories: bool,
    pub name: String,
    pub organization: Organization,
    pub permission: Permission,
    pub units: Vec<String>,
    pub units_map: HashMap<String, Permission>,
}
