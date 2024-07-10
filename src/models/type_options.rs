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
pub struct TypeOptions {
    #[serde(
        rename = "Type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<Option<String>>,
    #[serde(
        rename = "MetadataFetchers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetchers: Option<Option<Vec<String>>>,
    #[serde(
        rename = "MetadataFetcherOrder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetcher_order: Option<Option<Vec<String>>>,
    #[serde(
        rename = "ImageFetchers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetchers: Option<Option<Vec<String>>>,
    #[serde(
        rename = "ImageFetcherOrder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetcher_order: Option<Option<Vec<String>>>,
    #[serde(
        rename = "ImageOptions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_options: Option<Option<Vec<models::ImageOption>>>,
}

impl TypeOptions {
    pub fn new() -> TypeOptions {
        TypeOptions {
            r#type: None,
            metadata_fetchers: None,
            metadata_fetcher_order: None,
            image_fetchers: None,
            image_fetcher_order: None,
            image_options: None,
        }
    }
}
