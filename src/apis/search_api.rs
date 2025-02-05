/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

pub async fn get(
    configuration: &configuration::Configuration,
    search_term: &str,
    start_index: Option<i32>,
    limit: Option<i32>,
    user_id: Option<&str>,
    include_item_types: Option<Vec<models::BaseItemKind>>,
    exclude_item_types: Option<Vec<models::BaseItemKind>>,
    media_types: Option<Vec<String>>,
    parent_id: Option<&str>,
    is_movie: Option<bool>,
    is_series: Option<bool>,
    is_news: Option<bool>,
    is_kids: Option<bool>,
    is_sports: Option<bool>,
    include_people: Option<bool>,
    include_media: Option<bool>,
    include_genres: Option<bool>,
    include_studios: Option<bool>,
    include_artists: Option<bool>,
) -> Result<models::SearchHintResult, Error<GetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Search/Hints", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_index {
        local_var_req_builder =
            local_var_req_builder.query(&[("startIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    local_var_req_builder =
        local_var_req_builder.query(&[("searchTerm", &search_term.to_string())]);
    if let Some(ref local_var_str) = include_item_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("includeItemTypes".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "includeItemTypes",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = exclude_item_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("excludeItemTypes".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "excludeItemTypes",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = media_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("mediaTypes".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "mediaTypes",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = parent_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("parentId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_movie {
        local_var_req_builder =
            local_var_req_builder.query(&[("isMovie", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_series {
        local_var_req_builder =
            local_var_req_builder.query(&[("isSeries", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_news {
        local_var_req_builder =
            local_var_req_builder.query(&[("isNews", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_kids {
        local_var_req_builder =
            local_var_req_builder.query(&[("isKids", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_sports {
        local_var_req_builder =
            local_var_req_builder.query(&[("isSports", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_people {
        local_var_req_builder =
            local_var_req_builder.query(&[("includePeople", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_media {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeMedia", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_genres {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeGenres", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_studios {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeStudios", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_artists {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeArtists", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
