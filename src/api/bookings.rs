/*!
Bookings functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, SearchQueryBuildError,
                    BookingsPostBuildError, BookingsCancelBuildError};
use crate::response::SquareResponse;
use crate::objects::{AppointmentSegment, Booking, FilterValue,
                     enums::BusinessAppointmentSettingsBookingLocationType};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl SquareClient {
    pub fn bookings(&self) -> Bookings {
        Bookings {
            client: &self
        }
    }
}

pub struct Bookings<'a> {
    client: &'a SquareClient,
}

impl<'a> Bookings<'a> {
    /// Search for availability with the given [SearchQuery](SearchQuery) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `search_query` - A vector of search query parameter created through the
    /// [ListBookingsQueryBuilder](ListBookingsQueryBuilder)
    pub async fn list(self, search_query: Option<Vec<(String, String)>>)
                               -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Bookings("".to_string()),
            None::<&BookingsPost>,
            search_query,
        ).await
    }

    /// Search for availability with the given [SearchQuery](SearchQuery) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `search_query` - A [SearchQuery](SearchQuery) created from the
    /// [SearchAvailabilityQueryBuilder](SearchAvailabilityQueryBuilder)
    pub async fn search_availability(self, search_query: SearchAvailabilityQuery)
                                     -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Bookings("/availability/search".to_string()),
            Some(&search_query),
            None,
        ).await
    }

    /// Create a booking with the given [BookingsPost](BookingsPost) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `create_booking` - A [BookingsPost](BookingsPost) created from the
    /// [BookingsBuilder](BookingsBuilder)
    pub async fn create(self, booking_post: BookingsPost)
                                -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Bookings("".to_string()),
            Some(&booking_post),
            None,
        ).await
    }

    /// Update a booking with the given [BookingsPost](BookingsPost) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `updated_booking` - A [BookingsPost](BookingsPost) created from the
    /// [BookingsBuilder](BookingsBuilder)
    pub async fn update(self, updated_booking: BookingsPost, booking_id: String)
                                -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::PUT,
            SquareAPI::Bookings(format!("/{}", booking_id)),
            Some(&updated_booking),
            None,
        ).await
    }

    /// Retrieve an existing booking from the Square API.
    ///
    /// # Arguments
    /// * `booking_id` - The id of the booking as a String
    pub async fn retrieve(self, booking_id: String)
                                  -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Bookings(format!("/{}", booking_id)),
            None::<&BookingsPost>,
            None,
        ).await
    }

    /// Create a booking with the given [Bookings](Bookings) to the Square API
    /// and get the response back.
    ///
    /// # Arguments
    /// * `booking_to_cancel` - A [BookingsCancel](BookingsCancel) created from the
    /// [BookingsCancelBuilder](BookingsCancelBuilder)
    pub async fn cancel(&self, booking_to_cancel: BookingsCancel)
                                -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Bookings(format!("/{}/cancel",
                                        booking_to_cancel.booking_id.clone())),
            Some(&booking_to_cancel.body),
            None,
        ).await
    }

    /// Retrieves a seller's booking profile at the [Square API](https://developer.squareup.com).
    pub async fn retrieve_business_profile(self)
                                                   -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Bookings("/business-booking-profile".to_string()),
            None::<&BookingsPost>,
            None,
        ).await
    }

    /// Lists booking profiles for team members at the [Square API](https://developer.squareup.com).
    ///
    /// # Arguments
    /// * `search_query` - A search query created by the
    /// [ListTeamMemberBookingsProfileBuilder](ListTeamMemberBookingsProfileBuilder).
    pub async fn list_team_member_profiles(self, search_query: Option<Vec<(String, String)>>)
                                                   -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Bookings("/team-member-booking-profiles".to_string()),
            None::<&BookingsPost>,
            search_query,
        ).await
    }

    /// Lists booking profiles for team members at the [Square API](https://developer.squareup.com).
    ///
    /// # Arguments
    /// * `team_member_id` - The id of the team member you would like to retrieve from the
    /// [Square API](https://developer.squareup.com).
    pub async fn retrieve_team_member_profiles(self, team_member_id: String)
                                                       -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Bookings(format!("/team-member-booking-profiles/{}", team_member_id)),
            None::<&BookingsPost>,
            None,
        ).await
    }
}

#[derive(Default)]
pub struct ListBookingsQueryBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    team_member_id: Option<String>,
    location_id: Option<String>,
    start_at_min: Option<String>,
    start_at_max: Option<String>,
}

impl ListBookingsQueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// The maximum number of results per page to return in a paged response.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);

        self
    }

    /// The pagination cursor from the preceding response to return the next page of the results.
    /// Do not set this when retrieving the first page of the results.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    /// The team member for whom to retrieve bookings.
    /// If this is not set, bookings of all members are retrieved.
    pub fn team_member_id(mut self, team_member_id: String) -> Self {
        self.team_member_id = Some(team_member_id);

        self
    }

    /// The location for which to retrieve bookings.
    /// If this is not set, all locations' bookings are retrieved.
    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);

        self
    }

    /// The RFC 3339 timestamp specifying the earliest of the start time.
    /// If this is not set, the current time is used.
    //
    // Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    //
    // UTC: 2020-01-26T02:25:34Z
    //
    // Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub fn start_at_min(mut self, start_at_min: String) -> Self {
        self.start_at_min = Some(start_at_min);

        self
    }

    /// The RFC 3339 timestamp specifying the latest of the start time.
    /// If this is not set, the time of 31 days after start_at_min is used.
    //
    // Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    //
    // UTC: 2020-01-26T02:25:34Z
    //
    // Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub fn start_at_max(mut self, start_at_max: String) -> Self {
        self.start_at_max = Some(start_at_max);

        self
    }

    pub async fn build(mut self) -> Vec<(String, String)> {
        let ListBookingsQueryBuilder {
            limit,
            cursor,
            team_member_id,
            location_id,
            start_at_min,
            start_at_max,

        } = self;

        let mut res = vec![];

        if let Some(limit) = limit {
            res.push(("limit".to_string(), limit.to_string()))
        }

        if let Some(cursor) = cursor {
            res.push(("cursor".to_string(), cursor))
        }

        if let Some(team_member_id) = team_member_id {
            res.push(("team_member_id".to_string(), team_member_id))
        }

        if let Some(location_id) = location_id {
            res.push(("location_id".to_string(), location_id))
        }

        if let Some(start_at_min) = start_at_min {
            res.push(("start_at_min".to_string(), start_at_min))
        }

        if let Some(start_at_max) = start_at_max {
            res.push(("start_at_max".to_string(), start_at_max))
        }

        res
    }
}

#[derive(Default)]
pub struct ListTeamMemberBookingsProfileBuilder {
    limit: Option<i32>,
    cursor: Option<String>,
    bookable_only: Option<bool>,
    location_id: Option<String>,
}

impl ListTeamMemberBookingsProfileBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// The maximum number of results to return in a paged response.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);

        self
    }

    /// The pagination cursor from the preceding response to return the next page of the results.
    /// Do not set this when retrieving the first page of the results.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    /// Indicates whether to include only bookable team members in the returned result.
    pub fn bookable_only(mut self) -> Self {
        self.bookable_only = Some(true);

        self
    }

    /// Indicates whether to include only team members enabled at the given location in the
    /// returned result.
    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);

        self
    }

    pub async fn build(mut self) -> Vec<(String, String)> {
        let ListTeamMemberBookingsProfileBuilder {
            limit,
            cursor,
            bookable_only,
            location_id,
        } = self;

        let mut res = vec![];

        if let Some(limit) = limit {
            res.push(("limit".to_string(), limit.to_string()))
        }
        if let Some(cursor) = cursor {
            res.push(("cursor".to_string(), cursor))
        }
        if let Some(bookable_only) = bookable_only {
            res.push(("bookable_only".to_string(), bookable_only.to_string()))
        }
        if let Some(location_id) = location_id {
            res.push(("location_id".to_string(), location_id))
        }

        res
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookingsPost {
    idempotency_key: String,
    booking: Booking,
}

/// The [BookingsPostBuilder](BookingsPostBuilder) is used to build a valid
/// [BookingsPost](BookingsPost)
///
/// To build a valid BookingPost and to avoid returning a
/// [BookingsPostBuildError](BookingsPostBuildError) one must previously call:
/// * `.customer_id()`
/// * `.location_id()`
/// * `.add_appointment_segment()`
/// * `.start_at()`
///
/// # Example: Build a [BookingPost](BookingPost)
/// ```
/// async {
///     use square_ox::objects::AppointmentSegment;
///     let builder = square_ox::api::bookings::BookingsPostBuilder::new()
///     .customer_id("some_id".to_string())
///     .location_id("some_id".to_string())
///     .start_at("some_start_at_date_time".to_string())
///     .add_appointment_segment(AppointmentSegment::default())
///     .build()
///     .await;
/// };
/// ```
#[derive(Default)]
pub struct BookingsPostBuilder(Booking);

impl BookingsPostBuilder {
    /// Build a new [BookingsPost](BookingsPost) using the
    /// [BookingsPostBuilder](BookingsPostBuilder)
    ///
    /// # Example: Create a new client
    /// ```
    /// let builder = square_ox::api::bookings::BookingsPostBuilder::new();
    /// ```
    pub fn new() -> Self {
        let mut booking = Booking::default();
        booking.appointment_segments = Some(vec![]);
        Self(booking)
    }


    /// Add a customer_id
    ///
    /// # Arguments:
    /// * `customer_id` - The id of your booking customer.
    ///
    /// # Example: Set the customer id
    /// ```
    /// let builder = square_ox::api::bookings::BookingsPostBuilder::new()
    /// .customer_id("some_id".to_string());
    /// ```
    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.0.customer_id = Some(customer_id);

        self
    }

    // Add a location_id
    ///
    /// # Arguments:
    /// * `location_id` - The id of the booking location.
    ///
    /// # Example: Set the customer id
    /// ```
    /// let builder = square_ox::api::bookings::BookingsPostBuilder::new()
    /// .location_id("some_id".to_string());
    /// ```
    pub fn location_id(mut self, location_id: String) -> Self {
        self.0.location_id = Some(location_id);

        self
    }

    pub fn location_type(mut self, location_type: BusinessAppointmentSettingsBookingLocationType) -> Self {
        self.0.location_type = Some(location_type);

        self
    }

    pub fn start_at(mut self, start_at_date_time: String) -> Self {
        self.0.start_at = Some(start_at_date_time);

        self
    }

    pub fn add_appointment_segment(mut self, appointment_segment: AppointmentSegment) -> Self {
        self.0.appointment_segments.as_mut().unwrap().push(appointment_segment);

        self
    }

    pub fn seller_note(mut self, seller_note: String) -> Self {
        self.0.seller_note = Some(seller_note);

        self
    }

    pub fn customer_note(mut self, customer_note: String) -> Self {
        self.0.customer_note = Some(customer_note);

        self
    }
    /// Build a [BookingPost](BookingPost)
    ///
    /// To build a valid BookingPost and to avoid returning a
    /// [BookingsPostBuildError](BookingsPostBuildError) one must previously call:
    /// * `.customer_id()`
    /// * `.location_id()`
    /// * `.add_appointment_segment()`
    /// * `.start_at()`
    ///
    /// # Example: Build a [BookingPost](BookingPost)
    /// ```
    /// async {
    ///     use square_ox::objects::AppointmentSegment;
    ///     let builder = square_ox::api::bookings::BookingsPostBuilder::new()
    ///     .customer_id("some_id".to_string())
    ///     .location_id("some_id".to_string())
    ///     .start_at("some_start_at_date_time".to_string())
    ///     .add_appointment_segment(AppointmentSegment::default())
    ///     .build()
    ///     .await;
    /// };
    /// ```
    pub async fn build(mut self) -> Result<BookingsPost, BookingsPostBuildError> {

        let booking = self.0;

        if booking.customer_id.is_none()
            || booking.location_id.is_none()
            || booking.appointment_segments.as_ref().unwrap().len() < 1
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
    pub fn new() -> Self {
        BookingsCancelBuilder {
            booking_id: None,
            body: Some(BookingsCancelBody {
                idempotency_key: Some(Uuid::new_v4().to_string()),
                booking_version: None
            })
        }
    }

    pub fn booking_id(mut self, booking_id: String) -> Self {
        self.booking_id = Some(booking_id);
        
        self
    }

    pub fn booking_version(mut self, booking_version: i32) -> Self {
        let body = self.body.as_mut().unwrap();
        body.booking_version = Some(booking_version);


        self
    }

    pub async fn build(self) -> Result<BookingsCancel, BookingsCancelBuildError> {
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

// holds a QQuery struct which contains the actual query data, as this is the way it is expected
// by the Square API
#[derive(Serialize, Debug, Deserialize)]
pub struct SearchAvailabilityQuery {
    query: QueryBody,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct QueryBody {
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

/// The [SearchAvailabilityQueryBuilder](SearchAvailabilityQueryBuilder)
#[derive(Default)]
pub struct SearchAvailabilityQueryBuilder {
    start_at_range: Option<StartAtRange>,
    booking_id: Option<String>,
    location_id: Option<String>,
    segment_filters: Option<Vec<SegmentFilter>>
}

impl SearchAvailabilityQueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start_at_range(mut self, start: String, end: String) -> Self {
        self.start_at_range = Some(StartAtRange {
            end_at: end.clone(),
            start_at: start.clone(),
        });

        self
    }

    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);

        self
    }

    pub fn segment_filters(mut self, service_variation_id: String) -> Self {
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

    pub async fn build(&self) -> Result<SearchAvailabilityQuery, SearchQueryBuildError> {
        let start_at_range = match &self.start_at_range {
            Some(sar) => sar.clone(),
            None => return Err(SearchQueryBuildError),
        };

        let booking_id = self.booking_id.clone();
        let location_id = self.location_id.clone();
        let segment_filters = self.segment_filters.clone();

        Ok(SearchAvailabilityQuery {
            query: QueryBody {
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
        let sut = SearchAvailabilityQueryBuilder::new()
            .start_at_range(
                "2022-10-12T07:20:50.52Z".to_string(),
                "2023-10-12T07:20:50.52Z".to_string())
            .location_id("LPNXWH14W6S47".to_string());
        let expected = SearchAvailabilityQuery {
            query: QueryBody {
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

        let input = SearchAvailabilityQueryBuilder::new()
            .start_at_range(
                "2022-09-12T07:20:50.52Z".to_string(),
                "2022-10-12T07:20:50.52Z".to_string())
            .location_id("L1JC53TYHS40Z".to_string())
            .segment_filters("BJHURKYAIAQIDMY267GZNYNW".to_string())
            .build().await.unwrap();

        println!("{:?}", input);
        println!("{:?}", serde_json::to_string(&input).unwrap());

        let result = sut.bookings().search_availability(input).await;

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
                    service_variation_id: "BJHURKYAIAQIDMY267GZNYNW".to_string(),
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

        let res = sut.bookings().create(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_booking() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.bookings()
            .retrieve("burxkwa4ot1ydg".to_string())
            .await;

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

        let res = sut.bookings().cancel(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_update_booking() {
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
                seller_note: Some("be nice!".to_string()),
                source: None,
                start_at: Some("2022-10-11T16:30:00Z".to_string()),
                status: None,
                transition_time_minutes: None,
                updated_at: None,
                version: None
            }
        };

        let res = sut.bookings()
            .update(input, "oruft3c9lh0duq".to_string())
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_list_bookings_query_builder() {
        let expected = vec![
            ("location_id".to_string(), "L1JC53TYHS40Z".to_string()),
            ("start_at_min".to_string(), "2022-09-12T07:20:50.52Z".to_string()),
        ];

        let actual = ListBookingsQueryBuilder::new()
            .location_id("L1JC53TYHS40Z".to_string())
            .start_at_min("2022-09-12T07:20:50.52Z".to_string())
            .build()
            .await;

        assert_eq!(expected, actual)


    }

    #[actix_rt::test]
    async fn test_list_bookings() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![
            ("start_at_min".to_string(), "2022-09-12T07:20:50.52Z".to_string())
        ];

        let res = sut.bookings().list(Some(input)).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_business_booking_profile() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.bookings().retrieve_business_profile().await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_list_team_member_booking_profile_query_builder() {
        let expected = vec![
            ("limit".to_string(), "10".to_string()),
            ("bookable_only".to_string(), "true".to_string()),
            ("location_id".to_string(), "L1JC53TYHS40Z".to_string()),
        ];

        let actual = ListTeamMemberBookingsProfileBuilder::new()
            .bookable_only()
            .limit(10)
            .location_id("L1JC53TYHS40Z".to_string())
            .build()
            .await;

        assert_eq!(expected, actual)


    }

    #[actix_rt::test]
    async fn test_list_team_member_booking_profiles() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![
            ("limit".to_string(), "10".to_string()),
            ("bookable_only".to_string(), "true".to_string()),
            ("location_id".to_string(), "L1JC53TYHS40Z".to_string()),
        ];

        let res = sut.bookings()
            .list_team_member_profiles(Some(input))
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_team_member_booking_profile() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.bookings()
            .retrieve_team_member_profiles("TMKFnToW8ByXrcm6".to_string())
            .await;

        assert!(res.is_ok())
    }
}

