# square-ox

---

A crate for accessing the [Square API](https://developer.squareup.com) Rest endpoint in an idiomatic manner.<br/>
<br/>
Using this crate you are able to formulate standard queries to the [Square API](https://developer.squareup.com) without
worrying about the underlying workflows and specific JSON schemas.
<br/>
<br/>
*A [square-rs](https://github.com/KyleCotton/square-rs) fork*

## Examples

---

### Setup and Authentication with the API
```rust
use square_ox::client::SquareClient;
let client = SquareClient::new("your_access_token");
```

### Sending Requests
```rust
// listing all locations
let locations = client.locations().list().await?;


// retrieving the count of an item in the inventory
let count = client.inventory()
.retrieve_count(
"some_obj_id".to_string(),
Some("some_loc_id".to_string())
)
.await()?;

// registering a new booking
client.bookings().create(
    Builder::from(BookingsPost::default())
    .start_at("2022-10-11T16:30:00Z".to_string())
    .customer_id("some_id".to_string())
    .add_appointment_segment(AppointmentSegment {
    duration_minutes: 60.00,
    team_member_id: "some_id".to_string(),
    any_team_member_id: None,
    intermission_minutes: None,
    resource_ids: None,
    service_variation_id: "some_id".to_string(),
    service_variation_version:  1655427266071,
    })
    .build()
    .await?
).await()?;

```