use actix_web::{get};

#[get("/")]
pub async fn Rping() -> String {
    format!("Pong!")
}
