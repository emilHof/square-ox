/*!
Customers functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, LocationsBuildError, LocationBuildError};
use crate::response::SquareResponse;
use crate::objects::{
    Address, BusinessHours, BusinessHoursPeriod, Coordinates, Location, TaxIds,
    enums::{
        Currency, LocationStatus, LocationType
    }
};

use serde::{Deserialize, Serialize};

impl SquareClient {
    pub fn locations(&self) -> Locations {
        Locations {
            client: &self,
        }
    }
}

pub struct Locations<'a> {
    pub client: &'a SquareClient,
}

impl<'a> Locations<'a> {
    /// See which [Location](Location)s are available by requesting the information from the
    /// [Square API](https://developer.squareup.com) and subsequently receiving them formatted as a
    /// list of [Location](Location)s.
    /// # Example
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    /// async {
    ///     let locations = SquareClient::new("some_token")
    ///         .locations()
    ///         .list()
    ///         .await;
    /// };
    /// ```
    pub async fn list(self) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Locations("".to_string()),
            None::<&Location>,
            None,
        ).await
    }

    /// Create a new [Location](Location) at the [Square API](https://developer.squareup.com).
    /// # Arguments
    /// * `new_location` - A [LocationCreationWrapper](LocationCreationWrapper) that is build by the
    /// the [LocationBuilder](LocationBuilder).
    /// # Example
    /// ```rust
    /// use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient,
    ///    api::locations::LocationBuilder
    ///    };
    ///
    ///  async {
    ///     let location = LocationBuilder::new()
    ///         .name("The Foo Bar".to_string())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = SquareClient::new("some_token")
    ///         .locations()
    ///         .create(location)
    ///         .await;
    /// };
    /// ```
    pub async fn create(self, new_location: LocationCreationWrapper)
                                 -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Locations("".to_string()),
            Some(&new_location),
            None,
        ).await
    }

    /// Update an existing [Location](Location) at the [Square API](https://developer.squareup.com).
    /// # Arguments
    /// * `updated_location` - A [LocationCreationWrapper](LocationCreationWrapper) that is build by the
    /// the [LocationBuilder](LocationBuilder).
    /// * `location_id` - The id of the location that is to be updated.
    /// # Example
    /// ```rust
    /// use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient,
    ///    api::locations::LocationBuilder
    ///    };
    ///
    ///  async {
    ///     let location = LocationBuilder::new()
    ///         .name("The New Foo Bar".to_string())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = SquareClient::new("some_token")
    ///         .locations()
    ///         .update(location, "foo_bar_id".to_string())
    ///         .await;
    /// };
    /// ```
    pub async fn update(self, updated_location: LocationCreationWrapper, location_id: String)
                                 -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::PUT,
            SquareAPI::Locations(format!("/{}", location_id)),
            Some(&updated_location),
            None,
        ).await
    }

    /// Retrieve a [Location](Location) from [Square API](https://developer.squareup.com) by the
    /// location id.
    /// # Arguments
    /// * `location_id` - The id of the location that is to be retrieved.
    /// # Example
    /// ```rust
    /// use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    ///  async {
    ///     let res = SquareClient::new("some_token")
    ///         .locations()
    ///         .retrieve("foo_bar_id".to_string())
    ///         .await;
    /// };
    /// ```
    pub async fn retrieve(self, location_id: String)
                                   -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Locations(format!("/{}", location_id)),
            None::<&LocationCreationWrapper>,
            None,
        ).await
    }
}

/// Build a  wrapping a [Location](Location)
///
/// When passing a [Location](Location) to one of the request methods, they almost always must
/// be wrapped within a [LocationCreationWrapper](LocationCreationWrapper) to adhere to the
/// [Square API](https://developer.squareup.com) contract.
///
/// A [Location](Location) must have a name upon creation, otherwise it is not seen as a valid
/// new [Location](Location).
/// * `.name()`
///
/// # Example: Build a [LocationCreationWrapper](LocationCreationWrapper)
/// ```
/// async {
///     let builder = square_rs::api::locations::LocationBuilder::new()
///     .name("The Foo Bar".to_string())
///     .build()
///     .await;
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LocationCreationWrapper {
    location: Location
}

/// The [LocationBuilder](LocationBuilder) facilitates the creation of a [Location](Location) object
/// wrapped in a [LocationCreationWrapper](LocationCreationWrapper).
#[derive(Default)]
pub struct LocationBuilder {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub timezone: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub status: Option<LocationStatus>,
    pub created_id: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub country: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    pub facebook_url: Option<String>,
    pub full_format_logo_url: Option<String>,
    pub logo_url: Option<String>,
    pub instagram_username: Option<String>,
    pub language_code: Option<String>,
    pub mcc: Option<String>,
    pub merchant_id: Option<String>,
    pub phone_number: Option<String>,
    pub pos_background_url: Option<String>,
    pub tax_ids: Option<TaxIds>,
    pub twitter_username: Option<String>,
    pub type_name: Option<LocationType>,
    pub business_hours: Option<BusinessHours>,
    pub business_name: Option<String>,
    pub business_email: Option<String>,
    pub website_url: Option<String>,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);

        self
    }

    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);

        self
    }

    pub fn business_email(mut self, business_email: String) -> Self {
        self.business_email = Some(business_email);

        self
    }

    /// Add individual [BusinessHoursPeriod](BusinessHoursPeriod)'s by the use of this method.
    pub fn add_business_hours_period(mut self, business_hours_period: BusinessHoursPeriod) -> Self {
        match self.business_hours.take() {
            Some(mut business_hours) => {
                business_hours.periods.push(business_hours_period);
                self.business_hours = Some(business_hours);
            }
            None => self.business_hours = Some(BusinessHours {
                periods: vec![business_hours_period]
            })
        }

        self
    }

    /// Add a complete [BusinessHours](BusinessHours) object by using this method.
    pub fn business_hours(mut self, business_hours: BusinessHours) -> Self {
        self.business_hours = Some(business_hours);

        self
    }

    pub fn business_name(mut self, business_name: String) -> Self {
        self.business_name = Some(business_name);

        self
    }

    /// Add an individual *capability* by the use of this method.
    pub fn add_capability(mut self, capability: String) -> Self {
        match self.capabilities.take() {
            Some(mut capabilities) => {
                capabilities.push(capability);
                self.capabilities = Some(capabilities)
            }
            None => self.capabilities = Some(vec![capability]),
        }

        self
    }

    /// Add multiple *capabilities* at once through this method. This method will overwrite all
    /// other *capabilities* that are already held by the [Location](Location) object.
    pub fn capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = Some(capabilities);

        self
    }

    pub fn coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);

        self
    }

    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);

        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);

        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);

        self
    }

    pub fn facebook_url(mut self, facebook_url: String) -> Self {
        self.facebook_url = Some(facebook_url);

        self
    }

    pub fn full_format_logo_url(mut self, full_format_logo_url: String) -> Self {
        self.full_format_logo_url = Some(full_format_logo_url);

        self
    }

    pub fn instagram_username(mut self, instagram_username: String) -> Self {
        self.instagram_username = Some(instagram_username);

        self
    }

    pub fn language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);

        self
    }

    pub fn logo_url(mut self, logo_url: String) -> Self {
        self.logo_url = Some(logo_url);

        self
    }

    pub fn mcc(mut self, mcc: String) -> Self {
        self.mcc = Some(mcc);

        self
    }

    pub fn merchant_id(mut self, merchant_id: String) -> Self {
        self.merchant_id = Some(merchant_id);

        self
    }

    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);

        self
    }

    pub fn pos_background_url(mut self, pos_background_url: String) -> Self {
        self.pos_background_url = Some(pos_background_url);

        self
    }

    pub fn status(mut self, status: LocationStatus) -> Self {
        self.status = Some(status);

        self
    }

    pub fn tax_ids(mut self, tax_ids: TaxIds) -> Self {
        self.tax_ids = Some(tax_ids);

        self
    }

    pub fn timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);

        self
    }

    pub fn twitter_username(mut self, twitter_username: String) -> Self {
        self.twitter_username = Some(twitter_username);

        self
    }

    pub fn location_type(mut self, location_type: LocationType) -> Self {
        self.type_name = Some(location_type);

        self
    }

    pub fn website_url(mut self, website_url: String) -> Self {
        self.website_url = Some(website_url);

        self
    }

    /// Build a [LocationCreationWrapper](LocationCreationWrapper) wrapping a [Location](Location)
    ///
    /// When passing a [Location](Location) to one of the request methods, they almost always must
    /// be wrapped within a [LocationCreationWrapper](LocationCreationWrapper) to adhere to the
    /// [Square API](https://developer.squareup.com) contract.
    ///
    /// A [Location](Location) must have a name upon creation, otherwise it is not seen as a valid
    /// new [Location](Location).
    /// * `.name()`
    ///
    /// # Example: Build a [LocationCreationWrapper](LocationCreationWrapper)
    /// ```
    /// async {
    ///     let builder = square_ox::api::locations::LocationBuilder::new()
    ///     .name("The Foo Bar".to_string())
    ///     .build()
    ///     .await;
    /// };
    /// ```
    pub async fn build(mut self) -> Result<LocationCreationWrapper, LocationBuildError> {
        if let Some(name) = self.name.take() {
            Ok( LocationCreationWrapper {
                location: Location {
                    id: None,
                    name,
                    business_email: None,
                    address: self.address,
                    timezone: self.timezone,
                    capabilities: self.capabilities,
                    status: self.status,
                    created_id: self.created_id,
                    coordinates: self.coordinates,
                    country: self.country,
                    created_at: self.created_at,
                    currency: self.currency,
                    description: self.description,
                    facebook_url: self.facebook_url,
                    full_format_logo_url: self.full_format_logo_url,
                    logo_url: self.logo_url,
                    instagram_username: self.instagram_username,
                    language_code: self.language_code,
                    mcc: self.mcc,
                    merchant_id: self.merchant_id,
                    phone_number: self.phone_number,
                    pos_background_url: self.pos_background_url,
                    tax_ids: self.tax_ids,
                    twitter_username: self.twitter_username,
                    type_name: self.type_name,
                    business_hours: self.business_hours,
                    business_name: self.business_name,
                    website_url: self.website_url
            }})
        } else {
            Err(LocationBuildError)
        }
    }
}


#[cfg(test)]
mod test_locations {
    use super::*;

    #[actix_rt::test]
    async fn test_list_locations() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let result = sut.locations()
            .list()
            .await;
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    async fn test_location_builder() {
        let expected = Location {
            id: None,
            name: "New Test Location".to_string(),
            business_email: None,
            address: None,
            timezone: None,
            capabilities: None,
            status: None,
            created_id: None,
            coordinates: None,
            country: None,
            created_at: None,
            currency: None,
            description: None,
            facebook_url: Some("some_url".to_string()),
            full_format_logo_url: None,
            logo_url: None,
            instagram_username: None,
            language_code: None,
            mcc: None,
            merchant_id: None,
            phone_number: None,
            pos_background_url: None,
            tax_ids: None,
            twitter_username: None,
            type_name: Some(LocationType::Physical),
            business_hours: None,
            business_name: None,
            website_url: None
        };
        let actual = LocationBuilder::new()
            .name("New Test Location".to_string())
            .facebook_url("some_url".to_string())
            .location_type(LocationType::Physical)
            .build()
            .await;

        assert!(actual.is_ok());

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap().location))
    }

    #[actix_rt::test]
    async fn test_location_builder_fail() {
        let res = LocationBuilder::new()
            .facebook_url("some_url".to_string())
            .location_type(LocationType::Physical)
            .build()
            .await;

        assert!(res.is_err());
    }

    // #[actix_rt::test]
    async fn test_create_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = LocationCreationWrapper {
            location: Location {
                id: None,
                name: "New Test Location".to_string(),
                business_name: None,
                address: None,
                timezone: None,
                capabilities: None,
                status: None,
                created_id: None,
                coordinates: None,
                country: None,
                created_at: None,
                currency: None,
                description: None,
                facebook_url: None,
                full_format_logo_url: None,
                logo_url: None,
                instagram_username: None,
                language_code: None,
                mcc: None,
                merchant_id: None,
                phone_number: None,
                pos_background_url: None,
                tax_ids: None,
                twitter_username: None,
                type_name: Some(LocationType::Physical),
                business_hours: None,
                website_url: None,
                business_email: None
            }
        };

        let res = sut.locations()
            .create(input)
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_update_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = LocationCreationWrapper {
            location: Location {
                id: None,
                name: "Updated Test Location".to_string(),
                business_email: None,
                address: None,
                timezone: None,
                capabilities: None,
                status: None,
                created_id: None,
                coordinates: None,
                country: None,
                created_at: None,
                currency: None,
                description: None,
                facebook_url: None,
                full_format_logo_url: None,
                logo_url: None,
                instagram_username: None,
                language_code: None,
                mcc: None,
                merchant_id: None,
                phone_number: None,
                pos_background_url: None,
                tax_ids: None,
                twitter_username: None,
                type_name: Some(LocationType::Physical),
                business_hours: None,
                business_name: None,
                website_url: Some("example-website.com".to_string())
            }
        };

        let res = sut.locations()
            .update(input,"LBQ9DAD5WCHB0".to_string())
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.locations()
            .retrieve("LBQ9DAD5WCHB0".to_string())
            .await;

        assert!(res.is_ok())
    }
}