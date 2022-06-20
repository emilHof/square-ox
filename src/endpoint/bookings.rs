/*!
Bookings functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::SearchQueryBuildError;
use crate::error::SquareError;
use crate::response::{SquareResponse, FilterValue};

use serde::{Deserialize, Serialize};

impl SquareClient {
    /// Create a booking with the given [Bookings](Bookings) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `create_booking` - A [Bookings](Bookings) created from the [BookingsBuilder](BookingsBuilder)
    pub async fn create_booking(&self, booking: Bookings) -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::Bookings,
            Some(&booking),
            None,
        ).await
    }

    /// Search for availability with the given [SearchQuery](SearchQuery) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `search_availability` - A [SearchQuery](SearchQuery) created from the [SearchQueryBuilder](SearchQueryBuilder)
    pub async fn search_availability(&self, search_query: SearchQuery) -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::BookingsAvailabilitySearch,
            Some(&search_query),
            None,
        ).await
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
    query: QQuery,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct QQuery {
    filter: QueryFilter,
}


#[derive(Serialize, Debug, Deserialize)]
pub struct QueryFilter {
    start_at_range: StartAtRange,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_filters: Option<Vec<SegmentFilter>>
}

/// The [SearchQueryBuilder](SearchQueryBuilder)
#[derive(Default)]
pub struct SearchQueryBuilder {
    start_at_range: Option<StartAtRange>,
    booking_id: Option<String>,
    location_id: Option<String>,
    segment_filters: Option<Vec<SegmentFilter>>
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

    fn segment_filters(mut self, service_variation_id: String) -> Self {
        let new_filter = SegmentFilter {
            service_variation_id: service_variation_id.clone(),
            team_member_id_filter: None
        };

        match self.segment_filters.take() {
            Some(mut filters) => {
                filters.push(new_filter);
                self.segment_filters = Some(filters)
            },
            None => {
                let mut filters = Vec::new();
                filters.push(new_filter);
                self.segment_filters = Some(filters)
            }
        };

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
            query: QQuery {
                filter: QueryFilter {
                    start_at_range,
                    booking_id,
                    location_id,
                    segment_filters,
                }
            }
        })
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct StartAtRange {
    end_at: String,
    start_at: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SegmentFilter {
    service_variation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_member_id_filter: Option<FilterValue>,
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
            query: QQuery {
                filter: QueryFilter {
                    start_at_range: StartAtRange {
                        end_at: "2023-10-12T07:20:50.52Z".to_string(),
                        start_at: "2022-10-12T07:20:50.52Z".to_string(),
                    },
                    booking_id: None,
                    location_id: Some("LPNXWH14W6S47".to_string()),
                    segment_filters: None
                }
            }
        };
        let actual = sut.build().await;

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()))
    }


    #[actix_rt::test]
    async fn test_search_availability() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = SearchQueryBuilder::new()
            .start_at_range(
                "2022-09-12T07:20:50.52Z".to_string(),
                "2022-10-12T07:20:50.52Z".to_string())
            .location_id("L1JC53TYHS40Z".to_string())
            .segment_filters("BSOL4BB6RCMX6SH4KQIFWZDP".to_string())
            .build().await.unwrap();

        println!("{:?}", input);
        println!("{:?}", serde_json::to_string(&input).unwrap());

        let result = sut.search_availability(input).await;

        assert!(result.is_ok())
    }
}

