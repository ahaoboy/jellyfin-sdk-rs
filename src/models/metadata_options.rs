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

/// MetadataOptions : Class MetadataOptions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataOptions {
    #[serde(
        rename = "ItemType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub item_type: Option<Option<String>>,
    #[serde(
        rename = "DisabledMetadataSavers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_metadata_savers: Option<Option<Vec<String>>>,
    #[serde(
        rename = "LocalMetadataReaderOrder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_metadata_reader_order: Option<Option<Vec<String>>>,
    #[serde(
        rename = "DisabledMetadataFetchers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_metadata_fetchers: Option<Option<Vec<String>>>,
    #[serde(
        rename = "MetadataFetcherOrder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetcher_order: Option<Option<Vec<String>>>,
    #[serde(
        rename = "DisabledImageFetchers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_image_fetchers: Option<Option<Vec<String>>>,
    #[serde(
        rename = "ImageFetcherOrder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetcher_order: Option<Option<Vec<String>>>,
}

impl MetadataOptions {
    /// Class MetadataOptions.
    pub fn new() -> MetadataOptions {
        MetadataOptions {
            item_type: None,
            disabled_metadata_savers: None,
            local_metadata_reader_order: None,
            disabled_metadata_fetchers: None,
            metadata_fetcher_order: None,
            disabled_image_fetchers: None,
            image_fetcher_order: None,
        }
    }
}
