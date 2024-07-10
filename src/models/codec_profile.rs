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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodecProfile {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::CodecType>,
    #[serde(
        rename = "Conditions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Option<Vec<models::ProfileCondition>>>,
    #[serde(
        rename = "ApplyConditions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_conditions: Option<Option<Vec<models::ProfileCondition>>>,
    #[serde(
        rename = "Codec",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub codec: Option<Option<String>>,
    #[serde(
        rename = "Container",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<Option<String>>,
}

impl CodecProfile {
    pub fn new() -> CodecProfile {
        CodecProfile {
            r#type: None,
            conditions: None,
            apply_conditions: None,
            codec: None,
            container: None,
        }
    }
}
