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

/// ServerDiscoveryInfo : The server discovery info model.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerDiscoveryInfo {
    /// Gets the address.
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Gets the server identifier.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets the endpoint address.
    #[serde(
        rename = "EndpointAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_address: Option<Option<String>>,
}

impl ServerDiscoveryInfo {
    /// The server discovery info model.
    pub fn new() -> ServerDiscoveryInfo {
        ServerDiscoveryInfo {
            address: None,
            id: None,
            name: None,
            endpoint_address: None,
        }
    }
}
