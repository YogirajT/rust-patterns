use actix_web::{get, web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Service {
    name: String,
    address: String,
    port: i32,
}

#[derive(Debug)]
struct ServiceRegistry {
    services: Vec<Service>,
}

impl ServiceRegistry {
    fn new() -> Self {
        ServiceRegistry { services: vec![] }
    }

    fn register(&mut self, service: Service) {
        self.services.push(service);
    }

    fn find(&self, name: &str) -> Option<&Service> {
        self.services
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.clone())
    }
}

#[get("/{name}")]
async fn find_service(
    web::Path(name): web::Path<String>,
    registry: web::Data<Arc<Mutex<ServiceRegistry>>>,
) -> HttpResponse {
    let registry = registry.lock().unwrap();
    match registry.find(&name) {
        Some(service) => HttpResponse::Ok().json(&service),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry = Arc::new(Mutex::new(ServiceRegistry::new()));
    registry.lock().unwrap().register(Service {
        name: "users".to_string(),
        address: "localhost".to_string(),
        port: 8080,
    });

    HttpServer::new(move || App::new().service(find_service).data(Arc::clone(&registry)))
        .bind("localhost:8000")?
        .run()
        .await
}
