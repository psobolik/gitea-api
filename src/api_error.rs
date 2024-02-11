/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-10
 */

use std::fmt::{Debug, Display, Formatter};

pub enum ApiError {
    Reqwest(reqwest::Error),
    Url(url::ParseError),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ApiError::Reqwest(error) => error.to_string(),
            ApiError::Url(error) => error.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::Reqwest(err)
    }
}

impl From<url::ParseError> for ApiError {
    fn from(err: url::ParseError) -> ApiError {
        ApiError::Url(err)
    }
}
