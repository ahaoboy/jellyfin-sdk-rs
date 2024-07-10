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
pub enum LiveTvServiceStatus {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "Unavailable")]
    Unavailable,
}

impl std::fmt::Display for LiveTvServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "Ok"),
            Self::Unavailable => write!(f, "Unavailable"),
        }
    }
}

impl Default for LiveTvServiceStatus {
    fn default() -> LiveTvServiceStatus {
        Self::Ok
    }
}