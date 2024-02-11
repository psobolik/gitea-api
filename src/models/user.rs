/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-06
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u32,
    login: String,
    login_name: String,
    full_name: String,
    email: String,
    avatar_url: String,
    language: String,
    is_admin: bool,
    last_login: String,
    created: String,
    restricted: bool,
    active: bool,
    prohibit_login: bool,
    location: String,
    website: String,
    description: String,
    visibility: String,
    followers_count: u32,
    following_count: u32,
    starred_repos_count: u32,
    username: String,
}
