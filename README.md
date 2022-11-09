Base for general use Actix API. It provides a file structure that allows You to quickly start working on a new project.  
This base project uses [`MongoDB`](https://www.mongodb.com/).

## Important
`.env` file must be added:
```
SERVER.HOST = 127.0.0.1
SERVER.PORT = 8080
MONGO.URI = <mongo_uri>
MONGO.DB = <db_name>
```

# Crate structure

### models
Contains Objects representing documents from your `MongoDB` collections.  

### mongo
Contains ['DB'](mongo/src/lib.rs) object on which [`services`](mongo/src/services.rs) are implemented. They're handling all database related bussiness logic.  
This base project has services for adding, getting, updating and deleting data from `MongoDB`.

### server
Contains [`main`](server/src/main.rs) binary file that runs Actix's [`HttpServer`](https://actix.rs/docs/server/).  
[`config.rs`](server/src/config.rs) loads variables from `.env` file.  
[`routes.rs`](server/src/routes.rs) contains modules with [handlers](https://actix.rs/docs/handlers/). They're meant to assign `services` (e.g. like the ones from `mongo` crate) to endpoints and handle exceptions.

### static
Contains all `static` recsources that'll be exposed to client.   