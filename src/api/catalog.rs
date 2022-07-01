/*!
Catalog functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::SquareError;
use crate::response::SquareResponse;
use crate::objects::{CatalogObject, enums::CatalogObjectTypeEnum};

use serde::{Deserialize, Serialize};

impl SquareClient {
    pub async fn upsert_catalog_object(&self, object: CatalogObjectRequest)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::POST,
            SquareAPI::Catalog("/object".to_string()),
            Some(&object),
            None,
        ).await
    }

    pub async fn list_catalog(&self, list_parameters: Option<Vec<(String, String)>>)
    -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Catalog("/list".to_string()),
            None::<&CatalogObject>,
            list_parameters
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CatalogObjectRequest {
    idempotency_key: Option<String>
}


// TODO add catalog upsert feature
#[derive(Default)]
pub struct CatalogObjectRequestBuilder {
}

#[cfg(test)]
mod test_catalog {
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
}