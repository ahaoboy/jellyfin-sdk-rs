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

/// DeviceOptionsDto : A dto representing custom options for a device.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceOptionsDto {
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Gets or sets the device id.
    #[serde(
        rename = "DeviceId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_id: Option<Option<String>>,
    /// Gets or sets the custom name.
    #[serde(
        rename = "CustomName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_name: Option<Option<String>>,
}

impl DeviceOptionsDto {
    /// A dto representing custom options for a device.
    pub fn new() -> DeviceOptionsDto {
        DeviceOptionsDto {
            id: None,
            device_id: None,
            custom_name: None,
        }
    }
}
