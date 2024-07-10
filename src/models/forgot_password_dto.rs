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

/// ForgotPasswordDto : Forgot Password request body DTO.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForgotPasswordDto {
    /// Gets or sets the entered username to have its password reset.
    #[serde(rename = "EnteredUsername")]
    pub entered_username: String,
}

impl ForgotPasswordDto {
    /// Forgot Password request body DTO.
    pub fn new(entered_username: String) -> ForgotPasswordDto {
        ForgotPasswordDto { entered_username }
    }
}
