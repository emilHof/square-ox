/*!
Customers functionality of the [Square API](https://developer.squareup.com).
 */

// TODO import the necessary mods
use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, LocationsBuildError};
use crate::response::SquareResponse;
use crate::objects::Location;

use serde::{Deserialize, Serialize};

// TODO implement the SquareClient
impl SquareClient {
    pub async fn list_locations(&self) -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Locations,
            None::<&Location>,
            None,
        ).await
    }
}

// TODO create a Locations struct with minimum necessary information


#[cfg(test)]
mod test_locations {
    use super::*;

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