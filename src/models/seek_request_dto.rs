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

/// SeekRequestDto : Class SeekRequestDto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeekRequestDto {
    /// Gets or sets the position ticks.
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
}

impl SeekRequestDto {
    /// Class SeekRequestDto.
    pub fn new() -> SeekRequestDto {
        SeekRequestDto {
            position_ticks: None,
        }
    }
}
