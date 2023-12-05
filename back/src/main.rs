use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use serde::Serialize;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, serde::Deserialize)]
struct RequestData {
    data: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub second_name: String,
}

async fn create(person: Person, pool: &sqlx::PgPool) -> Result<(), actix_web::Error> {
    let query = "INSERT INTO person (name, surname, second_name) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&person.name)
        .bind(&person.surname)
        .bind(&person.second_name)
        .execute(pool)
        .await
        .map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Failed to execute query: {}", err))
        })?;

    log::info!("Sent SUCCESSFULLY");
    Ok(())
}

fn parse_input(parts: Vec<String>) -> Result<Person, &'static str> {
    if parts.len() != 3 {
        return Err("Invalid input format. Expected: name surname second_name");
    }

    Ok(Person {
        name: parts[0].to_string(),
        surname: parts[1].to_string(),
        second_name: parts[2].to_string(),
    })
}

async fn call_function(data: web::Json<RequestData>, pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, actix_web::Error> {
    // Process data, e.g., save to the database
    println!("Received data: {:?}", data);

    let person_data = match parse_input(data.data.clone()) {
        Ok(res) => res,
        Err(err) => panic!("{err}")
    };

    match create(person_data, &pool).await {
        Ok(_) => HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Created Successfully",
    })),
        Err(err) => {
            log::info!("Error creating record: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
            "error": "Internal Server Error Creating Object",
        }))
        }
    };

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:8080");

    // Establish a database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect( "postgres://postgres:12345@localhost:5432/postgres")
        .await
        .expect("Failed to connect to the database.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:63342")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(&[
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .expose_headers(&[http::header::CONTENT_DISPOSITION])
                    .block_on_origin_mismatch(false)
                    .max_age(3600)
            )
            .wrap(Logger::default())
            .service(web::resource("/call_function").route(web::post().to(call_function)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
