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

/// TaskCompletionStatus : Enum TaskCompletionStatus.
/// Enum TaskCompletionStatus.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskCompletionStatus {
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Aborted")]
    Aborted,
}

impl std::fmt::Display for TaskCompletionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
            Self::Cancelled => write!(f, "Cancelled"),
            Self::Aborted => write!(f, "Aborted"),
        }
    }
}

impl Default for TaskCompletionStatus {
    fn default() -> TaskCompletionStatus {
        Self::Completed
    }
}
