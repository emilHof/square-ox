use serde::{Deserialize, Serialize};
use crate::endpoint::locations::Locations;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SquareResponse {
    Locations(Vec<Location>),
    Payment {
        id: String,
        status: String,
        order_id: String,
        receipt_number: String,
        receipt_url: String,
    },
    Order {
        random_name: String,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Location {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    capabilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    created_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    coordinates: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    country: Option<String>,
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    facebook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    full_format_logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    instagram_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pos_background_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    tax_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    twitter_username: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    business_hours: Option<Periods>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    website_url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    address_line_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    address_line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    address_line_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    administrative_district_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    country: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Coordinates {
    longitude: f64,
    latitude: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Periods {
    periods: Vec<Period>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Period {
    day_of_week: String,
    start_local_time: String,
    end_local_time: String,
}
