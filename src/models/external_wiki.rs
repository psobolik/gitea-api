/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-07
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalWiki {
    pub external_wiki_url: String,
}
