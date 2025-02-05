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
pub struct QueryFiltersLegacy {
    #[serde(
        rename = "Genres",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(
        rename = "Tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(
        rename = "OfficialRatings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub official_ratings: Option<Option<Vec<String>>>,
    #[serde(
        rename = "Years",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub years: Option<Option<Vec<i32>>>,
}

impl QueryFiltersLegacy {
    pub fn new() -> QueryFiltersLegacy {
        QueryFiltersLegacy {
            genres: None,
            tags: None,
            official_ratings: None,
            years: None,
        }
    }
}
