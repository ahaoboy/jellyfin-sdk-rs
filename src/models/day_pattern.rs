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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayPattern {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekdays")]
    Weekdays,
    #[serde(rename = "Weekends")]
    Weekends,
}

impl std::fmt::Display for DayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Daily => write!(f, "Daily"),
            Self::Weekdays => write!(f, "Weekdays"),
            Self::Weekends => write!(f, "Weekends"),
        }
    }
}

impl Default for DayPattern {
    fn default() -> DayPattern {
        Self::Daily
    }
}
