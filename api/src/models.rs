#[derive(Queryable)]
pub struct User_Info
{
    pub uuid: String,
    pub home: String,
    pub home_stop: String,
    pub destination: String,
    pub destination_stop: String,
}