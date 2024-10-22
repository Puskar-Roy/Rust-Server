use crate::auth::generate_jwt;
use actix_web::{web, HttpResponse, Error};
use bcrypt::{hash, verify, DEFAULT_COST};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use actix_web::error::ErrorInternalServerError; 

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

// Register user
pub async fn register_user(
    pool: web::Data<Pool>,
    form: web::Json<RegisterData>,
) -> Result<HttpResponse, Error> {
    let client = pool.get().await.map_err(|e| {
        eprintln!("Error getting pool: {:?}", e);
        ErrorInternalServerError("Failed to get DB connection")
    })?;

    let hashed_password = hash(&form.password, DEFAULT_COST)
        .map_err(|e| {
            eprintln!("Error hashing password: {:?}", e);
            ErrorInternalServerError("Failed to hash password")
        })?;

    let result = client
        .execute(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
            &[&form.username, &form.email, &hashed_password],
        )
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);  // Log the actual error here
            Err(ErrorInternalServerError("Failed to execute query"))
        }
    }
}


// Login user
pub async fn login_user(
    pool: web::Data<Pool>,
    form: web::Json<LoginData>,
) -> Result<HttpResponse, Error> {
    let client = pool.get().await.map_err(|e| {
        eprintln!("Error getting pool: {:?}", e);
        ErrorInternalServerError("Failed to get DB connection")
    })?;

    let stmt = client
        .prepare("SELECT * FROM users WHERE email = $1")
        .await
        .map_err(|_| ErrorInternalServerError("Failed to prepare query"))?;

    let rows = client
        .query(&stmt, &[&form.email])
        .await
        .map_err(|_| ErrorInternalServerError("Failed to query user"))?;

    if let Some(row) = rows.first() {
        let stored_password: String = row.get("password");
        let valid = verify(&form.password, &stored_password)
            .map_err(|_| ErrorInternalServerError("Failed to verify password"))?;

        if valid {
            let token = generate_jwt(&form.email).map_err(|_| ErrorInternalServerError("Failed to generate token"))?;
            return Ok(HttpResponse::Ok().json(AuthResponse { token }));
        }
    }

    Ok(HttpResponse::Unauthorized().finish())
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register_user)))
       .service(web::resource("/login").route(web::post().to(login_user)));
}
