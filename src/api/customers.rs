/*!
Customers functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, CustomerBuildError, CustomerDeleteBuildError,
                    CustomerSearchQueryBuildError, ListParametersBuilderError};
use crate::response::SquareResponse;
use crate::objects::{Address, Customer, FilterValue, enums::CustomerCreationSource};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl SquareClient {
    pub fn customers(&self) -> Customers {
        Customers {
            client: &self
        }
    }
}

pub struct Customers<'a> {
    client: &'a SquareClient,
}

impl<'a> Customers<'a> {
    /// Lists customer profiles associated with a Square account.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/customers/list-customers)
    pub async fn list(self, list_parameters: Vec<(String, String)>)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Customers("".to_string()),
            None::<&Customer>,
            Some(list_parameters),
        ).await
    }

    /// Creates a new customer for a business.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/customers/create-customer)
    pub async fn create(self, customer: Customer)
                        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Customers("".to_string()),
            Some(&customer),
            None,
        ).await
    }

    /// Searches the customer profiles associated with a Square account using a supported query filter.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/customers/search-customers)
    pub async fn search(self, customer_search_query: CustomerSearchQuery)
                        -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Customers("/search".to_string()),
            Some(&customer_search_query),
            None,
        ).await
    }

    /// Deletes a customer profile from a business.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/customers/delete-customer)
    pub async fn delete(self, customer_to_delete: CustomerDelete)
                        -> Result<SquareResponse, SquareError > {
        self.client.request(
            Verb::DELETE,
            SquareAPI::Customers(format!("/{}", customer_to_delete.customer_id)),
            None::<&CustomerSearchQuery>,
            customer_to_delete.version,
        ).await
    }
}

#[derive(Default)]
pub struct CustomerListParametersBuilder {
    cursor: Option<String>,
    limit: Option<String>,
    sort_field: Option<String>,
    sort_order: Option<String>,
}

impl CustomerListParametersBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor.clone());

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        if limit < 1 || limit > 100 { return self }
        let limit = limit.to_string();
        self.limit = Some(limit);

        self
    }

    pub fn sort_field_default(mut self) -> Self {
        self.sort_field = Some("DEFAULT".to_string());

        self
    }

    pub fn sort_field_created_at(mut self) -> Self {
        self.sort_field = Some("CREATED_AT".to_string());

        self
    }

    pub fn sort_order_desc(mut self) -> Self {
        self.sort_order = Some("DESC".to_string());

        self
    }

    pub fn sort_order_asc(mut self) -> Self {
        self.sort_order = Some("ASC".to_string());

        self
    }

    pub async fn build(mut self) -> Result<Vec<(String, String)>, ListParametersBuilderError> {
        let mut res = Vec::new();
        if let Some(cursor) = self.cursor.take() {
            res.push(("cursor".to_string(), cursor))
        }
        if let Some(limit) = self.limit.take() {
            res.push(("limit".to_string(), limit))
        }
        if let Some(sort_field) = self.sort_field.take() {
            res.push(("sort_field".to_string(), sort_field))
        }
        if let Some(sort_order) = self.sort_order.take() {
            res.push(("sort_order".to_string(), sort_order))
        }

        Ok(res)
    }
}

#[derive(Default)]
pub struct CustomerBuilder {
    customer: Customer,
}

// TODO add building functions for remaining attributes
impl CustomerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn given_name(mut self, given_name: String) -> Self {
        self.customer.given_name = Some(given_name);

        self
    }

    pub fn family_name(mut self, family_name: String) -> Self {
        self.customer.family_name = Some(family_name);

        self
    }

    pub fn nickname(mut self, nickname: String) -> Self {
        self.customer.nickname = Some(nickname);

        self
    }

    pub fn email_address(mut self, email_address: String) -> Self {
        self.customer.email_address = Some(email_address);

        self
    }

    pub fn address_from_address(mut self, address: Address) -> Self {
        self.customer.address = Some(address);

        self
    }

    pub fn birthday(mut self, birthday: String) -> Self {
        self.customer.birthday = Some(birthday);

        self
    }

    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.customer.phone_number = Some(phone_number);

        self
    }

    pub fn note(mut self, note: String) -> Self {
        self.customer.birthday = Some(note);

        self
    }

    pub async fn build(self) -> Result<Customer, CustomerBuildError> {
        let mut customer = self.customer;
        let mut cnt = 0;
        if customer.given_name.is_some() {cnt += 1}
        if customer.family_name.is_some() {cnt += 1}
        if customer.company_name.is_some() {cnt += 1}
        if customer.email_address.is_some() {cnt += 1}
        if customer.phone_number.is_some() {cnt += 1}

        if cnt == 0 { return Err(CustomerBuildError) }

        customer.idempotency_key = Some(Uuid::new_v4().to_string());

        Ok(customer)
    }
}

#[derive(Debug)]
pub struct CustomerDelete {
    customer_id: String,
    version: Option<Vec<(String, String)>>,
}

#[derive(Default)]
pub struct CustomerDeleteBuilder {
    customer_id: Option<String>,
    version: Option<Vec<(String, String)>>
}

impl CustomerDeleteBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.customer_id = Some(customer_id);

        self
    }

    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(vec![("version".to_string(), version.to_string())]);

        self
    }

    async fn build(self) -> Result<CustomerDelete, CustomerDeleteBuildError>  {
        match self.customer_id {
            Some(customer_id) => Ok(CustomerDelete{ customer_id, version: self.version}),
            None => Err(CustomerDeleteBuildError)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CustomerSearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<SearchQueryAttribute>
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

#[derive(Default)]
pub struct CustomerSearchQueryBuilder {
    cursor: Option<String>,
    limit: Option<i64>,
    query: Option<SearchQueryAttribute>,
}

// TODO add building function for adding group_id's
impl CustomerSearchQueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        if limit < 1 || limit > 100 { return self };
        self.limit = Some(limit);

        self
    }

    pub fn created_at(mut self, start: String, end: String) -> Self {
        let time_range = TimeRange {
                start_at: Some(start),
                end_at: Some(end),
        };
        let filter = CustomerFilter {
            created_at:  Some(time_range.clone()),
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                filter.created_at = Some(time_range);
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn updated_at(mut self, start: String, end: String) -> Self {
        let time_range = TimeRange {
                start_at: Some(start),
                end_at: Some(end),
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: Some(time_range.clone())
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                filter.updated_at = Some(time_range);
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn exact_email_address(mut self, email: String) -> Self {
        let email_group = CustomerTextFilter {
            exact: Some(email.clone()),
            fuzzy: None
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: Some(email_group.clone()),
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut email_group) = &mut filter.email_address {
                    email_group.exact = Some(email);
                } else {
                    filter.email_address = Some(email_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn fuzzy_email_address(mut self, email: String) -> Self {
        let email_group = CustomerTextFilter {
            exact: None,
            fuzzy: Some(email.clone())
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: Some(email_group.clone()),
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut email_group) = &mut filter.email_address {
                    email_group.fuzzy = Some(email);
                } else {
                    filter.email_address = Some(email_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn exact_phone_number(mut self, number: String) -> Self {
        let phone_group = CustomerTextFilter {
            exact: Some(number.clone()),
            fuzzy: None,
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: Some(phone_group.clone()),
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut phone_group) = &mut filter.phone_number {
                    phone_group.exact = Some(number);
                } else {
                    filter.phone_number = Some(phone_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn fuzzy_phone_number(mut self, number: String) -> Self {
        let phone_group = CustomerTextFilter {
            exact: None,
            fuzzy: Some(number.clone())
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: Some(phone_group.clone()),
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut phone_group) = &mut filter.phone_number {
                    phone_group.fuzzy = Some(number);
                } else {
                    filter.phone_number = Some(phone_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn exact_reference_id(mut self, id: String) -> Self {
        let reference_id_group = CustomerTextFilter {
            exact: Some(id.clone()),
            fuzzy: None,
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: Some(reference_id_group.clone()),
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut reference_id_group) = &mut filter.reference_id {
                    reference_id_group.exact = Some(id);
                } else {
                    filter.reference_id = Some(reference_id_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn fuzzy_reference_id(mut self, id: String) -> Self {
        let reference_id_group = CustomerTextFilter {
            exact: None,
            fuzzy: Some(id.clone()),
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: None,
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: Some(reference_id_group.clone()),
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut reference_id_group) = &mut filter.reference_id {
                    reference_id_group.fuzzy = Some(id);
                } else {
                    filter.reference_id = Some(reference_id_group)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn set_creation_source_exclude(mut self) -> Self {
        let creation_source = CreationSource {
            rule: Some("EXCLUDE".to_string()),
            values: None
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: Some(creation_source.clone()),
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut creation_source) = &mut filter.creation_source {
                    creation_source.rule = Some("EXCLUDE".to_string());
                } else {
                    filter.creation_source = Some(creation_source)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn set_creation_source_include(mut self) -> Self {
        let creation_source = CreationSource {
            rule: Some("INCLUDE".to_string()),
            values: None
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: Some(creation_source.clone()),
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = query.filter {
                if let Some(ref mut creation_source) = &mut filter.creation_source {
                    creation_source.rule = Some("INCLUDE".to_string());
                } else {
                    filter.creation_source = Some(creation_source)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub fn creation_source_value(mut self, value: CustomerCreationSource) -> Self {
        let values = vec![value.clone()];
        let creation_source = CreationSource {
            rule: Some("INCLUDE".to_string()),
            values: Some(values.clone())
        };
        let filter = CustomerFilter {
            created_at:  None,
            creation_source: Some(creation_source.clone()),
            email_address: None,
            group_ids: None,
            phone_number: None,
            reference_id: None,
            updated_at: None
        };
        let query = SearchQueryAttribute {
            filter: Some(filter.clone()),
            sort: None
        };

        if let Some(ref mut query) = &mut self.query {
            if let Some(ref mut filter) = &mut query.filter {
                if let Some(ref mut creation_source) = &mut filter.creation_source {
                    if let Some(ref mut values) = &mut creation_source.values {
                        for val in values.iter() {
                            if *val == value {return self}
                        }
                        values.push(value)
                    } else {
                        creation_source.values = Some(values);
                    }
                } else {
                    filter.creation_source = Some(creation_source)
                }
            } else {
                query.filter = Some(filter);
            }
        } else {
            self.query = Some(query);
        }

        self
    }

    pub async fn build(self) -> Result<CustomerSearchQuery, CustomerSearchQueryBuildError> {
        Ok (
            CustomerSearchQuery {
                cursor: self.cursor,
                limit: self.limit,
                query: self.query,
            }
        )
    }
}

#[cfg(test)]
mod test_customers {
    use super::*;

    #[actix_rt::test]
    async fn test_list_parameter_builder() {
        let mut sut = CustomerListParametersBuilder::new();
        let expected =  vec![
            ("limit".to_string(), "4".to_string()),
            ("sort_field".to_string(), "DEFAULT".to_string())
        ];
        let actual =
            sut.limit(4).limit(101).sort_field_default().build().await;

        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap())
    }

    #[actix_rt::test]
    async fn test_list_customers() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![("limit".to_string(), "23".to_string()),
             ("sort_field".to_string(), "DEFAULT".to_string())];

        let result = sut.customers().list(input).await;

        assert!(result.is_ok());
        println!("{:?}", result.unwrap())
    }

    #[actix_rt::test]
    async fn test_customer_builder() {
        let expected = Customer {
            id: None,
            birthday: Some("1996-11-02".to_string()),
            address: Some(Address{
                address_line_1: Some("1234 Hillborrow Rd.".to_string()),
                address_line_2: None,
                address_line_3: None,
                locality: Some("Charleston".to_string()),
                sublocality: None,
                administrative_district_level: Some("MA".to_string()),
                postal_code: Some("12345".to_string()),
                country: Some("United States".to_string())
            }),
            company_name: None,
            created_at: None,
            creation_source: None,
            updated_at: None,
            email_address: None,
            family_name: Some("Ramsey".to_string()),
            given_name: Some("Pierre".to_string()),
            group_ids: None,
            nickname: None,
            note: None,
            phone_number: Some("123-456-7890".to_string()),
            preferences: None,
            reference_id: None,
            segment_ids: None,
            tax_ids: None,
            version: None,
            cards: None,
            idempotency_key: None
        };

        let address = Address{
            address_line_1: Some("1234 Hillborrow Rd.".to_string()),
            address_line_2: None,
            address_line_3: None,
            locality: Some("Charleston".to_string()),
            sublocality: None,
            administrative_district_level: Some("MA".to_string()),
            postal_code: Some("12345".to_string()),
            country: Some("United States".to_string())
        };

        let mut actual = CustomerBuilder::new().address_from_address(address)
            .given_name("Pierre".to_string())
            .family_name("Ramsey".to_string())
            .phone_number("123-456-7890".to_string())
            .phone_number("123-456-7890".to_string())
            .birthday("1996-11-02".to_string())
            .build()
            .await;

        assert!(actual.is_ok());

        actual.as_mut().unwrap().idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }

    // #[actix_rt::test]
    async fn test_create_customer() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CustomerBuilder::new()
            .given_name("Boyd".to_string())
            .nickname("the coolest".to_string())
            .build().await.unwrap();

        let result = sut.customers().create(input).await;

        assert!(result.is_ok());
        println!("{:?}", result.unwrap())
    }

    #[actix_rt::test]
    async fn test_customer_delete_builder() {
        let expected = CustomerDelete {
            customer_id: "dew212ewfd32123ca".to_string(),
            version: None
        };

        let actual = CustomerDeleteBuilder::new()
            .customer_id("dew212ewfd32123ca".to_string())
            .build()
            .await;

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }

    // #[actix_rt::test]
    async fn test_delete_customer() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CustomerDelete {
            customer_id: "WPGEDT7V38Y318JVGZ1G1C39W4".to_string(),
            version: None
        };

        let res = sut.customers().delete(input).await;

        assert!(res.is_ok());
    }

    #[actix_rt::test]
    async fn test_customer_search_query_builder() {
        let expected = CustomerSearchQuery {
            cursor: None,
            limit: Some(5),
            query: Some(SearchQueryAttribute {
                filter: Some(CustomerFilter {
                    created_at: Some(TimeRange {
                        end_at: Some("2022-01-23T20:21:54.859Z".to_string()),
                        start_at: Some("2018-01-23T20:21:54.859Z".to_string())
                    }),
                    creation_source: Some(CreationSource {
                        rule: Some("EXCLUDE".to_string()),
                        values: Some(vec![CustomerCreationSource::Appointments,
                                          CustomerCreationSource::Coupon])
                    }),
                    email_address: Some(CustomerTextFilter {
                        exact: Some("emil.k.hofstetter@gmail.com".to_string()),
                        fuzzy: Some("3umel.us@gmail.com".to_string())
                    }),
                    group_ids: None,
                    phone_number: Some(CustomerTextFilter {
                        exact: Some("571-694-6282".to_string()),
                        fuzzy: Some("0176-47-85-993".to_string())
                    }),
                    reference_id: Some(CustomerTextFilter {
                        exact: Some("cmiw9u209md82".to_string()),
                        fuzzy: Some("432mi23cß2".to_string())
                    }),
                    updated_at: None
                }),
                sort: None
            })
        };

        let actual = CustomerSearchQueryBuilder::new()
            .limit(5).limit(1001).created_at(
            "2018-01-23T20:21:54.859Z".to_string(),
            "2022-01-23T20:21:54.859Z".to_string(),
            ).fuzzy_email_address("3umel.us@gmail.com".to_string())
            .exact_email_address("emil.k.hofstetter@gmail.com".to_string())
            .exact_phone_number("571-694-6282".to_string())
            .fuzzy_phone_number("0176-47-85-993".to_string())
            .fuzzy_reference_id("432mi23cß2".to_string())
            .exact_reference_id("cmiw9u209md82".to_string())
            .creation_source_value(CustomerCreationSource::Appointments)
            .creation_source_value(CustomerCreationSource::Coupon)
            .creation_source_value(CustomerCreationSource::Appointments)
            .set_creation_source_exclude()
            .build().await;

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
    }

    #[actix_rt::test()]
    async fn test_search_customers() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CustomerSearchQuery {
            cursor: None,
            limit: None,
            query: Some(SearchQueryAttribute {
                filter: Some(CustomerFilter {
                    created_at: Some(TimeRange {
                        end_at: None,
                        start_at: None,
                    }),
                    creation_source: Some(CreationSource {
                        rule: Some("INCLUDE".to_string()),
                        values: Some(vec![CustomerCreationSource::Appointments,
                                          CustomerCreationSource::ThirdParty]),
                    }),
                    email_address: Some(CustomerTextFilter {
                        exact: None,
                        fuzzy: Some(".co".to_string()),
                    }),
                    group_ids: None,
                    phone_number: Some(CustomerTextFilter {
                        exact: None,
                        fuzzy: None,
                    }),
                    reference_id: Some(CustomerTextFilter {
                        exact: None,
                        fuzzy: None,
                    }),
                    updated_at: None
                }),
                sort: None
            })
        };

        let result = sut.customers().search(input).await;

        // assert!(result.is_ok() || result.err().unwrap().get().is_some())
        assert!(result.is_ok())
    }
}