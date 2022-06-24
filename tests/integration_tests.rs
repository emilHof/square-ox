use square_rs;
use actix_rt;

use dotenv::dotenv;
use std::env;
use square_rs::api::catalog;
use square_rs::api::customers::{CustomerListParametersBuilder, CustomerSearchQueryBuilder};
use square_rs::response::ResponseError;

#[actix_rt::test]
async fn test_list_locations() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_rs::client::SquareClient::new(&access_token);

    let res = sut.list_locations().await;

    assert!(res.is_ok())
}

#[actix_rt::test]
async fn test_list_customer() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_rs::client::SquareClient::new(&access_token);

    let input = CustomerListParametersBuilder::new().build().await;

    assert!(input.is_ok());

    let res = sut.list_customers(input.unwrap()).await;

    assert!(res.is_ok());
}

#[actix_rt::test]
async fn test_list_catalog() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_rs::client::SquareClient::new(&access_token);

    let input = catalog::CatalogListParameterBuilder::new().build().await;

    let res = sut.list_catalog(Some(input)).await;

    assert!(res.is_ok());
}