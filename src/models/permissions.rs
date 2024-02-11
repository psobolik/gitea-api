/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}
