/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-25
 */

use crate::Repository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchReposResult {
    ok: bool,
    data: Vec<Repository>,
}

impl SearchReposResult {
    pub fn ok(&self) -> bool {
        self.ok
    }
    pub fn repositories(&self) -> &Vec<Repository> {
        &self.data
    }
}
