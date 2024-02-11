/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-10
 */
use crate::api_error::ApiError;
use crate::models::create_repo_options::CreateRepoOptions;
use crate::models::repository::Repository;
use url::Url;

pub mod api_error;
pub mod models;

pub struct GiteaApi {
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl GiteaApi {
    const API_BASE_PATH: &'static str = "api/v1";

    pub fn new(base_url: &str, username: Option<&str>, password: Option<&str>) -> GiteaApi {
        GiteaApi {
            base_url: base_url.to_owned(),
            username: if let Some(username) = username {
                Some(username.to_owned())
            } else {
                None
            },
            password: if let Some(password) = password {
                Some(password.to_owned())
            } else {
                None
            },
        }
    }

    // POST CreateRepoOptions to /user/repos
    // Create a repository
    pub async fn create_repo(&self, options: &CreateRepoOptions) -> Result<Repository, ApiError> {
        let path = vec!["user", "repos"];
        match self.build_url(path) {
            Ok(url) => {
                let client = reqwest::Client::new();
                let mut request_builder = client.post(url).json(options);
                if let Some(username) = &self.username {
                    request_builder = request_builder.basic_auth(username, self.password.as_ref());
                };
                let response = request_builder.send().await?;
                match response.error_for_status() {
                    Ok(response) => Ok(response.json::<Repository>().await?),
                    Err(error) => Err(ApiError::from(error)),
                }
            }
            Err(error) => Err(ApiError::from(error)),
        }
    }
}

impl GiteaApi {
    fn build_url(&self, path: Vec<&str>) -> Result<Url, url::ParseError> {
        let mut url_string = format!("{}/{}", self.base_url, Self::API_BASE_PATH);
        for item in path {
            url_string.push_str(format!("/{}", item).as_str());
        }
        Url::parse(url_string.as_str())
    }
}
