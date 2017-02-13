use super::schema::user_info;

#[derive(Queryable)]
pub struct UserInfo
{
    pub uuid: String,
    pub home_address_text: String,
	pub home_address_lat: String,
	pub home_address_long: String,
	pub destination_address_text: String,
	pub destination_address_lat: String,
	pub destination_address_long: String,
}

#[derive(Insertable)]
#[table_name="user_info"]
pub struct NewUser<'a>
{
    pub uuid: &'a str,
}