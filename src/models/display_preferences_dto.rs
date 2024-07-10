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

/// DisplayPreferencesDto : Defines the display preferences for any item that supports them (usually Folders).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayPreferencesDto {
    /// Gets or sets the user id.
    #[serde(
        rename = "Id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
    /// Gets or sets the type of the view.
    #[serde(
        rename = "ViewType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_type: Option<Option<String>>,
    /// Gets or sets the sort by.
    #[serde(
        rename = "SortBy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_by: Option<Option<String>>,
    /// Gets or sets the index by.
    #[serde(
        rename = "IndexBy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_by: Option<Option<String>>,
    /// Gets or sets a value indicating whether [remember indexing].
    #[serde(rename = "RememberIndexing", skip_serializing_if = "Option::is_none")]
    pub remember_indexing: Option<bool>,
    /// Gets or sets the height of the primary image.
    #[serde(rename = "PrimaryImageHeight", skip_serializing_if = "Option::is_none")]
    pub primary_image_height: Option<i32>,
    /// Gets or sets the width of the primary image.
    #[serde(rename = "PrimaryImageWidth", skip_serializing_if = "Option::is_none")]
    pub primary_image_width: Option<i32>,
    /// Gets or sets the custom prefs.
    #[serde(rename = "CustomPrefs", skip_serializing_if = "Option::is_none")]
    pub custom_prefs: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ScrollDirection", skip_serializing_if = "Option::is_none")]
    pub scroll_direction: Option<models::ScrollDirection>,
    /// Gets or sets a value indicating whether to show backdrops on this item.
    #[serde(rename = "ShowBackdrop", skip_serializing_if = "Option::is_none")]
    pub show_backdrop: Option<bool>,
    /// Gets or sets a value indicating whether [remember sorting].
    #[serde(rename = "RememberSorting", skip_serializing_if = "Option::is_none")]
    pub remember_sorting: Option<bool>,
    #[serde(rename = "SortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<models::SortOrder>,
    /// Gets or sets a value indicating whether [show sidebar].
    #[serde(rename = "ShowSidebar", skip_serializing_if = "Option::is_none")]
    pub show_sidebar: Option<bool>,
    /// Gets or sets the client.
    #[serde(
        rename = "Client",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client: Option<Option<String>>,
}

impl DisplayPreferencesDto {
    /// Defines the display preferences for any item that supports them (usually Folders).
    pub fn new() -> DisplayPreferencesDto {
        DisplayPreferencesDto {
            id: None,
            view_type: None,
            sort_by: None,
            index_by: None,
            remember_indexing: None,
            primary_image_height: None,
            primary_image_width: None,
            custom_prefs: None,
            scroll_direction: None,
            show_backdrop: None,
            remember_sorting: None,
            sort_order: None,
            show_sidebar: None,
            client: None,
        }
    }
}