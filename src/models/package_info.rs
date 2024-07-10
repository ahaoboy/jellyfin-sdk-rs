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

/// PackageInfo : Class PackageInfo.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Gets or sets the name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets a long description of the plugin containing features or helpful explanations.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Gets or sets a short overview of what the plugin does.
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    /// Gets or sets the owner.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Gets or sets the category.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Gets or sets the guid of the assembly associated with this plugin.  This is used to identify the proper item for automatic updates.
    #[serde(rename = "guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<uuid::Uuid>,
    /// Gets or sets the versions.
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<models::VersionInfo>>,
    /// Gets or sets the image url for the package.
    #[serde(
        rename = "imageUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_url: Option<Option<String>>,
}

impl PackageInfo {
    /// Class PackageInfo.
    pub fn new() -> PackageInfo {
        PackageInfo {
            name: None,
            description: None,
            overview: None,
            owner: None,
            category: None,
            guid: None,
            versions: None,
            image_url: None,
        }
    }
}
