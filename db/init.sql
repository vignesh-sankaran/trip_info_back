DROP DATABASE IF EXISTS trip_info;

CREATE DATABASE trip_info;
\connect trip_info

CREATE TABLE user_info (
	uuid text PRIMARY KEY,
	home_address_text text,
	home_address_lat text,
	home_address_long text,
	destination_address_text text,
	destination_address_lat text,
	destination_address_long text
);