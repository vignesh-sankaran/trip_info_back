var conn = new Mongo("0.0.0.0", 27017);
var database = db.getSiblingDB('tripinfo');
database.createCollection('users', { id: 'string' });
db.createUser({ user: 'system', pwd: 'system', roles: [ { role: 'dbAdminAnyDatabase', db: 'admin' } ] });

cursor = db.collection.find();
while ( cursor.hasNext() ) {
   printjson( cursor.next() );
}
