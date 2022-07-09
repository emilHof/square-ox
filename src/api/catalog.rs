/*!
Catalog functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{ObjectUpsertRequestBuildError, SquareError};
use crate::response::SquareResponse;
use crate::objects::{CatalogItem, CatalogObject, CatalogObjectVariation, CatalogQuery, CustomAttributeFilter, enums::CatalogObjectTypeEnum};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::objects::enums::{CatalogItemProductType, CatalogObjectType, SearchCatalogItemsRequestStockLevel, SortOrder};

impl SquareClient {
    /// Returns a list of all [CatalogObjects](CatalogObjects) of the specified types in the catalog.
    pub async fn list_catalog(&self, list_parameters: Option<Vec<(String, String)>>)
                              -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Catalog("/list".to_string()),
            None::<&CatalogObject>,
            list_parameters
        ).await
    }

    /// Creates or updates the target [CatalogObject](CatalogObject).
    pub async fn upsert_catalog_object(&self, object: ObjectUpsertRequest)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::POST,
            SquareAPI::Catalog("/object".to_string()),
            Some(&object),
            None,
        ).await
    }

    /// Deletes a single CatalogObject based on the provided ID and returns the set of successfully
    /// deleted IDs in the response.
    pub async fn delete_catalog_object(&self, object_id: String)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::DELETE,
            SquareAPI::Catalog(format!("/object/{}", object_id)),
            None::<&ObjectUpsertRequest>,
            None,
        ).await
    }

    /// Returns a single [CatalogItem](CatalogItem) as a [CatalogObject](CatalogObject) based on the
    /// provided ID.
    pub async fn retrieve_catalog_object(
        &self,
        object_id: String,
        parameters: Option<Vec<(String, String)>>
    )
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Catalog(format!("/object/{}", object_id)),
            None::<&ObjectUpsertRequest>,
            parameters,
        ).await
    }

    /// Searches for [CatalogObject](CatalogObject) of any type by matching supported search attribute values,
    /// excluding custom attribute values on items or item variations, against one or more of
    /// the specified query filters.
    pub async fn search_catalog_objects(&self, search_body: SearchCatalogObjectsBody)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::POST,
            SquareAPI::Catalog("/search".to_string()),
            Some(&search_body),
            None,
        ).await
    }

    /// Retrieves information about the [Square Catalog API](https://developer.squareup.com), such
    /// as batch size limits that can be used by the `BatchUpsertCatalogObjects` endpoint.
    pub async fn catalog_info(&self)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Catalog("/info".to_string()),
            None::<&SearchCatalogObjectsBody>,
            None,
        ).await
    }

    // TODO implement search_catalog_items
    /// Retrieves information about the [Square Catalog API](https://developer.squareup.com), such
    /// as batch size limits that can be used by the `BatchUpsertCatalogObjects` endpoint.
    pub async fn search_catalog_items(&self, search_query: SearchCatalogItemsBody)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::POST,
            SquareAPI::Catalog("/search-catalog-items".to_string()),
            Some(&search_query),
            None,
        ).await
    }
}

#[derive(Default)]
pub struct CatalogListParameterBuilder {
    cursor: Option<String>,
    types: Option<Vec<CatalogObjectTypeEnum>>,
    catalog_version: Option<i64>,
}

impl CatalogListParameterBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn add_type(mut self, type_name: CatalogObjectTypeEnum) -> Self {
        if let Some(ref mut types) = &mut self.types {
                for existing_type in types.iter() {
                    if *existing_type == type_name {
                        return self
                    }
                }
                types.push(type_name)
        } else {
            let types = vec![type_name];
            self.types = Some(types);
        }

        self
    }

    pub async fn build(self) -> Vec<(String, String)> {
        let mut res = vec![];

        if let Some(cursor) = self.cursor {
            res.push(("cursor".to_string(), cursor))
        }

        if let Some(types) = self.types {
            let mut combined = "".to_string();
            for type_name in types {
                combined = format!("{}{}%2C", combined, type_name);
            }
            if combined.len() > 3 {
                for _ in 0..3 {
                    combined.pop();
                }
            }
            res.push(("types".to_string(), combined))
        }

        if let Some(catalog_version) = self.catalog_version {
            res.push(("catalog_version".to_string(), catalog_version.to_string()))
        }

        res
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectUpsertRequest {
    idempotency_key: String,
    object: CatalogObject,
}

pub struct ObjectUpsertRequestBuilder {
    object: CatalogObject,
}

// TODO add additional creation functionality to improve usability
impl ObjectUpsertRequestBuilder {
    pub fn new() -> Self {
        ObjectUpsertRequestBuilder {
            object: CatalogObject {
                id: None,
                type_name: None,
                absent_at_location_ids: None,
                catalog_v1_ids: None,
                category_data: None,
                custom_attribute_definition_data: None,
                custom_attributes_values: None,
                discount_data: None,
                image_data: None,
                is_deleted: None,
                item_data: None,
                item_option_data: None,
                measurement_unit_data: None,
                modifier_data: None,
                modifier_list_data: None,
                present_at_all_locations: None,
                present_at_location_ids: None,
                pricing_rule_data: None,
                product_set_data: None,
                quick_amount_settings_data: None,
                subscription_plan_data: None,
                tax_data: None,
                time_period_data: None,
                updated_at: None,
                created_at: None,
                version: None
            }
        }
    }

    pub fn from_object(object: CatalogObject) -> Self {
        ObjectUpsertRequestBuilder {
            object
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.object.id = Some(id);

        self
    }

    pub fn object_type(mut self, object_type: CatalogObjectType) -> Self {
        self.object.type_name = Some(object_type);

        self
    }

    pub fn item_data(mut self, item_data: CatalogItem) -> Self {
        self.object.item_data = Some(item_data);

        self
    }

    pub fn add_variations(mut self, variation: CatalogObjectVariation) -> Self {
        if let Some(mut item_data) = self.object.item_data.as_mut() {
            if let Some(mut variations) = item_data.variations.as_mut() {
                variations.push(variation)
            } else {
                item_data.variations = Some(vec![variation])
            }
        } else {
            self.object.item_data = Some(CatalogItem {
                abbreviation: None,
                available_electronically: None,
                available_for_pickup: None,
                available_online: None,
                category_id: None,
                description: None,
                image_ids: None,
                image_option: None,
                label_color: None,
                modifier_list_info: None,
                name: None,
                product_type: None,
                skip_modifier_scree: None,
                sort_name: None,
                tax_ids: None,
                variations: Some(vec![variation])
            })
        }

        self
    }

    pub async fn build(mut self) -> Result<ObjectUpsertRequest, ObjectUpsertRequestBuildError> {
        if self.object.id.is_none() ||
            self.object.type_name.is_none() {
            Err(ObjectUpsertRequestBuildError)
        } else {
            Ok(ObjectUpsertRequest {
                idempotency_key: Uuid::new_v4().to_string(),
                object: self.object
            })
        }
    }
}

#[derive(Default)]
pub struct ObjectRetrieveParameterBuilder {
    include_related_objects: Option<bool>,
    catalog_version: Option<i64>,
}

impl ObjectRetrieveParameterBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn exclude_related_objects(mut self) -> Self {
        self.include_related_objects = Some(false);

        self
    }

    pub fn include_related_objects(mut self) -> Self {
        self.include_related_objects = Some(true);

        self
    }

    pub fn catalog_version(mut self, version: i64) -> Self {
        self.catalog_version = Some(version);

        self
    }

    pub async fn build(mut self) -> Vec<(String, String)> {
        let ObjectRetrieveParameterBuilder {
            include_related_objects,
            catalog_version,
        } = self;

        let mut res = vec![];

        if let Some(include_related_objects) = include_related_objects {
            res.push(("include_related_objects".to_string(), include_related_objects.to_string()))
        }
        if let Some(catalog_version) = catalog_version {
            res.push(("catalog_version".to_string(), catalog_version.to_string()))
        }

        res
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchCatalogObjectsBody {
    begin_time: Option<String>,
    cursor: Option<String>,
    include_deleted_objects: Option<bool>,
    include_related_objects: Option<bool>,
    limit: Option<i64>,
    object_types: Option<Vec<CatalogObjectType>>,
    query: Option<CatalogQuery>,
}

#[derive(Default)]
pub struct SearchCatalogObjectsBodyBuilder {
    begin_time: Option<String>,
    cursor: Option<String>,
    include_deleted_objects: Option<bool>,
    include_related_objects: Option<bool>,
    limit: Option<i64>,
    object_types: Option<Vec<CatalogObjectType>>,
    query: Option<CatalogQuery>,
}

impl SearchCatalogObjectsBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn begin_time(mut self, begin_time: String) -> Self {
        self.begin_time = Some(begin_time);

        self
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn include_deleted_objects(mut self) -> Self {
        self.include_deleted_objects = Some(true);

        self
    }

    pub fn exclude_deleted_objects(mut self) -> Self {
        self.include_deleted_objects = Some(false);

        self
    }

    pub fn include_related_objects(mut self) -> Self {
        self.include_related_objects = Some(true);

        self
    }

    pub fn exclude_related_objects(mut self) -> Self {
        self.include_related_objects = Some(false);

        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);

        self
    }

    pub fn set_object_types(mut self, object_types: Vec<CatalogObjectType>) -> Self {
        self.object_types = Some(object_types);

        self
    }

    pub fn add_object_type(mut self, object_type: CatalogObjectType) -> Self {
        if let Some(mut object_types) = self.object_types.as_mut() {
            object_types.push(object_type)
        } else {
            self.object_types = Some(vec![object_type])
        }

        self
    }

    pub fn query(mut self, query: CatalogQuery) -> Self {
        self.query = Some(query);

        self
    }

    pub async fn build(mut self) -> SearchCatalogObjectsBody {
        SearchCatalogObjectsBody {
            begin_time: self.begin_time,
            cursor: self.cursor,
            include_deleted_objects: self.include_deleted_objects,
            include_related_objects: self.include_related_objects,
            limit: self.limit,
            object_types: self.object_types,
            query: self.query,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchCatalogItemsBody {
    category_ids: Option<Vec<String>>,
    cursor: Option<String>,
    custom_attribute_filters: Option<Vec<CustomAttributeFilter>>,
    enabled_location_ids: Option<Vec<String>>,
    limit: Option<i32>,
    product_types: Option<Vec<CatalogItemProductType>>,
    sort_order: Option<SortOrder>,
    stock_levels: Option<Vec<SearchCatalogItemsRequestStockLevel>>,
    text_filter: Option<String>,
}

#[derive(Default)]
pub struct SearchCatalogItemsBodyBuilder {
    category_ids: Option<Vec<String>>,
    cursor: Option<String>,
    custom_attribute_filters: Option<Vec<CustomAttributeFilter>>,
    enabled_location_ids: Option<Vec<String>>,
    limit: Option<i32>,
    product_types: Option<Vec<CatalogItemProductType>>,
    sort_order: Option<SortOrder>,
    stock_levels: Option<Vec<SearchCatalogItemsRequestStockLevel>>,
    text_filter: Option<String>,
}

// TODO implement search query builder
impl SearchCatalogItemsBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn low_stock_level(mut self) -> Self {
        if let Some(mut vec) = self.stock_levels.as_mut() {
            vec.push(SearchCatalogItemsRequestStockLevel::Low)
        } else {
            self.stock_levels = Some(vec![SearchCatalogItemsRequestStockLevel::Low])
        }

        self
    }

    pub async fn build(mut self) -> SearchCatalogItemsBody {
        SearchCatalogItemsBody {
            category_ids: self.category_ids,
            cursor: self.cursor,
            custom_attribute_filters: self.custom_attribute_filters,
            enabled_location_ids: self.enabled_location_ids,
            limit: self.limit,
            product_types: self.product_types,
            sort_order: self.sort_order,
            stock_levels: self.stock_levels,
            text_filter: self.text_filter
        }
    }
}

#[cfg(test)]
mod test_catalog {
    use crate::objects::{CatalogItem, CatalogItemVariation, CatalogObjectVariation, Money};
    use crate::objects::enums::{CatalogItemProductType, CatalogObjectType, CatalogPricingType, Currency};
    use super::*;

    #[actix_rt::test]
    async fn test_list_parameter_builder() {
        let expected = vec![("types".to_string(), "ITEM%2CCATEGORY".to_string())];
        let actual = CatalogListParameterBuilder::new()
            .add_type(CatalogObjectTypeEnum::Item)
            .add_type(CatalogObjectTypeEnum::Category)
            .add_type(CatalogObjectTypeEnum::Item)
            .build().await;

        assert_eq!(expected, actual)
    }

    #[actix_rt::test]
    async fn test_list_catalog() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![("types".to_string(), "ITEM,CATEGORY".to_string())];

        let res = sut.list_catalog(Some(input)).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_upsert_object_request_builder() {
        let expected = ObjectUpsertRequest {
            idempotency_key: "".to_string(),
            object: CatalogObject {
                id: Some("#91039132".to_string()),
                type_name: Some(CatalogObjectType::Item),
                absent_at_location_ids: None,
                catalog_v1_ids: None,
                category_data: None,
                custom_attribute_definition_data: None,
                custom_attributes_values: None,
                discount_data: None,
                image_data: None,
                is_deleted: None,
                item_data: Some(CatalogItem {
                    abbreviation: None,
                    available_electronically: None,
                    available_for_pickup: None,
                    available_online: None,
                    category_id: None,
                    description: None,
                    image_ids: None,
                    image_option: None,
                    label_color: None,
                    modifier_list_info: None,
                    name: Some("some name".to_string()),
                    product_type: Some(CatalogItemProductType::Regular),
                    skip_modifier_scree: None,
                    sort_name: None,
                    tax_ids: None,
                    variations: Some(vec![
                        CatalogObjectVariation {
                            id: Some("#234283522".to_string()),
                            type_name: Some(CatalogObjectType::ItemVariation),
                            absent_at_location_ids: None,
                            catalog_v1_ids: None,
                            category_data: None,
                            custom_attribute_definition_data: None,
                            custom_attributes_values: None,
                            discount_data: None,
                            image_data: None,
                            is_deleted: None,
                            item_option_data: None,
                            item_variation_data: Some(CatalogItemVariation {
                                available_for_booking: None,
                                image_ids: None,
                                inventory_alert_threshold: None,
                                inventory_alert_type: None,
                                item_id: None,
                                item_option_values: None,
                                location_overrides: None,
                                measurement_unit_id: None,
                                name: None,
                                ordinal: None,
                                price_money: Some(Money {
                                    amount: Some(15),
                                    currency: Currency::USD,
                                }),
                                pricing_type: Some(CatalogPricingType::FixedPricing),
                                sellable: None,
                                service_duration: None,
                                sku: None,
                                stockable: None,
                                stockable_conversion: None,
                                team_member_ids: None,
                                track_inventory: None,
                                upc: None,
                                user_data: None
                            }),
                            measurement_unit_data: None,
                            modifier_data: None,
                            modifier_list_data: None,
                            present_at_all_locations: None,
                            present_at_location_ids: None,
                            pricing_rule_data: None,
                            product_set_data: None,
                            quick_amount_settings_data: None,
                            subscription_plan_data: None,
                            tax_data: None,
                            time_period_data: None,
                            updated_at: None,
                            created_at: None,
                            version: None
                        }
                    ])
                }),
                item_option_data: None,
                measurement_unit_data: None,
                modifier_data: None,
                modifier_list_data: None,
                present_at_all_locations: None,
                present_at_location_ids: None,
                pricing_rule_data: None,
                product_set_data: None,
                quick_amount_settings_data: None,
                subscription_plan_data: None,
                tax_data: None,
                time_period_data: None,
                updated_at: None,
                created_at: None,
                version: None
            }
        };

        let mut actual = ObjectUpsertRequestBuilder::new()
            .id("#91039132".to_string())
            .object_type(CatalogObjectType::Item)
            .item_data(CatalogItem {
                abbreviation: None,
                available_electronically: None,
                available_for_pickup: None,
                available_online: None,
                category_id: None,
                description: None,
                image_ids: None,
                image_option: None,
                label_color: None,
                modifier_list_info: None,
                name: Some("some name".to_string()),
                product_type: Some(CatalogItemProductType::Regular),
                skip_modifier_scree: None,
                sort_name: None,
                tax_ids: None,
                variations: None
            })
            .add_variations(CatalogObjectVariation {
                id: Some("#234283522".to_string()),
                type_name: Some(CatalogObjectType::ItemVariation),
                absent_at_location_ids: None,
                catalog_v1_ids: None,
                category_data: None,
                custom_attribute_definition_data: None,
                custom_attributes_values: None,
                discount_data: None,
                image_data: None,
                is_deleted: None,
                item_option_data: None,
                item_variation_data: Some(CatalogItemVariation {
                    available_for_booking: None,
                    image_ids: None,
                    inventory_alert_threshold: None,
                    inventory_alert_type: None,
                    item_id: None,
                    item_option_values: None,
                    location_overrides: None,
                    measurement_unit_id: None,
                    name: None,
                    ordinal: None,
                    price_money: Some(Money {
                        amount: Some(15),
                        currency: Currency::USD
                    }),
                    pricing_type: Some(CatalogPricingType::FixedPricing),
                    sellable: None,
                    service_duration: None,
                    sku: None,
                    stockable: None,
                    stockable_conversion: None,
                    team_member_ids: None,
                    track_inventory: None,
                    upc: None,
                    user_data: None
                }),
                measurement_unit_data: None,
                modifier_data: None,
                modifier_list_data: None,
                present_at_all_locations: None,
                present_at_location_ids: None,
                pricing_rule_data: None,
                product_set_data: None,
                quick_amount_settings_data: None,
                subscription_plan_data: None,
                tax_data: None,
                time_period_data: None,
                updated_at: None,
                created_at: None,
                version: None
            })
            .build()
            .await
            .unwrap();

        actual.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
    async fn test_upsert_object() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = ObjectUpsertRequest {
            idempotency_key: Uuid::new_v4().to_string(),
            object: CatalogObject {
                id: Some("#91039132".to_string()),
                type_name: Some(CatalogObjectType::Item),
                absent_at_location_ids: None,
                catalog_v1_ids: None,
                category_data: None,
                custom_attribute_definition_data: None,
                custom_attributes_values: None,
                discount_data: None,
                image_data: None,
                is_deleted: None,
                item_data: Some(CatalogItem {
                    abbreviation: None,
                    available_electronically: None,
                    available_for_pickup: None,
                    available_online: None,
                    category_id: None,
                    description: None,
                    image_ids: None,
                    image_option: None,
                    label_color: None,
                    modifier_list_info: None,
                    name: Some("some name".to_string()),
                    product_type: Some(CatalogItemProductType::Regular),
                    skip_modifier_scree: None,
                    sort_name: None,
                    tax_ids: None,
                    variations: Some(vec![
                        CatalogObjectVariation {
                            id: Some("#234283522".to_string()),
                            type_name: Some(CatalogObjectType::ItemVariation),
                            absent_at_location_ids: None,
                            catalog_v1_ids: None,
                            category_data: None,
                            custom_attribute_definition_data: None,
                            custom_attributes_values: None,
                            discount_data: None,
                            image_data: None,
                            is_deleted: None,
                            item_option_data: None,
                            item_variation_data: Some(CatalogItemVariation {
                                available_for_booking: None,
                                image_ids: None,
                                inventory_alert_threshold: None,
                                inventory_alert_type: None,
                                item_id: None,
                                item_option_values: None,
                                location_overrides: None,
                                measurement_unit_id: None,
                                name: None,
                                ordinal: None,
                                price_money: Some(Money {
                                    amount: Some(15),
                                    currency: Currency::USD,
                                }),
                                pricing_type: Some(CatalogPricingType::FixedPricing),
                                sellable: None,
                                service_duration: None,
                                sku: None,
                                stockable: None,
                                stockable_conversion: None,
                                team_member_ids: None,
                                track_inventory: None,
                                upc: None,
                                user_data: None
                            }),
                            measurement_unit_data: None,
                            modifier_data: None,
                            modifier_list_data: None,
                            present_at_all_locations: None,
                            present_at_location_ids: None,
                            pricing_rule_data: None,
                            product_set_data: None,
                            quick_amount_settings_data: None,
                            subscription_plan_data: None,
                            tax_data: None,
                            time_period_data: None,
                            updated_at: None,
                            created_at: None,
                            version: None
                        }
                    ])
                }),
                item_option_data: None,
                measurement_unit_data: None,
                modifier_data: None,
                modifier_list_data: None,
                present_at_all_locations: None,
                present_at_location_ids: None,
                pricing_rule_data: None,
                product_set_data: None,
                quick_amount_settings_data: None,
                subscription_plan_data: None,
                tax_data: None,
                time_period_data: None,
                updated_at: None,
                created_at: None,
                version: None
            }
        };

        let res = sut.upsert_catalog_object(input).await;

        assert!(res.is_ok())
    }

    // #[actix_rt::test]
    async fn test_delete_object() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = "S5P6A46PDZCBB42ZTRGNWOBB".to_string();

        let res = sut.delete_catalog_object(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_object_retrieve_parameter_builder() {
        let expected = vec![
            ("include_related_objects".to_string(), false.to_string()),
            ("catalog_version".to_string(), 1655427266071_i64.to_string()),
        ];

        let actual = ObjectRetrieveParameterBuilder::new()
            .exclude_related_objects()
            .catalog_version(1655427266071)
            .build()
            .await;

        assert_eq!(expected, actual);
    }

    #[actix_rt::test]
    async fn test_retrieve_object() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![
            ("include_related_objects".to_string(), false.to_string()),
            ("catalog_version".to_string(), 1655427266071_i64.to_string()),
        ];

        let res =
            sut.retrieve_catalog_object(
                "RQITYDA5N7WZDMMJK23HLBHK".to_string(),
                Some(input)
            )
                .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_search_catalog_object_body_builder() {
        let expected = SearchCatalogObjectsBody {
            begin_time: Some("some time".to_string()),
            cursor: None,
            include_deleted_objects: Some(false),
            include_related_objects: Some(true),
            limit: Some(100),
            object_types: Some(vec![CatalogObjectType::Item, CatalogObjectType::ItemVariation]),
            query: None
        };

        let actual = SearchCatalogObjectsBodyBuilder::new()
            .limit(100)
            .exclude_deleted_objects()
            .include_related_objects()
            .begin_time("some time".to_string())
            .add_object_type(CatalogObjectType::Item)
            .add_object_type(CatalogObjectType::ItemVariation)
            .build()
            .await;

        assert_eq!(format!("{:?}",expected), format!("{:?}",actual));
    }

    #[actix_rt::test]
    async fn test_search_objects() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = SearchCatalogObjectsBody {
            begin_time: None,
            cursor: None,
            include_deleted_objects: Some(false),
            include_related_objects: Some(true),
            limit: Some(100),
            object_types: Some(vec![CatalogObjectType::Item, CatalogObjectType::ItemVariation]),
            query: None
        };

        let res = sut.search_catalog_objects(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_catalog_info() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.catalog_info().await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_search_catalog_items_body_builder() {
        let expected = SearchCatalogItemsBody {
            category_ids: None,
            cursor: None,
            custom_attribute_filters: None,
            enabled_location_ids: None,
            limit: None,
            product_types: None,
            sort_order: None,
            stock_levels: None,
            text_filter: None
        };

        let actual = SearchCatalogItemsBodyBuilder::new()
            .build()
            .await;

        assert_eq!(format!("{:?}",expected), format!("{:?}",actual));
    }

    #[actix_rt::test]
    async fn test_search_items() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = SearchCatalogItemsBody {
            category_ids: None,
            cursor: None,
            custom_attribute_filters: None,
            enabled_location_ids: None,
            limit: None,
            product_types: None,
            sort_order: None,
            stock_levels: None,
            text_filter: None
        };

        let res = sut.search_catalog_items(input).await;

        assert!(res.is_ok())
    }
}