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

/// struct for typed errors of method [`get_genre`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGenreError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_genres`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGenresError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

pub async fn get_genre(
    configuration: &configuration::Configuration,
    genre_name: &str,
    user_id: Option<&str>,
) -> Result<models::BaseItemDto, Error<GetGenreError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/Genres/{genreName}",
        local_var_configuration.base_path,
        genreName = crate::apis::urlencode(genre_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGenreError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_genres(
    configuration: &configuration::Configuration,
    start_index: Option<i32>,
    limit: Option<i32>,
    search_term: Option<&str>,
    parent_id: Option<&str>,
    fields: Option<Vec<models::ItemFields>>,
    exclude_item_types: Option<Vec<models::BaseItemKind>>,
    include_item_types: Option<Vec<models::BaseItemKind>>,
    is_favorite: Option<bool>,
    image_type_limit: Option<i32>,
    enable_image_types: Option<Vec<models::ImageType>>,
    user_id: Option<&str>,
    name_starts_with_or_greater: Option<&str>,
    name_starts_with: Option<&str>,
    name_less_than: Option<&str>,
    sort_by: Option<Vec<String>>,
    sort_order: Option<Vec<models::SortOrder>>,
    enable_images: Option<bool>,
    enable_total_record_count: Option<bool>,
) -> Result<models::BaseItemDtoQueryResult, Error<GetGenresError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Genres", local_var_configuration.base_path);
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
    if let Some(ref local_var_str) = search_term {
        local_var_req_builder =
            local_var_req_builder.query(&[("searchTerm", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("parentId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
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
    if let Some(ref local_var_str) = is_favorite {
        local_var_req_builder =
            local_var_req_builder.query(&[("isFavorite", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = image_type_limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("imageTypeLimit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_image_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("enableImageTypes".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "enableImageTypes",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_starts_with_or_greater {
        local_var_req_builder =
            local_var_req_builder.query(&[("nameStartsWithOrGreater", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_starts_with {
        local_var_req_builder =
            local_var_req_builder.query(&[("nameStartsWith", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_less_than {
        local_var_req_builder =
            local_var_req_builder.query(&[("nameLessThan", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_by {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("sortBy".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "sortBy",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = sort_order {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .iter()
                    .map(|p| ("sortOrder".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "sortOrder",
                &local_var_str
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = enable_images {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableImages", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_total_record_count {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableTotalRecordCount", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGenresError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
