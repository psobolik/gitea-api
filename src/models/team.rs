/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use crate::models::organization::Organization;
use crate::models::units_map::UnitsMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub can_create_org_repo: bool,
    pub description: String,
    pub id: i64,
    pub includes_all_repositories: bool,
    pub name: String,
    pub organization: Organization,
    pub permission: String,
    pub units: Vec<String>,
    pub units_map: UnitsMap,
}
