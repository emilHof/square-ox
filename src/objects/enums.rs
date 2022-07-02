use std::fmt;
use std::fmt::{Formatter, write};
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

/// The CustomerCreationSource type, indicating how the customer was created.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerCreationSource {
    Other,
    Appointments,
    Coupon,
    DeletionRecovery,
    Directory,
    Egifting,
    EmailCollection,
    Feedback,
    Import,
    Invoices,
    Loyalty,
    Marketing,
    Merge,
    OnlineStore,
    InstantProfile,
    Terminal,
    ThirdParty,
    ThirdPartyImport,
    UnmergeRecovery
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogObjectTypeEnum {
    Item,
    Image,
    Category,
    ItemVariation,
    Tax,
    Discount,
    ModifierList,
    Modifier,
    PricingRule,
    TimePeriod,
    ProductSet,
    MeasurementUnit,
    SubscriptionPlan,
    ItemOption,
    ItemOptionVal,
    CustomAttributeDefinition,
    QuickAmountsSettings,
}

impl fmt::Display for CatalogObjectTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CatalogObjectTypeEnum::Item => write!(f, "ITEM"),
            CatalogObjectTypeEnum::Image => write!(f, "IMAGE =>"),
            CatalogObjectTypeEnum::Category => write!(f, "CATEGORY"),
            CatalogObjectTypeEnum::ItemVariation => write!(f, "ItemVariation"),
            CatalogObjectTypeEnum::Tax => write!(f, "TAX"),
            CatalogObjectTypeEnum::Discount => write!(f, "DISCOUNT"),
            CatalogObjectTypeEnum::ModifierList => write!(f, "MODIFIER_LIST"),
            CatalogObjectTypeEnum::Modifier => write!(f, "MODIFIER"),
            CatalogObjectTypeEnum::PricingRule => write!(f, "PRICING_RULE"),
            CatalogObjectTypeEnum::TimePeriod => write!(f, "TIME_PERIOD"),
            CatalogObjectTypeEnum::ProductSet => write!(f, "PRODUCT_SET"),
            CatalogObjectTypeEnum::MeasurementUnit => write!(f, "MeasurementUnit"),
            CatalogObjectTypeEnum::SubscriptionPlan => write!(f, "SubscriptionPlan"),
            CatalogObjectTypeEnum::ItemOption => write!(f, "ItemOption"),
            CatalogObjectTypeEnum::ItemOptionVal => write!(f, "ItemOptionVal"),
            CatalogObjectTypeEnum::CustomAttributeDefinition
            => write!(f, "CustomAttributeDefinition"),
            CatalogObjectTypeEnum::QuickAmountsSettings => write!(f, "QuickAmountsSettings"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsBookingLocationType {
    BusinessLocation,
    CustomerLocation,
    Phone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingBookingSource {
    FirstPartyMerchant,
    FirstPartyBuyer,
    ThirdPartyBuyer,
    Api,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingStatus {
    Pending,
    CancelledByCustomer,
    CancelledBySeller,
    Declined,
    Accepted,
    NoShow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocationStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TaxIds {
    EsNif,
    EuVat,
    FrNaf,
    FrSiret,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationType {
    Physical,
    Mobile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    Desc,
    Asc,
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileBookingPolicy {
    AcceptAll,
    RequiresAcceptance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsCancellationPolicy {
    CancellationTreatedAsNoShow,
    CustomPolicy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType {
    PerTeamMember,
    PerLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileCustomerTimezoneChoice {
    BusinessLocationTimezone,
    CustomerChoice,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemDiscountScope {
    OtherDiscountScope,
    LineItem,
    Order,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemDiscountType {
    UnknownDiscount,
    FixedPercentage,
    FixedAmount,
    VariablePercentage,
    VariableAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentFulfillmentLineItemApplication {
    All,
    EntryList
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentPickupDetailsScheduleType {
    Scheduled,
    Asap
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemItemType {
    Item,
    CustomAmount,
    GiftCard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemTaxScope {
    OtherTaxScope,
    LineItem,
    Order
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemTaxType {
    UnknownTax,
    Additive,
    Inclusive,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    Open,
    Completed,
    Canceled,
    Draft
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeCalculationPhase {
    SubtotalPhase,
    TotalPhase,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeType {
    AutoGratuity,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderType {
    Card,
    Cash,
    ThirdPartyCard,
    SquareGiftCard,
    NoSale,
    Wallet,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderCardDetailsStatus {
    Authorized,
    Captured,
    Voided,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderCardDetailsEntryMethod {
    Swiped,
    Keyed,
    Emv,
    OnFile,
    Contactless
}


