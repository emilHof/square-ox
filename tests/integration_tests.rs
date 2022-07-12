use square_ox;
use actix_rt;

use dotenv::dotenv;
use std::env;
use square_ox::api::catalog;
use square_ox::api::customers::CustomerListParametersBuilder;

#[actix_rt::test]
async fn test_list_locations() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_ox::client::SquareClient::new(&access_token);

    let res = sut.locations()
        .list().await;

    assert!(res.is_ok())
}

#[actix_rt::test]
async fn test_list_customer() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_ox::client::SquareClient::new(&access_token);

    let input = CustomerListParametersBuilder::new().build().await;

    assert!(input.is_ok());

    let res = sut.customers().list(input.unwrap()).await;

    assert!(res.is_ok());
}

#[actix_rt::test]
async fn test_list_catalog() {
    dotenv().ok();
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set!");

    let sut = square_ox::client::SquareClient::new(&access_token);

    let input = catalog::CatalogListParameterBuilder::new().build().await;

    let res = sut.catalog()
        .list(Some(input))
        .await;

    assert!(res.is_ok());
}