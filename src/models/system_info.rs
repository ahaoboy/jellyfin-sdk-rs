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

/// SystemInfo : Class SystemInfo.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Gets or sets the local address.
    #[serde(
        rename = "LocalAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_address: Option<Option<String>>,
    /// Gets or sets the name of the server.
    #[serde(
        rename = "ServerName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<Option<String>>,
    /// Gets or sets the server version.
    #[serde(
        rename = "Version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<Option<String>>,
    /// Gets or sets the product name. This is the AssemblyProduct name.
    #[serde(
        rename = "ProductName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<Option<String>>,
    /// Gets or sets the operating system.
    #[serde(
        rename = "OperatingSystem",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system: Option<Option<String>>,
    /// Gets or sets the id.
    #[serde(
        rename = "Id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
    /// Gets or sets a value indicating whether the startup wizard is completed.
    #[serde(
        rename = "StartupWizardCompleted",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_wizard_completed: Option<Option<bool>>,
    /// Gets or sets the display name of the operating system.
    #[serde(
        rename = "OperatingSystemDisplayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system_display_name: Option<Option<String>>,
    /// Gets or sets the package name.
    #[serde(
        rename = "PackageName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<Option<String>>,
    /// Gets or sets a value indicating whether this instance has pending restart.
    #[serde(rename = "HasPendingRestart", skip_serializing_if = "Option::is_none")]
    pub has_pending_restart: Option<bool>,
    #[serde(rename = "IsShuttingDown", skip_serializing_if = "Option::is_none")]
    pub is_shutting_down: Option<bool>,
    /// Gets or sets a value indicating whether [supports library monitor].
    #[serde(
        rename = "SupportsLibraryMonitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_library_monitor: Option<bool>,
    /// Gets or sets the web socket port number.
    #[serde(
        rename = "WebSocketPortNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_socket_port_number: Option<i32>,
    /// Gets or sets the completed installations.
    #[serde(
        rename = "CompletedInstallations",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_installations: Option<Option<Vec<models::InstallationInfo>>>,
    /// Gets or sets a value indicating whether this instance can self restart.
    #[serde(rename = "CanSelfRestart", skip_serializing_if = "Option::is_none")]
    pub can_self_restart: Option<bool>,
    #[serde(
        rename = "CanLaunchWebBrowser",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_launch_web_browser: Option<bool>,
    /// Gets or sets the program data path.
    #[serde(
        rename = "ProgramDataPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub program_data_path: Option<Option<String>>,
    /// Gets or sets the web UI resources path.
    #[serde(
        rename = "WebPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_path: Option<Option<String>>,
    /// Gets or sets the items by name path.
    #[serde(
        rename = "ItemsByNamePath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub items_by_name_path: Option<Option<String>>,
    /// Gets or sets the cache path.
    #[serde(
        rename = "CachePath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_path: Option<Option<String>>,
    /// Gets or sets the log path.
    #[serde(
        rename = "LogPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_path: Option<Option<String>>,
    /// Gets or sets the internal metadata path.
    #[serde(
        rename = "InternalMetadataPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_metadata_path: Option<Option<String>>,
    /// Gets or sets the transcode path.
    #[serde(
        rename = "TranscodingTempPath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_temp_path: Option<Option<String>>,
    /// Gets or sets a value indicating whether this instance has update available.
    #[serde(rename = "HasUpdateAvailable", skip_serializing_if = "Option::is_none")]
    pub has_update_available: Option<bool>,
    #[serde(rename = "EncoderLocation", skip_serializing_if = "Option::is_none")]
    pub encoder_location: Option<models::FFmpegLocation>,
    #[serde(rename = "SystemArchitecture", skip_serializing_if = "Option::is_none")]
    pub system_architecture: Option<models::Architecture>,
}

impl SystemInfo {
    /// Class SystemInfo.
    pub fn new() -> SystemInfo {
        SystemInfo {
            local_address: None,
            server_name: None,
            version: None,
            product_name: None,
            operating_system: None,
            id: None,
            startup_wizard_completed: None,
            operating_system_display_name: None,
            package_name: None,
            has_pending_restart: None,
            is_shutting_down: None,
            supports_library_monitor: None,
            web_socket_port_number: None,
            completed_installations: None,
            can_self_restart: None,
            can_launch_web_browser: None,
            program_data_path: None,
            web_path: None,
            items_by_name_path: None,
            cache_path: None,
            log_path: None,
            internal_metadata_path: None,
            transcoding_temp_path: None,
            has_update_available: None,
            encoder_location: None,
            system_architecture: None,
        }
    }
}
