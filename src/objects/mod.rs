/*!
A non-comprehensive list of the Objects used by the
[Square API](https://developer.squareup.com).
*/

pub mod enums;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use square_ox_derive::{Builder};
use crate::api::orders::Orders;
use crate::api::terminal::Terminal;
use crate::builder::{AddField, Buildable};
use crate::objects::enums::{
    ActionCancelReason, ApplicationDetailsExternalSquareProduct,
    BankAccountOwnershipType, BusinessAppointmentSettingsBookingLocationType,
    BusinessAppointmentSettingsCancellationPolicy,
    BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType,
    BusinessBookingProfileBookingPolicy, BusinessBookingProfileCustomerTimezoneChoice,
    BuyNowPayLaterBrand, CatalogCustomAttributeDefinitionType, CatalogItemProductType,
    CatalogObjectType, CatalogPricingType, CCVStatus, CheckoutOptionsPaymentType, Currency,
    CustomerCreationSource, DigitalWalletBrand, DigitalWalletStatus, InventoryAlertType,
    InventoryChangeType, InventoryState, LocationStatus, LocationType,
    OrderFulfillmentFulfillmentLineItemApplication, OrderFulfillmentPickupDetailsScheduleType,
    OrderFulfillmentState, OrderFulfillmentType, OrderLineItemDiscountScope,
    OrderLineItemDiscountType, OrderLineItemItemType, OrderLineItemTaxScope,
    OrderLineItemTaxType, OrderServiceChargeCalculationPhase, OrderServiceChargeType,
    OrderState, PaymentSourceType, PaymentStatus, PaymentType, PaymentVerificationMethod,
    PaymentVerificationResults, ProcessingFeeType, RefundStatus, RiskEvaluationRiskLevel,
    SearchOrdersSortField, SortOrder, TenderCardDetailsEntryMethod, TenderCardDetailsStatus,
    TenderType, TerminalCheckoutStatus
};
use crate::response::ResponseError;

/// The Response enum holds the variety of responses that can be returned from a
/// [Square API](https://developer.squareup.com) call.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Response {
    // Payments Endpoint Responses
    Payment(Payment),

    // Orders Endpoint Responses
    Order(Order),
    Orders(Vec<Order>),
    OrderEntries(Vec<OrderEntry>),

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
    Checkout(CheckoutEnum), // Also a possible Terminals Endpoint Response
    PaymentLinks(Vec<PaymentLink>),
    PaymentLink(PaymentLink),

    // Inventory Endpoint Responses
    Counts(Vec<InventoryCount>),

    // Sites Endpoint Responses
    Sites(Vec<Site>),

    // Terminal Endpoint Responses
    Checkouts(Vec<TerminalCheckout>),
}

