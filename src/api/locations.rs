/*!
Customers functionality of the [Square API](https://developer.squareup.com).
 */

// TODO import the necessary mods
use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, LocationsBuildError, LocationBuildError};
use crate::response::SquareResponse;
use crate::objects::{Address, BusinessHours, BusinessHoursPeriod, Coordinates, Location, TaxIds};

use serde::{Deserialize, Serialize};
use crate::objects::enums::{Currency, LocationStatus, LocationType};

impl SquareClient {
    pub async fn list_locations(&self) -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Locations("".to_string()),
            None::<&Location>,
            None,
        ).await
    }

    pub async fn create_location(&self, new_location: LocationCreationBody)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::POST,
            SquareAPI::Locations("".to_string()),
            Some(&new_location),
            None,
        ).await
    }

    pub async fn update_location(&self, updated_location: LocationCreationBody, location_id: String)
        -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::PUT,
            SquareAPI::Locations(format!("/{}", location_id)),
            Some(&updated_location),
            None,
        ).await
    }

    pub async fn retrieve_location(&self, location_id: String)
                                 -> Result<SquareResponse, SquareError> {
        self.request(
            Verb::GET,
            SquareAPI::Locations(format!("/{}", location_id)),
            None::<&LocationCreationBody>,
            None,
        ).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LocationCreationBody {
    location: Location
}

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
    fn new() -> Self {
        Default::default()
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);

        self
    }

    fn address(mut self, address: Address) -> Self {
        self.address = Some(address);

        self
    }

    fn business_email(mut self, business_email: String) -> Self {
        self.business_email = Some(business_email);

        self
    }

    fn add_business_hours_period(mut self, business_hours_period: BusinessHoursPeriod) -> Self {
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

    fn business_hours(mut self, business_hours: BusinessHours) -> Self {
        self.business_hours = Some(business_hours);

        self
    }

    fn business_name(mut self, business_name: String) -> Self {
        self.business_name = Some(business_name);

        self
    }

    fn add_capability(mut self, capability: String) -> Self {
        match self.capabilities.take() {
            Some(mut capabilities) => {
                capabilities.push(capability);
                self.capabilities = Some(capabilities)
            }
            None => self.capabilities = Some(vec![capability]),
        }

        self
    }

    fn capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = Some(capabilities);

        self
    }

    fn coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);

        self
    }

    fn country(mut self, country: String) -> Self {
        self.country = Some(country);

        self
    }

    fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);

        self
    }

    fn description(mut self, description: String) -> Self {
        self.description = Some(description);

        self
    }

    fn facebook_url(mut self, facebook_url: String) -> Self {
        self.facebook_url = Some(facebook_url);

        self
    }

    fn full_format_logo_url(mut self, full_format_logo_url: String) -> Self {
        self.full_format_logo_url = Some(full_format_logo_url);

        self
    }

    fn instagram_username(mut self, instagram_username: String) -> Self {
        self.instagram_username = Some(instagram_username);

        self
    }

    fn language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);

        self
    }

    fn logo_url(mut self, logo_url: String) -> Self {
        self.logo_url = Some(logo_url);

        self
    }

    fn mcc(mut self, mcc: String) -> Self {
        self.mcc = Some(mcc);

        self
    }

    fn merchant_id(mut self, merchant_id: String) -> Self {
        self.merchant_id = Some(merchant_id);

        self
    }

    fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);

        self
    }

    fn pos_background_url(mut self, pos_background_url: String) -> Self {
        self.pos_background_url = Some(pos_background_url);

        self
    }

    fn status(mut self, status: LocationStatus) -> Self {
        self.status = Some(status);

        self
    }

    fn tax_ids(mut self, tax_ids: TaxIds) -> Self {
        self.tax_ids = Some(tax_ids);

        self
    }

    fn timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);

        self
    }

    fn twitter_username(mut self, twitter_username: String) -> Self {
        self.twitter_username = Some(twitter_username);

        self
    }

    fn location_type(mut self, location_type: LocationType) -> Self {
        self.type_name = Some(location_type);

        self
    }

    fn website_url(mut self, website_url: String) -> Self {
        self.website_url = Some(website_url);

        self
    }

    pub async fn build(mut self) -> Result<LocationCreationBody, LocationBuildError> {
        if let Some(name) = self.name.take() {
            Ok( LocationCreationBody {
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
    async fn test_create_location_request() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);
        let result = sut.list_locations().await;
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
            type_name: Some(LocationType::PHYSICAL),
            business_hours: None,
            business_name: None,
            website_url: None
        };
        let actual = LocationBuilder::new()
            .name("New Test Location".to_string())
            .facebook_url("some_url".to_string())
            .location_type(LocationType::PHYSICAL)
            .build()
            .await;

        assert!(actual.is_ok());

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap().location))
    }

    #[actix_rt::test]
    async fn test_location_builder_fail() {
        let res = LocationBuilder::new()
            .facebook_url("some_url".to_string())
            .location_type(LocationType::PHYSICAL)
            .build()
            .await;

        assert!(res.is_err());
    }

    #[actix_rt::test]
    async fn test_create_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = LocationCreationBody{
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
                type_name: Some(LocationType::PHYSICAL),
                business_hours: None,
                website_url: None,
                business_email: None
            }
        };

        let res = sut.create_location(input).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_update_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = LocationCreationBody {
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
                type_name: Some(LocationType::PHYSICAL),
                business_hours: None,
                business_name: None,
                website_url: Some("example-website.com".to_string())
            }
        };

        let res = sut.update_location(input,
                                      "LBQ9DAD5WCHB0".to_string()).await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_location() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res =
            sut.retrieve_location("LBQ9DAD5WCHB0".to_string()).await;

        assert!(res.is_ok())
    }
}