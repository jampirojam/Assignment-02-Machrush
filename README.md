# Assignment-02-Machrush
This assignment is about test the data from RestAPI, with structure like this.

https://jsonplaceholder.typicode.com<br/>
-- /posts<br/>
----- /1<br/>
----- /2<br/>
----- /{n max 100}<br/>
-- /comments<br/>
----- /1<br/>
----- /2<br/>
----- /{n max 500}<br/>
-- /albums<br/>
----- /1<br/>
----- /2<br/>
----- /{n max 100}<br/>
-- /photos<br/>
----- /1<br/>
----- /2<br/>
----- /{n max 5000}<br/>
-- /todos<br/>
----- /1<br/><br/>
----- /2<br/>
----- /{n max 200}<br/>
-- /users<br/>
----- /1<br/>
----- /2<br/>
----- /{n max 10}<br/>

## RestAPI Route
### GET
This route is about getting all data on RestAPI, but before the data shown, this route check all data.
### GET by ID
This route is about getting data on RestAPI by ID, so this return one data only, but before the data shown, this route check all data.
### DELETE
This route is about delete data on RestAPI by ID, so this return null data, but before the null data shown, this route check all data.
### POST
This route is about posting data on RestAPI, so this return one data only, but before the data shown, this route check all data.
