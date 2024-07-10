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

/// UpdateLibraryOptionsDto : Update library options dto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLibraryOptionsDto {
    /// Gets or sets the library item id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "LibraryOptions", skip_serializing_if = "Option::is_none")]
    pub library_options: Option<Box<models::LibraryOptions>>,
}

impl UpdateLibraryOptionsDto {
    /// Update library options dto.
    pub fn new() -> UpdateLibraryOptionsDto {
        UpdateLibraryOptionsDto {
            id: None,
            library_options: None,
        }
    }
}