// TODO import the necessary mods
use crate::client::SquareClient;
use crate::endpoint::payment::PaymentBuilder;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::{SquareError, LocationsBuildError};
use crate::response::{Location, SquareResponse};

use serde::{Deserialize, Serialize};

// TODO implement the SquareClient
impl SquareClient {
    pub async fn list_locations(&self) -> Result<SquareResponse, SquareError> {
        self.request::<Locations>(EndpointVerb::GET ,SquareEndpoint::Locations, None).await
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
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn test_locations_builder_success() {
        let expected = Locations;
        let actual = LocationsBuilder::new().build().await;
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }

    #[actix_rt::test]
    async fn test_create_location_request() {
        let sut = SquareClient::new("EAAAEIQAdp294a9PHkqjSlWhi3qWBHnUwposv1n-TZrn0XW6xHGg810-72UrNr6U");
        let actual = sut.list_locations().await;
        assert!(actual.is_ok())
    }
}