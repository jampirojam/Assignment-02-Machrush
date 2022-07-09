# Assignment-02-Machrush
This assignment is about test the data from RestAPI, with structure like this.

https://jsonplaceholder.typicode.com
-- /posts
----- /1
----- /2
----- /{n max 100}
-- /comments
----- /1
----- /2
----- /{n max 500}
-- /albums
----- /1
----- /2
----- /{n max 100}
-- /photos
----- /1
----- /2
----- /{n max 5000}
-- /todos
----- /1
----- /2
----- /{n max 200}
-- /users
----- /1
----- /2
----- /{n max 10}

## RestAPI Route
### GET
This route is about getting all data on RestAPI, but before the data shown, this route check all data.
### GET by ID
This route is about getting data on RestAPI by ID, so this return one data only, but before the data shown, this route check all data.
### DELETE
This route is about delete data on RestAPI by ID, so this return null data, but before the null data shown, this route check all data.
### POST
This route is about posting data on RestAPI, so this return one data only, but before the data shown, this route check all data.
