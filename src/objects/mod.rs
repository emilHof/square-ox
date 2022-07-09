/*!
A non-comprehensive list of the Objects used by the
[Square API](https://developer.squareup.com).
*/

pub mod enums;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::json_internal_vec;
use crate::objects::enums::{BusinessAppointmentSettingsBookingLocationType, BusinessAppointmentSettingsCancellationPolicy, BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType, BusinessBookingProfileBookingPolicy, BusinessBookingProfileCustomerTimezoneChoice, CatalogCustomAttributeDefinitionType, CatalogItemProductType, CatalogObjectType, CatalogPricingType, Currency, InventoryAlertType, LocationStatus, LocationType, OrderFulfillmentFulfillmentLineItemApplication, OrderFulfillmentPickupDetailsScheduleType, OrderLineItemDiscountScope, OrderLineItemDiscountType, OrderLineItemItemType, OrderLineItemTaxScope, OrderLineItemTaxType, OrderServiceChargeCalculationPhase, OrderServiceChargeType, OrderState, RefundStatus, SortOrder, TenderCardDetailsEntryMethod, TenderCardDetailsStatus, TenderType};

/// The Response enum holds the variety of responses that can be returned from a
/// [Square API](https://developer.squareup.com) call.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Response {
    Payment {
        id: String,
        status: String,
        order_id: String,
        receipt_number: String,
        receipt_url: String,
    },
    Order {
        random_name: String,
    },

    // Locations Endpoint Responses
    Locations(Vec<Location>),

    // Customer Endpoint Responses
    Customer(Customer),
    Customers(Vec<Customer>),

    // Catalog Endpoint Responses
    Objects(Vec<CatalogObject>),
    CatalogObject(CatalogObject),
    Items(Vec<CatalogObject>),
    MatchedVariationIds(Vec<String>),
    Limits(CatalogInfoResponseLimits),
    StandardUnitDescriptionGroup(StandardUnitDescriptionGroup),
    RelatedObjects(Vec<CatalogObject>),

    // Bookings Endpoint Responses
    Booking(Booking),
    Bookings(Vec<Booking>),
    Availabilities(Vec<Availability>),
    BusinessBookingProfile(BusinessBookingProfile),
    TeamMemberBookingProfiles(Vec<TeamMemberBookingProfile>),
    TeamMemberBookingProfile(TeamMemberBookingProfile),

    // Cards Endpoint Responses
    Cards(Vec<Card>),
    Card(Card),

    // Checkout Endpoint Responses
    Checkout(Checkout),
    PaymentLinks(Vec<PaymentLink>),
    PaymentLink(PaymentLink),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_email: Option<String>,
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LocationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_format_logo_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instagram_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos_background_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<TaxIds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<LocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_hours: Option<BusinessHours>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line_3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sublocality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_district_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BusinessHours {
    pub periods: Vec<BusinessHoursPeriod>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BusinessHoursPeriod {
    pub day_of_week: String,
    pub start_local_time: String,
    pub end_local_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Availability {
    pub start_at: String,
    pub location_id: String,
    pub appointment_segments: Vec<AppointmentSegment>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppointmentSegment {
    pub duration_minutes: f64,
    pub team_member_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_team_member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intermission_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<String>,
    pub service_variation_id: String,
    pub service_variation_version: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Customer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub  given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<TaxIds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<Card>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Preferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_subscribed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TaxIds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eu_vat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_siret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_naf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub es_nif: Option<String>,
}

/// Representation of a Credit/Debit Card for the crate and the Square API.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Card {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_co_brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct FilterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub  all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absent_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_data: Option<CatalogCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attributes_values: Option<HashMap<String, CatalogCustomAttributeValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_data: Option<CatalogDiscount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_data: Option<CatalogImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_data: Option<CatalogItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_data: Option<CatalogObjectOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_data: Option<CatalogModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_data: Option<CatalogModifierList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_all_locations: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_rule_data: Option<CatalogPricingRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_set_data: Option<CatalogProductSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_amount_settings_data: Option<CatalogQuickAmountsSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_data: Option<CatalogTax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_period_data: Option<CatalogTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogObjectVariation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absent_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_data: Option<CatalogCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attributes_values: Option<HashMap<String, CatalogCustomAttributeValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_data: Option<CatalogDiscount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_data: Option<CatalogImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_data: Option<CatalogObjectOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_variation_data: Option<CatalogItemVariation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_data: Option<CatalogModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_data: Option<CatalogModifierList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_all_locations: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_rule_data: Option<CatalogPricingRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_set_data: Option<CatalogProductSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_amount_settings_data: Option<CatalogQuickAmountsSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_data: Option<CatalogTax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_period_data: Option<CatalogTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogObjectOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absent_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_data: Option<CatalogCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attributes_values: Option<HashMap<String, CatalogCustomAttributeValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_data: Option<CatalogDiscount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_data: Option<CatalogImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_value_data: Option<CatalogItemOptionValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_variation_data: Option<CatalogItemVariation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_data: Option<CatalogModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_data: Option<CatalogModifierList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_all_locations: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub present_at_location_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_rule_data: Option<CatalogPricingRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_set_data: Option<CatalogProductSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_amount_settings_data: Option<CatalogQuickAmountsSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_data: Option<CatalogTax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_period_data: Option<CatalogTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogV1ID {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_v1_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCategory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allowed_object_types: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    app_visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    custom_attribute_usage_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    number_config: Option<CatalogCustomAttributeDefinitionNumberConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    selection_config: Option<CatalogCustomAttributeDefinitionSelectionConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    seller_visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_application: Option<SourceApplication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string_config: Option<CatalogCustomAttributeDefinitionStringConfig>,

}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeDefinitionNumberConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_selections: Option<Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_allowed_selections: Option<i32>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SourceApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeDefinitionStringConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_uniqueness: Option<bool>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogCustomAttributeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_uid_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(rename="type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<CatalogCustomAttributeDefinitionType>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogDiscount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_tax_basis: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin_required: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_studio_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_electronically: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_for_pickup: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_online: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids : Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_option: Option<Vec<CatalogItemOptionForItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="type", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<CatalogItemProductType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_modifier_scree: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<CatalogObjectVariation>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemOptionForItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemModifierListInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_selected_modifiers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_selected_modifiers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_overrides: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogModifierOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_by_default: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_colors: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<CatalogItemOptionValue>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemOptionValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemVariation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_for_booking: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_alert_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_alert_type: Option<InventoryAlertType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_values: Option<Vec<CatalogItemOptionValueForItemVariation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_overrides: Option<Vec<ItemVariationLocationOverrides>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<CatalogPricingType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sellable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stockable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stockable_conversion: Option<CatalogStockConversion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_inventory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub  upc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogItemOptionValueForItemVariation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_value_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ItemVariationLocationOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_alert_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_alert_type: Option<InventoryAlertType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<CatalogPricingType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sold_out: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sold_out_valid_until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_inventory: Option<bool>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogStockConversion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonstockable_quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stockable_item_variation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stockable_quantity: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogMeasurementUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<MeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct MeasurementUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_unit: Option<MeasurementUnitCustom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    #[serde(rename= "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct MeasurementUnitCustom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_list_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogModifierList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<CatalogModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_type: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogPricingRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_group_ids_any: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_products_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_strategy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_products_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_order_subtotal_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_period_ids: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from_local_time: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until_local_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_products_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogProductSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_products: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ids_all: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ids_any: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity_exact: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity_max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity_min: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQuickAmountsSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<CatalogQuickAmount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible_for_auto_amounts: Option<bool>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQuickAmount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    #[serde(rename="type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogSubscriptionPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<SubscriptionPhase>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SubscriptionPhase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cadence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periods: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogTax {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applies_to_custom_amounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculation_phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inclusion_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogTimePeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct Booking {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appointment_segments: Option<Vec<AppointmentSegment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_creator_details: Option<BookingCreatorDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<BusinessAppointmentSettingsBookingLocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_time_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct BookingCreatorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

/// Representation of Money for the crate.
/// The amount is given in the lowest possible denomination.
/// So for GBP the amount is in pence.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Money {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessBookingProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_user_cancel: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_policy: Option<BusinessBookingProfileBookingPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_appointment_settings: Option<BusinessAppointmentSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_timezone_choice: Option<BusinessBookingProfileCustomerTimezoneChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_seller_level_writes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessAppointmentSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_team_member_booking_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<BusinessAppointmentSettingsCancellationPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_window_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_types: Option<Vec<BusinessAppointmentSettingsBookingLocationType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_appointments_per_day_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_appointments_per_day_limit_type: Option<BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_booking_lead_time_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_booking_lead_time_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_service_booking_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_booking_flow_staff_selection: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMemberBookingProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bookable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub idempotency_key: String,
    pub order: Order,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct Order {
    pub id: Option<String>,
    pub location_id: Option<String>,
    pub close_at: Option<String>,
    pub created_at: Option<String>,
    pub customer_id: Option<String>,
    pub discounts: Option<Vec<OrderLineItemDiscount>>,
    pub fulfillments: Option<OrderFulfillment>,
    pub line_items: Option<Vec<OrderLineItem>>,
    pub metadata: Option<HashMap<String, String>>,
    pub net_amounts: Option<OrderMoneyAmounts>,
    pub pricing_options: Option<OrderPricingOptions>,
    pub reference_id: Option<String>,
    pub refunds: Option<Vec<Refund>>,
    pub return_amounts: Option<OrderMoneyAmounts>,
    pub returns: Option<Vec<OrderReturn>>,
    pub rewards: Option<Vec<OrderReward>>,
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    pub service_charges: Option<Vec<OrderServiceChargeType>>,
    pub source: Option<OrderSource>,
    pub state: Option<OrderState>,
    pub taxes: Option<Vec<OrderLineItemTax>>,
    pub tenders: Option<Vec<Tender>>,
    pub ticket_name: Option<String>,
    pub total_discount_money: Option<Money>,
    pub total_money: Option<Money>,
    pub total_service_charge_money: Option<Money>,
    pub total_tax_money: Option<Money>,
    pub total_tip_money: Option<Money>,
    pub updated_at: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ChargeRequestAdditionalRecipient {

}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemDiscount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reward_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<OrderLineItemDiscountScope>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<OrderLineItemDiscountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entries: Option<Vec<OrderFulfillmentFulfillmentEntry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_item_application: Option<OrderFulfillmentFulfillmentLineItemApplication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pickup_details: Option<OrderFulfillmentPickupDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shipment_details: Option<OrderFulfillmentShipmentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    fulfillment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillmentFulfillmentEntry {
    pub line_item_uid: String,
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillmentPickupDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_complete_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curbside_pickup_details: Option<OrderFulfillmentPickupDetailsCurbsidePickupDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_curbside_pickup: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picked_up_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_window_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prep_time_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrderFulfillmentRecipient>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<OrderFulfillmentPickupDetailsScheduleType>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillmentPickupDetailsCurbsidePickupDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_arrived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curbside_details: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillmentRecipient {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderFulfillmentShipmentDetails {
    cancel_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    carrier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expected_shipped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    in_progress_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    packaged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    placed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recipient: Option<OrderFulfillmentRecipient>,
    shipped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shipping_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shipping_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tracking_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tracking_url: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItem {
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_sales_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<OrderLineItemItemType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<OrderLineItemModifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub  pricing_blocklists: Option<OrderLineItemPricingBlocklists>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<OrderQuantityUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_discount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation_total_price_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemAppliedDiscount {
    pub discount_uid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemAppliedTax {
    pub tax_uid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub  applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemPricingBlocklists {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_discounts: Option<Vec<OrderLineItemPricingBlocklistsBlockedDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_taxes: Option<Vec<OrderLineItemPricingBlocklistsBlockedTax>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemPricingBlocklistsBlockedDiscount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemPricingBlocklistsBlockedTax {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderQuantityUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<MeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderMoneyAmounts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_charge_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderPricingOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_apply_discounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_apply_taxes: Option<bool>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub amount_money: Money,
    pub location_id: String,
    pub reason: String,
    pub status: RefundStatus,
    pub tender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amounts: Option<OrderMoneyAmounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_discounts: Option<Vec<OrderReturnDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_line_items: Option<Vec<OrderReturnLineItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_service_charges: Option<Vec<OrderReturnServiceCharge>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_taxes: Option<Vec<OrderReturnTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturnDiscount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_discount_uid: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<OrderLineItemDiscountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturnLineItem {
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_return_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<OrderLineItemItemType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<OrderQuantityUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_modifiers: Option<Vec<OrderReturnLineItemModifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_line_item_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_discount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation_total_price_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturnLineItemModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_modifier_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturnServiceCharge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculation_phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_service_charge_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReturnTax {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<OrderLineItemTaxScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_tax_uid: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub calculation_type: Option<OrderLineItemTaxType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderRoundingAdjustment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderReward {
    pub id: String,
    pub reward_tier_id: String
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderServiceCharge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculation_phase: Option<OrderServiceChargeCalculationPhase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub service_charge_type: Option<OrderServiceChargeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct OrderLineItemTax {
    pub applied_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_applied: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<OrderLineItemTaxScope>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub calculation_method_type: Option<OrderLineItemTaxType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Tender {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub tender_type: TenderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_details: Option<TenderCardDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_details: Option<TenderCashDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct TenderCardDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_method: Option<TenderCardDetailsEntryMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TenderCardDetailsStatus>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct TenderCashDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_tendered_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_back_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Checkout {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ask_for_shipping_address: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_page_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_support_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_populate_buyer_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_populate_shipping_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct PaymentLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub version: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_options: Option<CheckoutOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_populated_data: Option<PrePopulatedData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CheckoutOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_payment_methods: Option<AcceptedPaymentMethods>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_tipping: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ask_for_shipping_address: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_support_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_plan_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct AcceptedPaymentMethods {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_app_pay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<bool>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CustomField {
    pub title: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct PrePopulatedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_phone_number: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct QuickPay {
    pub location_id: String,
    pub name: String,
    pub price_money: Money,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_query: Option<CatalogQueryExact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_variations_for_item_option_values_query: Option<CatalogQueryItemVariationsForItemOptionValues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items_for_item_options_query: Option<CatalogQueryItemsForItemOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items_for_modifier_list_query: Option<CatalogQueryItemsForModifierList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items_for_tax_query: Option<CatalogQueryItemsForTax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_query: Option<CatalogQueryPrefix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_query: Option<CatalogQueryRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_query: Option<CatalogQuerySet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sorted_attribute_query: Option<CatalogQuerySortedAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_query: Option<CatalogQueryText>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryExact {
    pub attribute_name: String,
    pub attribute_value: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryItemVariationsForItemOptionValues {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_value_ids: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryItemsForItemOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_option_ids: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryItemsForModifierList {
    pub modifier_list_ids: Vec<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryItemsForTax {
    pub tax_ids: Vec<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryPrefix {
    pub attribute_name: String,
    pub attribute_prefix: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryRange {
    pub attribute_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_max_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_min_value: Option<i64>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQuerySet {
    pub attribute_name: String,
    pub attribute_values: Vec<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQuerySortedAttribute {
    pub attribute_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_attribute_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogQueryText {
    pub keywords: Vec<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogInfoResponseLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_delete_max_object_ids: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_retrieve_max_object_ids: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_upsert_max_objects_per_batch: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_upsert_max_total_objects: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_max_page_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_item_ids: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_modifier_lists_to_disable: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_modifier_lists_to_enable: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_item_ids: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_taxes_to_disable: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_taxes_to_enable: Option<i32>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct StandardUnitDescriptionGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_unit_descriptions: Option<Vec<StandardUnitDescription>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct StandardUnitDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<MeasurementUnit>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CustomAttributeFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bool_filter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    custom_attribute_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    number_filter: Option<Range>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    selection_uids_filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string_filter: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Range {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}












