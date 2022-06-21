/*!
Bookings functionality of the [Square API](https://developer.squareup.com).
 */

use std::fmt::format;
use crate::client::SquareClient;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::{SearchQueryBuildError, BookingsPostBuildError, BookingsCancelBuildError};
use crate::error::SquareError;
use crate::response::{SquareResponse, jsons::FilterValue,
                      enums::BusinessAppointmentSettingsBookingLocationType};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::jsons::{AppointmentSegment, Booking};

impl SquareClient {
    /// Search for availability with the given [SearchQuery](SearchQuery) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `search_availability` - A [SearchQuery](SearchQuery)
    /// created from the [SearchQueryBuilder](SearchQueryBuilder)
    pub async fn search_availability(&self, search_query: SearchQuery)
                                     -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::Bookings("/availability/search".to_string()),
            Some(&search_query),
            None,
        ).await
    }

    /// Create a booking with the given [Bookings](Bookings) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `create_booking` - A [Bookings](Bookings) created from the
    /// [BookingsBuilder](BookingsBuilder)
    pub async fn create_booking(&self, booking_post: BookingsPost)
        -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::Bookings("".to_string()),
            Some(&booking_post),
            None,
        ).await
    }

    /// Create a booking with the given [Bookings](Bookings) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `create_booking` - A [Bookings](Bookings) created from the
    /// [BookingsBuilder](BookingsBuilder)
    pub async fn cancel_booking(&self, booking_to_cancel: BookingsCancel)
                                -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::Bookings(format!("/{}/cancel",
                                             booking_to_cancel.booking_id.clone())),
            Some(&booking_to_cancel.body),
            None,
        ).await
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookingsPost {
    idempotency_key: String,
    booking: Booking,
}

/// The [BookingsBuilder](BookingsBuilder)
#[derive(Default)]
pub struct BookingsPostBuilder(Booking);

impl BookingsPostBuilder {
    fn new() -> Self {
        let mut booking = Booking::default();
        booking.appointment_segments = Some(vec![]);
        Self(booking)
    }

    fn customer_id(mut self, customer_id: String) -> Self {
        self.0.customer_id = Some(customer_id);

        self
    }

    fn location_id(mut self, location_id: String) -> Self {
        self.0.location_id = Some(location_id);

        self
    }

    fn location_type(mut self, location_type: String) -> Self {
        if BusinessAppointmentSettingsBookingLocationType::validate(&location_type) {
            self.0.location_type = Some(location_type);
        }

        self
    }

    fn start_at(mut self, start_at_date_time: String) -> Self {
        self.0.start_at = Some(start_at_date_time);

        self
    }

    fn add_appointment_segment(mut self, appointment_segment: AppointmentSegment) -> Self {
        self.0.appointment_segments.as_mut().unwrap().push(appointment_segment);

        self
    }

    fn seller_note(mut self, seller_note: String) -> Self {
        self.0.seller_note = Some(seller_note);

        self
    }

    fn customer_note(mut self, customer_note: String) -> Self {
        self.0.customer_note = Some(customer_note);

        self
    }

