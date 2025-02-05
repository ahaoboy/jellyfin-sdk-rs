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
pub struct RecommendationDto {
    #[serde(
        rename = "Items",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub items: Option<Option<Vec<models::BaseItemDto>>>,
    #[serde(rename = "RecommendationType", skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<models::RecommendationType>,
    #[serde(
        rename = "BaselineItemName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub baseline_item_name: Option<Option<String>>,
    #[serde(rename = "CategoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<uuid::Uuid>,
}

impl RecommendationDto {
    pub fn new() -> RecommendationDto {
        RecommendationDto {
            items: None,
            recommendation_type: None,
            baseline_item_name: None,
            category_id: None,
        }
    }
}
