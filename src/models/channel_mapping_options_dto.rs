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

/// ChannelMappingOptionsDto : Channel mapping options dto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelMappingOptionsDto {
    /// Gets or sets list of tuner channels.
    #[serde(rename = "TunerChannels", skip_serializing_if = "Option::is_none")]
    pub tuner_channels: Option<Vec<models::TunerChannelMapping>>,
    /// Gets or sets list of provider channels.
    #[serde(rename = "ProviderChannels", skip_serializing_if = "Option::is_none")]
    pub provider_channels: Option<Vec<models::NameIdPair>>,
    /// Gets or sets list of mappings.
    #[serde(rename = "Mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<models::NameValuePair>>,
    /// Gets or sets provider name.
    #[serde(
        rename = "ProviderName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<Option<String>>,
}

impl ChannelMappingOptionsDto {
    /// Channel mapping options dto.
    pub fn new() -> ChannelMappingOptionsDto {
        ChannelMappingOptionsDto {
            tuner_channels: None,
            provider_channels: None,
            mappings: None,
            provider_name: None,
        }
    }
}
