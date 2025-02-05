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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieInfoRemoteSearchQuery {
    #[serde(rename = "SearchInfo", skip_serializing_if = "Option::is_none")]
    pub search_info: Option<Box<models::MovieInfo>>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    /// Gets or sets the provider name to search within if set.
    #[serde(
        rename = "SearchProviderName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<Option<String>>,
    /// Gets or sets a value indicating whether disabled providers should be included.
    #[serde(
        rename = "IncludeDisabledProviders",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
}

impl MovieInfoRemoteSearchQuery {
    pub fn new() -> MovieInfoRemoteSearchQuery {
        MovieInfoRemoteSearchQuery {
            search_info: None,
            item_id: None,
            search_provider_name: None,
            include_disabled_providers: None,
        }
    }
}
