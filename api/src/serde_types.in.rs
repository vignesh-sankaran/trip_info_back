#[derive(Serialize)]
struct UUID
{
    uuid: String,
}

#[derive(Deserialize, Clone)]
struct HomeInfoAdd
{
    uuid: String,
    home_address_text: String,
    home_address_lat: String,
    home_address_long: String,
}

#[derive(Deserialize, Clone)]
struct DestinationInfoAdd
{
    uuid: String,
    destination_address_text: String,
    destination_address_lat: String,
    destination_address_long: String,
}