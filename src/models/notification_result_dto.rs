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

/// NotificationResultDto : A list of notifications with the total record count for pagination.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationResultDto {
    /// Gets or sets the current page of notifications.
    #[serde(rename = "Notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<models::NotificationDto>>,
    /// Gets or sets the total number of notifications.
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
}

impl NotificationResultDto {
    /// A list of notifications with the total record count for pagination.
    pub fn new() -> NotificationResultDto {
        NotificationResultDto {
            notifications: None,
            total_record_count: None,
        }
    }
}