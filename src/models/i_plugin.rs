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

/// IPlugin : Defines the MediaBrowser.Common.Plugins.IPlugin.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPlugin {
    /// Gets the name of the plugin.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets the Description.
    #[serde(
        rename = "Description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// Gets the unique id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Gets the plugin version.
    #[serde(
        rename = "Version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<Option<String>>,
    /// Gets the path to the assembly file.
    #[serde(
        rename = "AssemblyFilePath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assembly_file_path: Option<Option<String>>,
    /// Gets a value indicating whether the plugin can be uninstalled.
    #[serde(rename = "CanUninstall", skip_serializing_if = "Option::is_none")]
    pub can_uninstall: Option<bool>,
    /// Gets the full path to the data folder, where the plugin can store any miscellaneous files needed.
    #[serde(
        rename = "DataFolderPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_folder_path: Option<Option<String>>,
}

impl IPlugin {
    /// Defines the MediaBrowser.Common.Plugins.IPlugin.
    pub fn new() -> IPlugin {
        IPlugin {
            name: None,
            description: None,
            id: None,
            version: None,
            assembly_file_path: None,
            can_uninstall: None,
            data_folder_path: None,
        }
    }
}
