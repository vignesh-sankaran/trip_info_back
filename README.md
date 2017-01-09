# trip_info_back
A collection of systems which serve as the backend for Trip Info. It consists of the following:

* API: The entry point into the system.
* DB: Stores a person's average speed and pedometer history.
* Journey Manager: Communicates with whatever the ML system will be, and sends out push notifications back to the device.

## API
Interface between mobile clients and other backend systems. All requests are made in HTTPS. The endpoints are as follows:

* NewUUID GET: Generate a unique identifier for each device, assuming 1 device per user
* {Id}/Pedometer POST: Pass all historical pedometer to DB
* {Id}/Pedometer PUT: Add new pedometer record at 12am
* {Id}/Journey/Home POST: Pass in home address and chosen PT stop
* {Id}/Journey/Destination POST: Pass in destination type, destination address and chosen PT stop
* {Id}/Journey/Start PUT: Start journey
* {Id}/Journey/Stop PUT: Stop journey

## DB
Postgres was chosen for its performance, and the fact that it does graph storage and GIS handling. 

To run:
* Make sure `postgres` is not running already, if it is, run `killall postgres` in macOS.
* Run `docker build -t db:db ./ && docker run -p "5432:5432" db:db`.