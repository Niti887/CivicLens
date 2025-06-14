use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// Data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Issue {
    id: String,
    title: String,
    description: String,
    location: Location,
    category: String,
    severity: String,
    status: String,
    reported_by: String,
    created_at: String,
    image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    points: i32,
}

// App state
struct AppState {
    issues: Mutex<HashMap<String, Issue>>,
    users: Mutex<HashMap<String, User>>,
}

// API endpoints
async fn report_issue(
    data: web::Json<Issue>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut issues = state.issues.lock().unwrap();
    let issue_id = uuid::Uuid::new_v4().to_string();
    let mut new_issue = data.into_inner();
    new_issue.id = issue_id.clone();
    issues.insert(issue_id.clone(), new_issue);
    HttpResponse::Created().json(issue_id)
}

async fn get_issues(state: web::Data<AppState>) -> impl Responder {
    let issues = state.issues.lock().unwrap();
    HttpResponse::Ok().json(issues.values().collect::<Vec<_>>())
}

async fn get_issue_by_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let issues = state.issues.lock().unwrap();
    if let Some(issue) = issues.get(&path.into_inner()) {
        HttpResponse::Ok().json(issue)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_issue_status(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (issue_id, new_status) = path.into_inner();
    let mut issues = state.issues.lock().unwrap();
    
    if let Some(issue) = issues.get_mut(&issue_id) {
        issue.status = new_status;
        HttpResponse::Ok().json(issue)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize app state
    let app_state = web::Data::new(AppState {
        issues: Mutex::new(HashMap::new()),
        users: Mutex::new(HashMap::new()),
    });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/api/issues", web::post().to(report_issue))
            .route("/api/issues", web::get().to(get_issues))
            .route("/api/issues/{id}", web::get().to(get_issue_by_id))
            .route("/api/issues/{id}/status/{status}", web::put().to(update_issue_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 