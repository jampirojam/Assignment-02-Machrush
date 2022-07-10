# Assignment-02-Machrush
This assignment is about test the data from RestAPI, with structure like this.

jsonplaceholder.typicode.com<br/>
______ /posts<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 100}<br/>
______ /comments<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 500}<br/>
______ /albums<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 100}<br/>
______ /photos<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 5000}<br/>
______ /todos<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 200}<br/>
______ /users<br/>
_______________ /1<br/>
_______________ /2<br/>
_______________ /{n max 10}<br/>

## RestAPI Route
### GET
This route is about getting all data on RestAPI, but before the data shown, this route check all data.
### GET by ID
This route is about getting data on RestAPI by ID, so this return one data only, but before the data shown, this route check all data.
### DELETE
This route is about delete data on RestAPI by ID, so this return null data, but before the null data shown, this route check all data.
### POST
This route is about posting data on RestAPI, so this return one data only, but before the data shown, this route check all data.
