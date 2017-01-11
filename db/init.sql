DROP DATABASE IF EXISTS trip_info;

CREATE DATABASE trip_info;
\connect trip_info

CREATE TABLE user_info (
	uuid text PRIMARY KEY not null,
	home_address_text text not null DEFAULT '',
	home_address_lat text not null DEFAULT '',
	home_address_long text not null DEFAULT '',
	destination_address_text text not null DEFAULT '',
	destination_address_lat text not null DEFAULT '',
	destination_address_long text not null DEFAULT ''
);