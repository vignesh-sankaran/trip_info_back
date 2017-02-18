extern crate iron;
extern crate trip_info_api_lib;

fn main() {
    trip_info_api_lib::start_server(std::net::Ipv4Addr::new(0, 0, 0, 0), 20000);
}