    async fn build(mut self) -> Result<BookingsPost, BookingsPostBuildError> {

        let booking = self.0;

        if booking.customer_id.is_none()
            || booking.location_id.is_none()
            || booking.appointment_segments.as_ref().unwrap().len() < 0
            || booking.start_at.is_none() {
            Err(BookingsPostBuildError)
        } else {
            Ok(BookingsPost{
                idempotency_key: Uuid::new_v4().to_string(),
                booking,
            })
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookingsCancel {
    booking_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<BookingsCancelBody>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookingsCancelBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booking_version: Option<i32>,
}

#[derive(Default)]
pub struct BookingsCancelBuilder {
    booking_id: Option<String>,
    body: Option<BookingsCancelBody>,
}

impl BookingsCancelBuilder {
    fn new() -> Self {
        BookingsCancelBuilder {
            booking_id: None,
            body: Some(BookingsCancelBody {
                idempotency_key: Some(Uuid::new_v4().to_string()),
                booking_version: None
            })
        }
    }
    
    fn booking_id(mut self, booking_id: String) -> Self {
        self.booking_id = Some(booking_id);
        
        self
    }

    fn booking_version(mut self, booking_version: i32) -> Self {
        let body = self.body.as_mut().unwrap();
        body.booking_version = Some(booking_version);


        self
    }

    async fn build(self) -> Result<BookingsCancel, BookingsCancelBuildError> {
        if self.booking_id.is_none() {
            Err(BookingsCancelBuildError)
        } else {
            Ok( BookingsCancel {
                booking_id: self.booking_id.unwrap(),
                body: self.body
            })
        }
    }
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

    #[actix_rt::test]
    async fn test_booking_post_builder() {
        let actual = BookingsPostBuilder::new()
            .start_at("2022-10-11T16:30:00Z".to_string())
            .location_id("L1JC53TYHS40Z".to_string())
            .customer_id("7PB8P9553RYA3F672D15369VK4".to_string())
            .add_appointment_segment(AppointmentSegment {
                duration_minutes: 60.00,
                team_member_id: "TMKFnToW8ByXrcm6".to_string(),
                any_team_member_id: None,
                intermission_minutes: None,
                resource_ids: None,
                service_variation_id: "BSOL4BB6RCMX6SH4KQIFWZDP".to_string(),
                service_variation_version:  1655427266071,
            })
            .build().await;
        let expected = Booking {
            id: None,
            all_day: None,
            appointment_segments: Some(vec![AppointmentSegment {
                duration_minutes: 60.00,
                team_member_id: "TMKFnToW8ByXrcm6".to_string(),
                any_team_member_id: None,
                intermission_minutes: None,
                resource_ids: None,
                service_variation_id: "BSOL4BB6RCMX6SH4KQIFWZDP".to_string(),
                service_variation_version:  1655427266071,
            }]),
            created_at: None,
            booking_creator_details: None,
            customer_id: Some("7PB8P9553RYA3F672D15369VK4".to_string()),
            customer_note: None,
            location_id: Some("L1JC53TYHS40Z".to_string()),
            location_type: None,
            seller_note: None,
            source: None,
            start_at: Some("2022-10-11T16:30:00Z".to_string()),
            status: None,
            transition_time_minutes: None,
            updated_at: None,
            version: None
        };

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap().booking))
    }

    #[actix_rt::test]
    async fn test_booking_post_builder_fail() {
        let res = BookingsPostBuilder::new()
            .start_at("2022-10-11T16:30:00Z".to_string())
            .customer_id("7PB8P9553RYA3F672D15369VK4".to_string())
            .add_appointment_segment(AppointmentSegment {
                duration_minutes: 60.00,
                team_member_id: "TMKFnToW8ByXrcm6".to_string(),
                any_team_member_id: None,
                intermission_minutes: None,
                resource_ids: None,
                service_variation_id: "BSOL4BB6RCMX6SH4KQIFWZDP".to_string(),
                service_variation_version:  1655427266071,
            })
            .build().await;

        assert!(res.is_err());
    }

    #[actix_rt::test]
    async fn test_create_booking() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = BookingsPost {
            idempotency_key: Uuid::new_v4().to_string(),
            booking: Booking {
                id: None,
                all_day: None,
                appointment_segments: Some(vec![AppointmentSegment {
                    duration_minutes: 60.00,
                    team_member_id: "TMKFnToW8ByXrcm6".to_string(),
                    any_team_member_id: None,
                    intermission_minutes: None,
                    resource_ids: None,
                    service_variation_id: "BSOL4BB6RCMX6SH4KQIFWZDP".to_string(),
                    service_variation_version:  1655427266071,
                }]),
                created_at: None,
                booking_creator_details: None,
                customer_id: Some("7PB8P9553RYA3F672D15369VK4".to_string()),
                customer_note: None,
                location_id: Some("L1JC53TYHS40Z".to_string()),
                location_type: None,
                seller_note: None,
                source: None,
                start_at: Some("2022-10-11T16:30:00Z".to_string()),
                status: None,
                transition_time_minutes: None,
                updated_at: None,
                version: None
            }
        };

        let res = sut.create_booking(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_bookings_cancel_builder() {
        let expected = BookingsCancel {
            booking_id: "9uv6i3p5x5ao1p".to_string(),
            body: Some(BookingsCancelBody {
                idempotency_key: Some(Uuid::new_v4().to_string()),
                booking_version: None
            })
        };
        let actual = BookingsCancelBuilder::new()
            .booking_id("9uv6i3p5x5ao1p".to_string()).build().await;

        assert!(actual.is_ok());
        assert_eq!(format!("{:?}", expected.booking_id),
                   format!("{:?}", actual.unwrap().booking_id));
    }

    #[actix_rt::test]
    async fn test_bookings_cancel_builder_fail() {

        let res = BookingsCancelBuilder::new().build().await;

        assert!(res.is_err());
    }

    #[actix_rt::test]
    async fn test_cancel_booking() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = BookingsCancel {
            booking_id: "pi7kr2va3y4h4f".to_string(),
            body: Some(BookingsCancelBody {
                idempotency_key: Some(Uuid::new_v4().to_string()),
                booking_version: None
            })
        };

        let res = sut.cancel_booking(input).await;

        assert!(res.is_ok())
    }
}

