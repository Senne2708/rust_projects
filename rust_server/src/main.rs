use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ActivityEntry {
    title: String,
    url: String,
    timestamp: DateTime<Utc>,
}

struct AppState {
    history: Mutex<VecDeque<ActivityEntry>>,
}

async fn record_activity(
    data: web::Json<ActivityEntry>,
    state: web::Data<AppState>,
) {
    // Clone the data before moving it
    let entry = data.into_inner();
    let entry_clone = entry.clone();
    
    let mut history = state.history.lock().unwrap();
    history.push_front(entry);
    
    // Keep only last 1000 entries
    while history.len() > 1000 {
        history.pop_back();
    }
    
    println!("New activity recorded:");
    println!("  Title: {}", entry_clone.title);
    println!("  URL: {}", entry_clone.url);
    println!("  Time: {}", entry_clone.timestamp);
    
    HttpResponse::Ok().json("Activity recorded")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        history: Mutex::new(VecDeque::with_capacity(1000)),
    });

    println!("Starting server on http://localhost:3000");
    
    HttpServer::new(move || {
        let cors = Cors::permissive(); // Configure CORS for development
        
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/activity", web::post().to(record_activity))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
