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

/// DynamicDayOfWeek : An enum that represents a day of the week, weekdays, weekends, or all days.
/// An enum that represents a day of the week, weekdays, weekends, or all days.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DynamicDayOfWeek {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
    #[serde(rename = "Everyday")]
    Everyday,
    #[serde(rename = "Weekday")]
    Weekday,
    #[serde(rename = "Weekend")]
    Weekend,
}

impl std::fmt::Display for DynamicDayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Sunday => write!(f, "Sunday"),
            Self::Monday => write!(f, "Monday"),
            Self::Tuesday => write!(f, "Tuesday"),
            Self::Wednesday => write!(f, "Wednesday"),
            Self::Thursday => write!(f, "Thursday"),
            Self::Friday => write!(f, "Friday"),
            Self::Saturday => write!(f, "Saturday"),
            Self::Everyday => write!(f, "Everyday"),
            Self::Weekday => write!(f, "Weekday"),
            Self::Weekend => write!(f, "Weekend"),
        }
    }
}

impl Default for DynamicDayOfWeek {
    fn default() -> DynamicDayOfWeek {
        Self::Sunday
    }
}
