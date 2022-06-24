/*!
The [maps](maps) module defines the Enums expected and returned by the
[Square API](https://developer.squareup.com). They are used for verifying input for multiple
builder functions.
 */

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