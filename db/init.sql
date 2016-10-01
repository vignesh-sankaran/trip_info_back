DROP TABLE IF EXISTS user_info;

CREATE TABLE user_info (
	uuid              string,
	home              string,
	home_stop         string,
	destination       string,
	destination_stop  string,
	PRIMARY_KEY(uuid)
);
