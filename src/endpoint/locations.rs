// TODO import the necessary mods
use crate::client::SquareClient;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::{SquareError, LocationsBuildError};
use crate::response::{jsons::Location, SquareResponse};

use serde::{Deserialize, Serialize};

// TODO implement the SquareClient
impl SquareClient {
    pub async fn list_locations(&self) -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::GET,
            SquareEndpoint::Locations,
            None::<&Location>,
            None,
        ).await
    }
}

// TODO create a Locations struct with minimum necessary information

#[derive(Serialize, Debug, Deserialize)]
pub struct Locations;

// TODO have a LocationBuilder
pub struct LocationsBuilder;

impl Default for LocationsBuilder {
    fn default() -> Self {
        Self
    }
}

impl LocationsBuilder {
    pub fn new() -> Self{ Default::default() }

    pub async fn build(&self) -> Result<Locations, LocationsBuildError> {
        Ok(Locations)
    }
}


#[cfg(test)]
mod test_locations {
    use super::*;

    #[actix_rt::test]
    async fn test_locations_builder_success() {
        let expected = Locations;
        let actual = LocationsBuilder::new().build().await;
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }

    #[actix_rt::test]
    async fn test_create_location_request() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);
        let result = sut.list_locations().await;
        assert!(result.is_ok())
    }
}