// Since both the Checkout and Terminal endpoint can return a field tagged with checkout it is
// necessary to define this return field as an untagged enum
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckoutEnum {
    Checkout(Checkout),
    TerminalCheckout(TerminalCheckout),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_vis("private")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[builder_vis("private")]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub struct AppointmentSegment {
    pub duration_minutes: f64,
    #[builder_into]
    pub team_member_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub any_team_member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intermission_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub resource_ids: Option<String>,
    #[builder_into]
    pub service_variation_id: String,
    pub service_variation_version: i64,
}

#[cfg(test)]
mod test_appointment_segment {
    use crate::builder::Builder;
    use super::*;

    #[tokio::test]
    async fn test_derive() {
        let expected = AppointmentSegment {
            duration_minutes: 30.0,
            team_member_id: "some_id".to_string(),
            any_team_member_id: None,
            intermission_minutes: None,
            resource_ids: None,
            service_variation_id: "some_id".to_string(),
            service_variation_version: 1232941981
        };

        let actual = Builder::from(AppointmentSegment::default())
            .duration_minutes(30.0)
            .team_member_id("some_id")
            .service_variation_id("some_id")
            .service_variation_version(1232941981)
            .build()
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }
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
#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub struct Card {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_vis("private")]
    #[builder_into]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub bin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub card_brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub card_co_brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub card_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub cardholder_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[builder_into]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub last_4: Option<String>,
    #[builder_into]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[builder_into]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_type: Option<String>,
    #[builder_into]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[cfg(test)]
mod test_card_builder {
    use crate::builder::Builder;
    use super::*;

    #[tokio::test]
    async fn test_card_builder() {
        let expected = Card {
            id: None,
            billing_address: Some(Address {
                address_line_1: Some("some line".to_string()),
                address_line_2: None,
                address_line_3: None,
                locality: None,
                sublocality: None,
                administrative_district_level: None,
                postal_code: None,
                country: None
            }),
            bin: None,
            card_brand: None,
            card_co_brand: None,
            card_type: None,
            cardholder_name: None,
            customer_id: None,
            enabled: Some(true),
            exp_month: None,
            exp_year: None,
            fingerprint: None,
            last_4: None,
            merchant_id: None,
            prepaid_type: None,
            reference_id: None,
            version: None
        };

        let actual = Builder::from(Card::default())
            .billing_address(Address {
                address_line_1: Some("some line".to_string()),
                address_line_2: None,
                address_line_3: None,
                locality: None,
                sublocality: None,
                administrative_district_level: None,
                postal_code: None,
                country: None
            })
            .enabled(true)
            .build()
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }
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

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
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
    pub item_variation_data: Option<CatalogItemVariation>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct Booking {
    #[builder_vis("private")]
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
    #[builder_into]
    #[builder_validate("is_some")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub customer_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    #[builder_validate("is_some")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<BusinessAppointmentSettingsBookingLocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub seller_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub start_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_time_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>
}

impl AddField<AppointmentSegment> for Booking {
    fn add_field(&mut self, field: AppointmentSegment) {
        if let Some(segments) = self.appointment_segments.as_mut() {
            segments.push(field);
        } else {
            self.appointment_segments = Some(vec![field])
        }
    }
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

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct CreateOrderRequest {
    #[builder_rand("uuid")]
    pub idempotency_key: Option<String>,
    pub order: Order,
}

impl AddField<Order> for CreateOrderRequest {
    fn add_field(&mut self, field: Order) {
        self.order = field;
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct Order {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    #[builder_validate("is_some")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub close_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<OrderLineItemDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<OrderFulfillment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<OrderLineItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_amounts: Option<OrderMoneyAmounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_options: Option<OrderPricingOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<Refund>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amounts: Option<OrderMoneyAmounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub returns: Option<Vec<OrderReturn>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<OrderReward>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_charges: Option<Vec<OrderServiceCharge>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<OrderSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<OrderLineItemTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenders: Option<Vec<Tender>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub ticket_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_discount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_service_charge_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tip_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl AddField<OrderLineItem> for Order {
    fn add_field(&mut self, field: OrderLineItem) {
        if let Some(line_items) = self.line_items.as_mut() {
            line_items.push(field);
        } else {
            self.line_items = Some(vec![field]);
        }
    }
}

impl AddField<OrderServiceCharge> for Order {
    fn add_field(&mut self, field: OrderServiceCharge) {
        if let Some(line_items) = self.service_charges.as_mut() {
            line_items.push(field);
        } else {
            self.service_charges = Some(vec![field]);
        }
    }
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

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct OrderLineItem {
    #[builder_into]
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_reference_ids: Option<HashMap<String, String>>,
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

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
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
    #[builder_into]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    #[builder_validate("is_some")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
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

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct PaymentLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    #[builder_vis("private")]
    pub id: Option<String>,
    pub version: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_options: Option<CheckoutOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub payment_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_populated_data: Option<PrePopulatedData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_into]
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

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct InventoryCount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<InventoryState>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct InventoryChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<InventoryAdjustment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<CatalogMeasurementUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_count: Option<InventoryPhysicalCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer: Option<InventoryTransfer>,
    #[serde(rename = "type")]
    pub inventory_change_type: InventoryChangeType,
}

impl AddField<InventoryPhysicalCount> for InventoryChange {
    fn add_field(&mut self, field: InventoryPhysicalCount) {
        self.physical_count = Some(field)
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct InventoryAdjustment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustment_group: Option<InventoryAdjustmentGroup>,
    pub catalog_object_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_state: Option<InventoryState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_receipt_id: Option<String>,
    pub location_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceApplication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_state: Option<InventoryState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_price_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct InventoryAdjustmentGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_state: Option<InventoryState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_adjustment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_state: Option<InventoryState>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default, Builder)]
pub struct InventoryPhysicalCount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder_vis("private")]
    pub id: Option<String>,
    pub catalog_object_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<CatalogObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub location_id: String,
    pub occurred_at: String,
    pub quantity: String, /// As decimal with up to 5 digits after the decimal point
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceApplication>,
    pub state: InventoryState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct InventoryTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub catalog_object_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    pub from_location_id: String,
    pub occurred_at: String,
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceApplication>,
    pub state: InventoryState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    pub to_location_id: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct Payment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_details: Option<ApplicationDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<BankAccountPaymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_now_pay_later_details: Option<BuyNowPayLaterDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_details: Option<CardPaymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_details: Option<CashPaymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delayed_until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_details: Option<DeviceDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_details: Option<ExternalPaymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_fee: Option<ProcessingFee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunded_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<RiskEvaluation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PaymentSourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_description_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_details: Option<DigitalWalletDetails>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CashPaymentDetails {
    pub buyer_supplied_money: Money,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_back_money: Option<Money>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ExternalPaymentDetails {
    pub source: String,
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ApplicationDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub square_product: Option<ApplicationDetailsExternalSquareProduct>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct BankAccountPaymentDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ownership_type: Option<BankAccountOwnershipType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_details: Option<ACHDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ACHDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<BankAccountOwnershipType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct BuyNowPayLaterDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_details: Option<AfterpayDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<BuyNowPayLaterBrand>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct AfterpayDetails {
    pub email_address: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CardPaymentDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_result_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avs_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_payment_timeline: Option<CardPaymentTimeline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvv_status: Option<CCVStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_method: Option<TenderCardDetailsEntryMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_requires_card_presence: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<PaymentVerificationMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_results: Option<PaymentVerificationResults>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CardPaymentTimeline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct DeviceDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_installation_id: Option<String>,
    pub device_name: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct ProcessingFee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<ProcessingFeeType>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct RiskEvaluation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<RiskEvaluationRiskLevel>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct DigitalWalletDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    brand: Option<DigitalWalletBrand>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cash_app_details: Option<CashPaymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    status: Option<DigitalWalletStatus>
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CashAppDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_cashtag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_full_name: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Site {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct TerminalCheckout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub amount_money: Option<Money>,
    pub device_options: Option<DeviceCheckoutOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_fee_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<ActionCancelReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<PaymentOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<CheckoutOptionsPaymentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TerminalCheckoutStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct DeviceCheckoutOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collect_signature: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_itemized_cart: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_receipt_screen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_settings: Option<TipSettings>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct TipSettings {
    /// Indicates whether tipping is enabled for this checkout. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_tipping: Option<bool>,
    /// Indicates whether custom tip amounts are allowed during the checkout flow. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_tip_field: Option<bool>,
    /// Indicates whether tip options should be presented on the screen before presenting the
    /// signature screen during card payment. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separate_tip_screen: Option<bool>,
    /// Enables the "Smart Tip Amounts" behavior. Exact tipping options depend on the region in which
    /// the Square seller is active. <br/>
    /// For payments under 10.00, in the Australia, Canada, Ireland, United Kingdom, and United
    /// States, tipping options are presented as no tip, .50, 1.00 or 2.00. <br/>
    /// For payment amounts of 10.00 or greater, tipping options are presented as the following
    /// percentages: 0%, 5%, 10%, 15%.
    /// If set to true, the `tip_percentages` settings is ignored. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smart_tipping: Option<bool>,
    /// A list of tip percentages that should be presented during the checkout flow, specified as up
    /// to 3 non-negative integers from 0 to 100 (inclusive). Defaults to 15, 20, and 25.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_percentages: Option<Vec<i32>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct PaymentOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept_partial_authorization: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_duration: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TerminalCheckoutQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<TerminalCheckoutQueryFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<TerminalCheckoutQuerySort>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalCheckoutQueryFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TerminalCheckoutStatus>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalCheckoutQuerySort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TerminalRefund {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<ActionCancelReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TerminalCheckoutStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TerminalRefundQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<TerminalRefundQueryFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<TerminalCheckoutQuerySort>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TerminalRefundQueryFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TerminalCheckoutStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersQuery {
    pub filter: Option<SearchOrdersFilter>,
    pub sort: Option<SearchOrdersSort>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_filter: Option<SearchOrdersCustomerFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_filter: Option<SearchOrdersDateTimeFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_filter: Option<SearchOrdersFulfillmentFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_filter: Option<SearchOrdersSourceFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<SearchOrdersStateFilter>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersCustomerFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersDateTimeFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TimeRange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersFulfillmentFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_states: Option<Vec<OrderFulfillmentState>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_types: Option<Vec<OrderFulfillmentType>>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersSourceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_names: Option<Vec<String>>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersStateFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<OrderState>>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrdersSort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<SearchOrdersSortField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OrderEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchQueryAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CustomerFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CustomerSort>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CustomerFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_source: Option<CreationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<FilterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TimeRange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CustomerSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimeRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CustomerTextFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreationSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<CustomerCreationSource>>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct StartAtRange {
    pub end_at: String,
    pub start_at: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SegmentFilter {
    pub service_variation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id_filter: Option<FilterValue>,
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct AvailabilityQueryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at_range: Option<StartAtRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_filters: Option<Vec<SegmentFilter>>
}
