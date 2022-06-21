use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SquareResponse {
    #[serde(flatten)]
    pub response: crate::response::jsons::Response,
    #[serde(default)]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub id_mapping: Option<Vec<(String, String)>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub category: String,
    pub code: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub field: Option<String>,
}

pub mod enums {
    use std::collections::HashSet;

    pub struct CatalogObjectTypeEnum(HashSet<String>);

    impl CatalogObjectTypeEnum {
        pub fn validate(object_type: &String) -> bool {
            let are_valid = CatalogObjectTypeEnum(vec![
                "ITEM".to_string(), "IMAGE".to_string(), "CATEGORY".to_string(),
                "ITEM_VARIATION".to_string(), "TAX".to_string(), "DISCOUNT".to_string(),
                "MODIFIER_LIST".to_string(), "MODIFIER".to_string(), "PRICING_RULE".to_string(),
                "TIME_PERIOD".to_string(), "PRODUCT_SET".to_string(), "MEASUREMENT_UNIT".to_string(),
                "SUBSCRIPTION_PLAN".to_string(), "ITEM_OPTION".to_string(),
                "ITEM_OPTION_VAL".to_string(),"CUSTOM_ATTRIBUTE_DEFINITION".to_string(),
                "QUICK_AMOUNTS_SETTINGS".to_string(),
            ].into_iter().collect());

            are_valid.check_validity(object_type)
        }

        fn check_validity(self, object_type: &String) -> bool {
            self.0.contains(object_type)
        }
    }

    pub struct CustomerCreationSource(HashSet<String>);

    impl CustomerCreationSource {
        pub fn validate(object_type: &String) -> bool {
            let are_valid = CustomerCreationSource(vec![
                "OTHER".to_string(), "APPOINTMENTS".to_string(), "COUPON".to_string(),
                "DELETION_RECOVERY".to_string(), "DIRECTORY".to_string(), "EGIFTING".to_string(),
                "EMAIL_COLLECTION".to_string(), "FEEDBACK".to_string(), "MPORT".to_string(),
                "INVOICES".to_string(), "LOYALTY".to_string(), "MARKETING".to_string(),
                "MERGE".to_string(), "ONLINE_STORE".to_string(), "INSTANT_PROFILE".to_string(),
                "TERMINAL".to_string(), "THIRD_PARTY".to_string(), "THIRD_PARTY_IMPORT".to_string(),
                "UNMERGE_RECOVERY".to_string(),
            ].into_iter().collect());

            are_valid.check_validity(object_type)
        }

        fn check_validity(self, object_type: &String) -> bool {
            self.0.contains(object_type)
        }
    }

    pub struct BusinessAppointmentSettingsBookingLocationType(HashSet<String>);

    impl BusinessAppointmentSettingsBookingLocationType {
        pub fn validate(object_type: &String) -> bool {
            let are_valid = CustomerCreationSource(vec![
                "BUSINESS_LOCATION".to_string(),
                "CUSTOMER_LOCATION".to_string(),
                "PHONE".to_string(),
            ].into_iter().collect());

            are_valid.check_validity(object_type)
        }

        fn check_validity(self, object_type: &String) -> bool {
            self.0.contains(object_type)
        }
    }

    pub struct BookingBookingSource(HashSet<String>);

    impl BookingBookingSource {
        pub fn validate(object_type: &String) -> bool {
            let are_valid = CustomerCreationSource(vec![
                "FIRST_PARTY_MERCHANT".to_string(),
                "FIRST_PARTY_BUYER".to_string(),
                "THIRD_PARTY_BUYER".to_string(),
                "API".to_string(),
            ].into_iter().collect());

            are_valid.check_validity(object_type)
        }

        fn check_validity(self, object_type: &String) -> bool {
            self.0.contains(object_type)
        }
    }

    pub struct BookingStatus(HashSet<String>);

    impl BookingStatus {
        pub fn validate(object_type: &String) -> bool {
            let are_valid = CustomerCreationSource(vec![
                "PENDING".to_string(), "CANCELLED_BY_CUSTOMER".to_string(),
                "CANCELLED_BY_SELLER".to_string(), "DECLINED".to_string(),
                "ACCEPTED".to_string(), "NO_SHOW".to_string(),
            ].into_iter().collect());

            are_valid.check_validity(object_type)
        }

        fn check_validity(self, object_type: &String) -> bool {
            self.0.contains(object_type)
        }
    }
}

pub mod jsons {
    use serde::{Deserialize, Serialize};
    use crate::money::Money;

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
        Objects(Vec<CatalogObject>),
        Booking(Booking),
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

    #[derive(Clone, Serialize, Debug, Deserialize)]
    pub struct CatalogObject {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub absent_at_location_ids: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category_data: Option<CatalogCategory>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attributes_values: Option<std::collections::HashMap<String, CatalogCustomAttributeValue>>,
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
        pub type_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub absent_at_location_ids: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category_data: Option<CatalogCategory>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attributes_values: Option<std::collections::HashMap<String, CatalogCustomAttributeValue>>,
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
        pub type_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub absent_at_location_ids: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub catalog_v1_ids: Option<Vec<CatalogV1ID>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category_data: Option<CatalogCategory>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_attributes_values: Option<std::collections::HashMap<String, CatalogCustomAttributeValue>>,
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
        pub type_name: Option<String>,
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
        pub type_name: Option<String>,
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
        abbreviation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        available_electronically: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        available_for_pickup: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        available_online: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        category_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image_ids : Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image_option: Option<Vec<CatalogItemOptionForItem>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label_color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(rename="type", default, skip_serializing_if = "Option::is_none")]
        product_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        skip_modifier_scree: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sort_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tax_ids: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variations: Option<Vec<CatalogObjectVariation>>,
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
        pub inventory_alert_type: Option<String>,
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
        pub pricing_type: Option<String>,
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
        pub inventory_alert_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub price_money: Option<Money>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pricing_type: Option<String>,
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
        pub location_type: Option<String>,
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
}