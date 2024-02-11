/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub avatar_url: String,
    pub description: String,
    pub email: String,
    pub full_name: String,
    pub id: i64,
    pub location: String,
    pub name: String,
    pub repo_admin_change_team_access: bool,
    pub username: String,
    pub visibility: String,
    pub website: String,
}
