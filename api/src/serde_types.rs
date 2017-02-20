#[derive(Serialize, Deserialize, Debug)]
pub struct UUID {
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HomeInfoAdd {
    pub uuid: String,
    pub home_address_text: String,
    pub home_address_lat: String,
    pub home_address_long: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DestinationInfoAdd {
    pub uuid: String,
    pub destination_address_text: String,
    pub destination_address_lat: String,
    pub destination_address_long: String,
}
