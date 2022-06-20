use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use crate::money::Money;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SquareResponse {
    #[serde(flatten)]
    pub response: Response,
    #[serde(default)]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub id_mapping: Option<Vec<(String, String)>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    Locations(Vec<Location>),
    Availabilities(Vec<Availability>),
    Customer(Customer),
    Customers(Vec<Customer>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub category: String,
    pub code: String,
    pub detail: Option<String>,
    pub field: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Location {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub address: Option<Address>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub capabilities: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_id: Option<String>,
    #[serde(default)]
    pub coordinates: Option<Coordinates>,
    #[serde(default)]
    pub country: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub facebook_url: Option<String>,
    #[serde(default)]
    pub full_format_logo_url: Option<String>,
    #[serde(default)]
    pub instagram_username: Option<String>,
    #[serde(default)]
    pub language_code: Option<String>,
    #[serde(default)]
    pub mcc: Option<String>,
    #[serde(default)]
    pub merchant_id: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub pos_background_url: Option<String>,
    #[serde(default)]
    pub tax_ids: Option<String>,
    #[serde(default)]
    pub twitter_username: Option<String>,
    #[serde(rename = "type", default)]
    pub type_name: Option<String>,
    #[serde(default)]
    pub business_hours: Option<Periods>,
    #[serde(default)]
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
pub struct Periods {
    pub periods: Vec<Period>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Period {
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
    pub service_variation_id: String,
    pub service_variation_version: f64,
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
    pub eu_vat: String
}

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
    all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    none: Option<Vec<String>>,
}

// TODO add default and if_none tags
#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CatalogObject {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type", default)]
    pub type_name: Option<String>,
    pub absent_at_location_ids: Option<Vec<String>>,
    pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
    pub category_data: Option<CatalogCategory>,
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    pub custom_attributes_values: Option<serde_json::Map<String, CatalogCustomAttributeValue>>,
    pub discount_data: Option<CatalogDiscount>,
    pub image_data: Option<CatalogImage>,
    pub is_deleted: Option<bool>,
    pub item_data: Option<CatalogItem>,
    pub item_option_data: Option<CatalogItemOption>,
    pub item_option_value_data: Option<CatalogItemOptionValue>,
    pub item_variation_data: Option<CatalogItemVariation>,
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    pub modifier_data: Option<CatalogModifier>,
    pub modifier_list_data: Option<CatalogModifierList>,
    pub present_at_all_locations: Option<bool>,
    pub present_at_location_ids: Option<Vec<String>>,
    pub pricing_rule_data: Option<CatalogPricingRule>,
    pub product_set_data: Option<CatalogProductSet>,
    pub quick_amount_settings_data: Option<CatalogQuickAmountsSettings>,
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    pub tax_data: Option<CatalogTax>,
    pub time_period_data: Option<CatalogTimePeriod>,
    pub updated_at: Option<String>,
    pub version: Option<i64>,
}

pub struct CatalogV1ID {
    pub catalog_v1_id: Option<String>,
    pub location_id: Option<String>,
}

pub struct CatalogCategory {
    pub image_ids: Option<Vec<String>>,
    pub name: Option<String>,
}

pub struct CatalogCustomAttributeDefinition {
    allowed_object_types: Option<String>,
    name: Option<String>,
    #[serde(rename = "type", default)]
    pub type_name: Option<String>,
    app_visibility: Option<String>,
    custom_attribute_usage_count: Option<i32>,
    description: Option<String>,
    key: Option<String>,
    number_config: Option<CatalogCustomAttributeDefinitionNumberConfig>,
    selection_config: Option<CatalogCustomAttributeDefinitionSelectionConfig>,
    seller_visibility: Option<String>,
    source_application: Option<SourceApplication>,
    string_config: Option<CatalogCustomAttributeDefinitionStringConfig>,

}

pub struct CatalogCustomAttributeDefinitionNumberConfig {
    pub precision: Option<i32>
}

pub struct CatalogCustomAttributeDefinitionSelectionConfig {
    pub allowed_selections: Option<Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection>>,
    pub max_allowed_selections: Option<i32>
}

pub struct CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection {
    pub name: Option<String>,
    pub uid: Option<String>,
}

pub struct SourceApplication {
    pub application_id: Option<String>,
    pub name: Option<String>,
    pub product: Option<String>,
}

pub struct CatalogCustomAttributeDefinitionStringConfig {
    pub enforce_uniqueness: Option<bool>
}

pub struct CatalogCustomAttributeValue {
    pub boolean_value: Option<bool>,
    pub custom_attribute_definition_id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub number_value: Option<String>,
    pub selection_uid_values: Option<Vec<String>>,
    pub string_value: Option<String>,
    #[serde(rename="type", default)]
    pub type_name: Option<String>,
}

pub struct CatalogDiscount {
    pub amount_money: Option<Money>,
    pub discount_type: Option<String>,
    pub label_color: Option<String>,
    pub maximum_amount_money: Option<Money>,
    pub modify_tax_basis: Option<String>,
    pub name: Option<String>,
    pub percentage: Option<String>,
    pub pin_required: Option<String>,
}

pub struct CatalogImage {
    pub caption: Option<String>,
    pub name: Option<String>,
    pub photo_studio_order_id: Option<String>,
    pub url: Option<String>,
}

pub struct CatalogItem {
    abbreviation: Option<String>,
    available_electronically: Option<bool>,
    available_for_pickup: Option<bool>,
    available_online: Option<bool>,
    category_id: Option<String>,
    description: Option<String>,
    image_ids : Option<Vec<String>>,
    image_option: Option<Vec<CatalogItemOptionForItem>>,
    label_color: Option<String>,
    modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    name: Option<String>,
    product_type: Option<String>,
    skip_modifier_scree: Option<bool>,
    sort_name: Option<String>,
    tax_ids: Option<Vec<String>>,
    variations: Option<CatalogObject>,
}

pub struct CatalogItemOptionForItem {
    pub item_option_id: Option<String>,
}

pub struct CatalogItemModifierListInfo {
    pub modifier_list_id: Option<String>,
    pub enabled: Option<String>,
    pub max_selected_modifiers: Option<String>,
    pub min_selected_modifiers: Option<String>,
    pub modifier_overrides: Option<String>,
}

pub struct CatalogModifierOverride {
    pub modifier_id: Option<String>,
    pub on_by_default: Option<String>,
}

pub struct CatalogItemOption {
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub show_colors: Option<bool>,
    pub values: Option<CatalogObject>,
}

pub struct CatalogItemOptionValue {
    pub color: Option<String>,
    pub description: Option<String>,
    pub item_option_id: Option<String>,
    pub name: Option<String>,
    pub ordinal: Option<i32>,
}

pub struct CatalogItemVariation {
    pub available_for_booking: Option<bool>,
    pub image_ids: Option<Vec<String>>,
    pub inventory_alert_threshold: Option<i64>,
    pub inventory_alert_type: Option<String>,
    pub item_id: Option<String>,
    pub item_option_values: Option<Vec<CatalogItemOptionValueForItemVariation>>,
    pub location_overrides: Option<Vec<ItemVariationLocationOverrides>>,
    pub measurement_unit_id: Option<String>,
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub price_money: Option<Money>,
    pub pricing_type: Option<String>,
    pub sellable: Option<bool>,
    pub service_duration: Option<i64>,
    pub sku: Option<String>,
    pub stockable: Option<bool>,
    pub stockable_conversion: Option<CatalogStockConversion>,
    pub team_member_ids: Option<Vec<String>>,
    pub track_inventory: Option<bool>,
    pub  upc: Option<String>,
    pub user_data: Option<String>,
}

pub struct CatalogItemOptionValueForItemVariation {
    pub item_option_id: Option<String>,
    pub item_option_value_id: Option<String>,
}

pub struct ItemVariationLocationOverrides {
    pub inventory_alert_threshold: Option<i64>,
    pub inventory_alert_type: Option<String>,
    pub location_id: Option<String>,
    pub price_money: Option<Money>,
    pub pricing_type: Option<String>,
    pub sold_out: Option<bool>,
    pub sold_out_valid_until: Option<String>,
    pub track_inventory: Option<bool>,
}

pub struct CatalogStockConversion {
    pub nonstockable_quantity: Option<String>,
    pub stockable_item_variation_id: Option<String>,
    pub stockable_quantity: Option<String>,
}

pub struct CatalogMeasurementUnit {
    pub measurement_unit: Option<MeasurementUnit>,
    pub precision: Option<i32>,
}

pub struct MeasurementUnit {
    pub area_unit: Option<String>,
    pub custom_unit: Option<MeasurementUnitCustom>,
    pub generic_unit: Option<String>,
    pub length_unit: Option<String>,
    pub time_unit: Option<String>,
    #[serde(rename= "type")]
    pub type_name: Option<String>,
    pub volume_unit: Option<String>,
    pub weight_unit: Option<String>,
}

pub struct MeasurementUnitCustom {
    pub abbreviation: Option<String>,
    pub name: Option<String>,
}

pub struct CatalogModifier {
    pub image_ids: Option<Vec<String>>,
    pub modifier_list_id: Option<String>,
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub price_money: Option<Money>,
}

pub struct CatalogModifierList {
    pub image_ids: Option<Vec<String>>,
    pub modifiers: Option<CatalogObject>,
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub selection_type: Option<string>,
}

pub struct CatalogPricingRule {
    pub customer_group_ids_any: Option<Vec<String>>,
    pub discount_id: Option<String>,
    pub exclude_products_id: Option<String>,
    pub exclude_strategy: Option<String>,
    pub match_products_id: Option<String>,
    pub minimum_order_subtotal_money: Option<Money>,
    pub name: Option<String>,
    pub time_period_ids: Option<String>,
    pub valid_from_date: Option<String>,
    pub valid_from_local_time: Option<Vec<String>>,
    pub valid_until_date: Option<String>,
    pub valid_until_local_time: Option<String>,
    pub apply_products_id: Option<String>,
}

pub struct CatalogProductSet {
    pub all_products: Option<bool>,
    pub name: Option<String>,
    pub product_ids_all: Option<Vec<String>>,
    pub product_ids_any: Option<Vec<String>>,
    pub quantity_exact: Option<i64>,
    pub quantity_max: Option<i64>,
    pub quantity_min: Option<i64>,
}

pub struct CatalogQuickAmountsSettings {
    pub option: Option<String>,
    pub amounts: Option<Vec<CatalogQuickAmount>>,
    pub eligible_for_auto_amounts: Option<bool>,
}

pub struct CatalogQuickAmount {
    pub amount: Option<Money>,
    #[serde(rename="type")]
    pub type_name: Option<String>,
    pub ordinal: Option<i64>,
    pub score: Option<i64>,
}

pub struct CatalogSubscriptionPlan {
    pub name: Option<String>,
    pub phases: Option<SubscriptionPhase>
}

pub struct SubscriptionPhase {
    pub cadence: Option<String>,
    pub ordinal: Option<i64>,
    pub periods: Option<i32>,
    pub recurring_price_money: Option<Money>,
    pub uid: Option<String>,
}

pub struct CatalogTax {
    pub applies_to_custom_amounts: Option<bool>,
    pub calculation_phase: Option<String>,
    pub enabled: Option<bool>,
    pub inclusion_type: Option<String>,
    pub name: Option<String>,
    pub percentage: Option<String>,
}

pub struct CatalogTimePeriod {
    pub event: Option<String>,
}