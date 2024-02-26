/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use crate::{Team, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoTransfer {
    pub doer: User,
    pub recipient: User,
    pub teams: Vec<Team>,
}
