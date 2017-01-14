# trip_info_back
A collection of systems which serve as the backend for Trip Info. It currently consists of the following:

* API: The entry point into the system.
* DB: Stores a person's average speed and pedometer history.

## API
Interface between mobile clients and other backend systems. All requests are made in HTTPS. The endpoints are as follows:

* NewUUID GET: Generate a unique identifier for each device, assuming 1 device per user
* Pedometer POST: Pass all historical pedometer to DB
* Pedometer PUT: Add new pedometer record at 12am
* Journey/Home POST: Pass in home address and chosen PT stop
* Journey/Destination POST: Pass in destination type, destination address and chosen PT stop
* Journey/Start PUT: Start journey
* Journey/Stop PUT: Stop journey

## DB
Postgres was chosen for its performance, and the fact that it does graph storage and GIS handling. 

To run:
* Make sure `postgres` is not running already, if it is, run `killall postgres` in macOS.
* Run `docker build -t db:db ./ && docker run -p "5432:5432" db:db`.