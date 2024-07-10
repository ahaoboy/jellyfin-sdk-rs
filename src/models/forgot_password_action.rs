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
pub enum ForgotPasswordAction {
    #[serde(rename = "ContactAdmin")]
    ContactAdmin,
    #[serde(rename = "PinCode")]
    PinCode,
    #[serde(rename = "InNetworkRequired")]
    InNetworkRequired,
}

impl std::fmt::Display for ForgotPasswordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ContactAdmin => write!(f, "ContactAdmin"),
            Self::PinCode => write!(f, "PinCode"),
            Self::InNetworkRequired => write!(f, "InNetworkRequired"),
        }
    }
}

impl Default for ForgotPasswordAction {
    fn default() -> ForgotPasswordAction {
        Self::ContactAdmin
    }
}