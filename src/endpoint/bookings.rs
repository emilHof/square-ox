/*!
Bookings functionality of the [Square API](https://developer.squareup.com).
 */

use std::iter::Filter;
use std::ops::Deref;
use crate::client::SquareClient;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::SearchQueryBuildError;
use crate::error::SquareError;
use crate::money::{Currency, Money};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};

impl SquareClient {
    /// Create a booking with the given [Bookings](Bookings) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `create_booking` - A [Bookings](Bookings) created from the [BookingsBuilder](BookingsBuilder)
    pub async fn create_booking(&self, booking: Bookings) -> Result<SquareResponse, SquareError> {
        self.request(EndpointVerb::POST ,SquareEndpoint::Bookings, Some(&booking)).await
    }



    /// Search for availability with the given [SearchQuery](SearchQuery) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `search_availability` - A [SearchQuery](SearchQuery) created from the [SearchQueryBuilder](SearchQueryBuilder)
    pub async fn search_availability(&self, search_query: SearchQuery) -> Result<SquareResponse, SquareError> {
        self.request(EndpointVerb::POST ,SquareEndpoint::BookingsAvailabilitySearch, Some(&search_query)).await
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Bookings {
    id: String,
    customer_id: String,
}

/// The [BookingsBuilder](BookingsBuilder)
pub struct BookingsBuilder {
    id: Option<String>,
    customer_id: Option<String>,
}

/// The [SearchQuery](SearchQuery)
#[derive(Serialize, Debug, Deserialize)]
pub struct SearchQuery {
    start_at_range: StartAtRange,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_filters: Option<Vec<SegmentFilers>>
}

/// The [SearchQueryBuilder](SearchQueryBuilder)
#[derive(Default)]
pub struct SearchQueryBuilder {
    start_at_range: Option<StartAtRange>,
    booking_id: Option<String>,
    location_id: Option<String>,
    segment_filters: Option<Vec<SegmentFilers>>
}

impl SearchQueryBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn start_at_range(mut self, start: String, end: String) -> Self {
        self.start_at_range = Some(StartAtRange {
            end_at: end.clone(),
            start_at: start.clone(),
        });

        self
    }

    fn location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);

        self
    }

    async fn build(&self) -> Result<SearchQuery, SearchQueryBuildError> {
        let start_at_range = match &self.start_at_range {
            Some(sar) => sar.clone(),
            None => return Err(SearchQueryBuildError),
        };

        let booking_id = self.booking_id.clone();
        let location_id = self.location_id.clone();
        let segment_filters = self.segment_filters.clone();

        Ok(SearchQuery {
            start_at_range,
            booking_id,
            location_id,
            segment_filters,
        })
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct StartAtRange {
    end_at: String,
    start_at: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SegmentFilers {
    service_variation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_member_id_filter: Option<FilterValue>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct FilterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    none: Option<Vec<String>>,
}

#[cfg(test)]
mod test_bookings {
    use super::*;

    #[actix_rt::test]
    async fn test_search_query_builder() {
        let sut = SearchQueryBuilder::new()
            .start_at_range(
                "2022-10-12T07:20:50.52Z".to_string(),
                "2023-10-12T07:20:50.52Z".to_string())
            .location_id("LPNXWH14W6S47".to_string());
        let expected = SearchQuery {
            start_at_range: StartAtRange {
                end_at: "2023-10-12T07:20:50.52Z".to_string(),
                start_at: "2022-10-12T07:20:50.52Z".to_string(),
            },
            booking_id: None,
            location_id: Some("LPNXWH14W6S47".to_string()),
            segment_filters: None
        };
        let actual = sut.build().await;

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }


    #[actix_rt::test]
    fn test_search_availability() {
        // TODO test the booking availability search function
    }
}

