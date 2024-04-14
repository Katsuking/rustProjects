mod db;
mod errors;
mod models;
use crate::models::sushi::{AddSushiRequest, Sushi, UpdateSushiURL};
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::Database;
use errors::SushiError;
use validator::Validate;

#[get("/sushi")]
async fn get_all_sushi(db: Data<Database>) -> Result<Json<Vec<Sushi>>, SushiError> {
    let sushi = db.get_all_sushi().await;
    match sushi {
        Some(found_sushi) => Ok(Json(found_sushi)),
        None => Err(SushiError::NoSushiFound),
    }
}

#[post("/addsushi")]
async fn add_sushi(body: Json<AddSushiRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate(); // validation

    match is_valid {
        Ok(_) => {
            // request から 値をコピー
            let sushi_name = body.sushi_name.clone();
            // create a new uuid
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_sushi = db
                .add_new_sushi(Sushi::new(String::from(new_uuid), sushi_name))
                .await;
            match new_sushi {
                Some(s) => {
                    println!("{:?}", s);
                    HttpResponse::Ok().body(format!("Added new sushi: {:?}", s))
                }
                None => HttpResponse::Ok().body("Failed to add new sushi..."),
            }
        }
        Err(..) => HttpResponse::Ok().body("sushi name is required!"),
    }
}

#[post("/createsushi")]
async fn create_sushi(
    body: Json<AddSushiRequest>,
    db: Data<Database>,
) -> Result<Json<Sushi>, SushiError> {
    let is_valid = body.validate(); // validation

    match is_valid {
        Ok(_) => {
            // request から 値をコピー
            let sushi_name = body.sushi_name.clone();
            // create a new uuid
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_sushi = db
                .add_new_sushi(Sushi::new(String::from(new_uuid), sushi_name))
                .await;
            match new_sushi {
                Some(s) => {
                    println!("{:?}", s);
                    Ok(Json(s))
                }
                None => Err(SushiError::SushiCreationFailure),
            }
        }
        Err(..) => Err(SushiError::SushiCreationFailure),
    }
}

#[patch("/updatesushi/{uuid}")]
async fn update_sushi(update_sushi_url: Path<UpdateSushiURL>) -> impl Responder {
    let uuid = update_sushi_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("update sushi with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("Failed to connect to db");

    // from actic-web: to wrap data layer
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_all_sushi)
            .service(add_sushi)
            .service(create_sushi)
            .service(update_sushi)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
