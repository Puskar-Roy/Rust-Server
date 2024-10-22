use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: String,
}

async fn get_json_data() -> HttpResponse {
    let response = Person {
        name: "Good!".to_string(),
        age: "21".to_string(),
    };

    HttpResponse::Ok().json(response)
}