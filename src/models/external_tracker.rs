/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalTracker {
    pub external_tracker_format: String,
    pub external_tracker_regexp_pattern: String,
    pub external_tracker_style: String,
    pub external_tracker_url: String,
}
