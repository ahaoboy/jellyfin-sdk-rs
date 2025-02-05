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
pub enum Architecture {
    #[serde(rename = "X86")]
    X86,
    #[serde(rename = "X64")]
    X64,
    #[serde(rename = "Arm")]
    Arm,
    #[serde(rename = "Arm64")]
    Arm64,
    #[serde(rename = "Wasm")]
    Wasm,
    #[serde(rename = "S390x")]
    S390x,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X86 => write!(f, "X86"),
            Self::X64 => write!(f, "X64"),
            Self::Arm => write!(f, "Arm"),
            Self::Arm64 => write!(f, "Arm64"),
            Self::Wasm => write!(f, "Wasm"),
            Self::S390x => write!(f, "S390x"),
        }
    }
}

impl Default for Architecture {
    fn default() -> Architecture {
        Self::X86
    }
}
