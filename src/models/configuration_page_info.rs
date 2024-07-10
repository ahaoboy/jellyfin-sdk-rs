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

/// ConfigurationPageInfo : The configuration page info.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationPageInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets a value indicating whether the configurations page is enabled in the main menu.
    #[serde(rename = "EnableInMainMenu", skip_serializing_if = "Option::is_none")]
    pub enable_in_main_menu: Option<bool>,
    /// Gets or sets the menu section.
    #[serde(
        rename = "MenuSection",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub menu_section: Option<Option<String>>,
    /// Gets or sets the menu icon.
    #[serde(
        rename = "MenuIcon",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub menu_icon: Option<Option<String>>,
    /// Gets or sets the display name.
    #[serde(
        rename = "DisplayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<Option<String>>,
    /// Gets or sets the plugin id.
    #[serde(
        rename = "PluginId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_id: Option<Option<uuid::Uuid>>,
}

impl ConfigurationPageInfo {
    /// The configuration page info.
    pub fn new() -> ConfigurationPageInfo {
        ConfigurationPageInfo {
            name: None,
            enable_in_main_menu: None,
            menu_section: None,
            menu_icon: None,
            display_name: None,
            plugin_id: None,
        }
    }
}
