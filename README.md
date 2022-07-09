# Assignment_02_Machrush
This assignment is about test the data from RestAPI, with structure like this.

https://jsonplaceholder.typicode.com<br/>
__ /posts<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 100}<br/>
__ /comments<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 500}<br/>
__ /albums<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 100}<br/>
__ /photos<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 5000}<br/>
__ /todos<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 200}<br/>
__ /users<br/>
_____ /1<br/>
_____ /2<br/>
_____ /{n max 10}<br/>

## RestAPI Route
### GET
This route is about getting all data on RestAPI, but before the data shown, this route check all data.
### GET by ID
This route is about getting data on RestAPI by ID, so this return one data only, but before the data shown, this route check all data.
### DELETE
This route is about delete data on RestAPI by ID, so this return null data, but before the null data shown, this route check all data.
### POST
This route is about posting data on RestAPI, so this return one data only, but before the data shown, this route check all data.
