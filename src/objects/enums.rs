use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

// TODO change the implementation of existing Response Object fields to use the right enums

/// The Currency code corresponding to the amount of Money.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    GBP,
    USD,
    EUR,
    JPY,
    SGD
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CustomerCreationSource {
    OTHER,
    APPOINTMENTS,
    COUPON,
    DELETION_RECOVERY,
    DIRECTORY,
    EGIFTING,
    EMAIL_COLLECTION,
    FEEDBACK,
    IMPORT,
    INVOICES,
    LOYALTY,
    MARKETING,
    MERGE,
    ONLINE_STORE,
    INSTANT_PROFILE,
    TERMINAL,
    THIRD_PARTY,
    THIRD_PARTY_IMPORT,
    UNMERGE_RECOVERY
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CatalogObjectTypeEnum {
    ITEM,
    IMAGE,
    CATEGORY,
    ITEM_VARIATION,
    TAX,
    DISCOUNT,
    MODIFIER_LIST,
    MODIFIER,
    PRICING_RULE,
    TIME_PERIOD,
    PRODUCT_SET,
    MEASUREMENT_UNIT,
    SUBSCRIPTION_PLAN,
    ITEM_OPTION,
    ITEM_OPTION_VAL,
    CUSTOM_ATTRIBUTE_DEFINITION,
    QUICK_AMOUNTS_SETTINGS,
}

impl fmt::Display for CatalogObjectTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CatalogObjectTypeEnum::ITEM => write!(f, "ITEM"),
            CatalogObjectTypeEnum::IMAGE => write!(f, "IMAGE =>"),
            CatalogObjectTypeEnum::CATEGORY => write!(f, "CATEGORY"),
            CatalogObjectTypeEnum::ITEM_VARIATION => write!(f, "ITEM_VARIATION"),
            CatalogObjectTypeEnum::TAX => write!(f, "TAX"),
            CatalogObjectTypeEnum::DISCOUNT => write!(f, "DISCOUNT"),
            CatalogObjectTypeEnum::MODIFIER_LIST => write!(f, "MODIFIER_LIST"),
            CatalogObjectTypeEnum::MODIFIER => write!(f, "MODIFIER"),
            CatalogObjectTypeEnum::PRICING_RULE => write!(f, "PRICING_RULE"),
            CatalogObjectTypeEnum::TIME_PERIOD => write!(f, "TIME_PERIOD"),
            CatalogObjectTypeEnum::PRODUCT_SET => write!(f, "PRODUCT_SET"),
            CatalogObjectTypeEnum::MEASUREMENT_UNIT => write!(f, "MEASUREMENT_UNIT"),
            CatalogObjectTypeEnum::SUBSCRIPTION_PLAN => write!(f, "SUBSCRIPTION_PLAN"),
            CatalogObjectTypeEnum::ITEM_OPTION => write!(f, "ITEM_OPTION"),
            CatalogObjectTypeEnum::ITEM_OPTION_VAL => write!(f, "ITEM_OPTION_VAL"),
            CatalogObjectTypeEnum::CUSTOM_ATTRIBUTE_DEFINITION
            => write!(f, "CUSTOM_ATTRIBUTE_DEFINITION"),
            CatalogObjectTypeEnum::QUICK_AMOUNTS_SETTINGS => write!(f, "QUICK_AMOUNTS_SETTINGS"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BusinessAppointmentSettingsBookingLocationType {
    BUSINESS_LOCATION,
    CUSTOMER_LOCATION,
    PHONE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BookingBookingSource {
    FIRST_PARTY_MERCHANT,
    FIRST_PARTY_BUYER,
    THIRD_PARTY_BUYER,
    API,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BookingStatus {
    PENDING,
    CANCELLED_BY_CUSTOMER,
    CANCELLED_BY_SELLER,
    DECLINED,
    ACCEPTED,
    NO_SHOW,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocationStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaxIds {
    es_nif,
    eu_vat,
    fr_naf,
    fr_siret,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocationType {
    PHYSICAL,
    MOBILE
}