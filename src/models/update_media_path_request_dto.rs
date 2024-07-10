/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateMediaPathRequestDto : Update library options dto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMediaPathRequestDto {
    /// Gets or sets the library name.
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PathInfo")]
    pub path_info: Box<models::MediaPathInfo>,
}

impl UpdateMediaPathRequestDto {
    /// Update library options dto.
    pub fn new(name: String, path_info: models::MediaPathInfo) -> UpdateMediaPathRequestDto {
        UpdateMediaPathRequestDto {
            name,
            path_info: Box::new(path_info),
        }
    }
}
