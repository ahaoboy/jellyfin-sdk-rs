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

/// TaskState : Enum TaskState.
/// Enum TaskState.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskState {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Cancelling")]
    Cancelling,
    #[serde(rename = "Running")]
    Running,
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Cancelling => write!(f, "Cancelling"),
            Self::Running => write!(f, "Running"),
        }
    }
}

impl Default for TaskState {
    fn default() -> TaskState {
        Self::Idle
    }
}
