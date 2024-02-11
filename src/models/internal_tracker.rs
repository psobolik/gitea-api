/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTracker {
    pub allow_only_contributors_to_track_time: bool,
    pub enable_issue_dependencies: bool,
    pub enable_time_tracker: bool,
}
