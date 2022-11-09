use actix_web::web;

/// Function registering all Routes from `routes.rs`
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(pages::index)
    // scoped enpoints
    .service(
        web::scope("/todo")
            .service(todo::all)
            .service(todo::get)
            .service(todo::post_get)
            .service(todo::add)
            .service(todo::update)
            .service(todo::delete)
    );
}


//==========| ROUTES |=========
/// Module containing routes for handling html pages 
mod pages {
    use actix_files::NamedFile;
    use std::path::PathBuf;
    use actix_web::{get, Error};

    // ===========| GET |===========
    #[get("/")]
    async fn index() -> Result<NamedFile, Error> {
        let path: PathBuf = "./static/html/hello.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
    }
}

/// Module containing routes for handling 'Todo' 
mod todo {
    use actix_web::{HttpResponse, get, post, Responder, web};
    use models::Todo;
    use mongo::DB;
    use bson::Document;

    // ===========| GET |===========
    #[get("/all")]
    async fn all(db: web::Data<DB>) -> impl Responder {
        HttpResponse::Ok()
            .json(db.get_all_todo().await.unwrap())
    }

    #[get("/get/{id}")] 
    async fn get(path: web::Path<String>, db: web::Data<DB>) -> impl Responder {
        // get id from path
        let id = path.into_inner();
        
        // get Todo from database using id
        match db.get_todo(&id).await {
            Ok(Some(todo)) =>
                HttpResponse::Ok().json(todo),
            Ok(_) => 
                HttpResponse::NotFound().json("Todo with given id not found"),
            Err(_) => 
                HttpResponse::InternalServerError().json("Error occured while getting todo from database")
        }
    }

    // ===========| POST |===========

    #[post("/get")] 
    async fn post_get(id: web::Json<Document>, db: web::Data<DB>) -> impl Responder {
        // get id from req body
        if let Ok(id) = id.get_str("id") {
            // get Todo from database using id
            match db.get_todo(id).await {
                Ok(Some(todo)) => HttpResponse::Ok().json(todo),
                Ok(_) => HttpResponse::NotFound().json("Todo with given id not found"),
                Err(_) => HttpResponse::InternalServerError().json("Error occured while getting todo from database")
            }
        } else {
            HttpResponse::BadRequest().json("No id given")
        }
    }
    
    #[post("/add")] 
    async fn add(todo: web::Json<Todo>, db: web::Data<DB>) -> impl Responder {
        HttpResponse::Ok()
            .json(db.add_todo(&todo.0).await.unwrap())
    }

    #[post("/update")] 
    async fn update(data: web::Json<Document>, db: web::Data<DB>) -> impl Responder {
        match (
            data.get_str("id"), 
            data.get_str("title"), 
            data.get_str("description")
        ) {
            // check if all fields needed were given
            (Ok(id), Ok(title), Ok(description)) =>
                match db.update_todo(id, title, description).await {
                    Ok(true) => HttpResponse::Ok().json("Updated successfully"),
                    Ok(false) => HttpResponse::NotFound().json("Todo with given id not found"),
                    Err(_) => HttpResponse::InternalServerError().json("Error occured while getting todo from database")
                }
            _ => HttpResponse::BadRequest().json("missing fields: id, title or description")
        }

       
    }

    #[post("/delete")] 
    async fn delete(id: web::Json<Document>, db: web::Data<DB>) -> impl Responder {
        // get id from req body
        if let Ok(id) = id.get_str("id") {
            // delete todo
            match db.delete_todo(id).await {
                Ok(true) => HttpResponse::Ok().json("Deleted successfully"),
                Ok(false) => HttpResponse::NotFound().json("Todo with given id not found"),
                Err(_) => HttpResponse::InternalServerError().json("Error occured while deleting todo from database")
            }
        } else {
            HttpResponse::BadRequest().json("No id given")
        } 
    }
}