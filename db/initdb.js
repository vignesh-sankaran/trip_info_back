var conn = new Mongo();
var database = db.getSiblingDB('tripinfo');
database.createCollection('users', { id: 'string' });
db.createUser({ user: 'system', pwd: 'system', roles: [ { role: 'dbAdminAnyDatabase', db: 'auth' } ] });

cursor = db.collection.find();
while ( cursor.hasNext() ) {
   printjson( cursor.next() );
}
