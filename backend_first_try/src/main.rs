mod models;
// use crate::models::sushi;
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use validator::Validate;

#[get("/sushi")]
async fn get_all_sushi() -> impl Responder {
    HttpResponse::Ok().body("sushi availables! ")
}

#[post("/addsushi")]
async fn add_sushi(body: Json<models::AddSushiRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            // request から 値をコピー
            let sushi_name = body.sushi_name.clone();
            HttpResponse::Ok().body(format!("name is {sushi_name}"))
        }
        Err(..) => HttpResponse::Ok().body("sushi name is required!"),
    }
}

#[patch("/updatesushi/{uuid}")]
async fn update_sushi(update_sushi_url: Path<models::UpdateSushiURL>) -> impl Responder {
    let uuid = update_sushi_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("update sushi with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_sushi)
            .service(add_sushi)
            .service(update_sushi)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
