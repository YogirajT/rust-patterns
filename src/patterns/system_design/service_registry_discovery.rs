#![allow(dead_code)]
#![allow(private_in_public)] // to fix issues with find_service

use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
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

    fn find(&self, name: &str) -> Option<Service> {
        let service = self.services.iter().find(|s| s.name == name);
        service.cloned()
    }
}

#[get("/{name}")]
pub async fn find_service(
    name: web::Path<String>,
    registry: web::Data<Arc<Mutex<ServiceRegistry>>>,
) -> HttpResponse {
    #[allow(private_in_public)]
    let registry = registry.lock().unwrap();
    match registry.find(&name) {
        Some(service) => HttpResponse::Ok().json(service),
        None => HttpResponse::NotFound().finish(),
    }
}

#[cfg(test)]
mod service_discovery_tests {
    use std::sync::{Arc, Mutex};

    use actix_web::{web::Data, App, HttpServer};

    use super::{find_service, Service, ServiceRegistry};

    #[actix_web::test]
    async fn main() -> std::io::Result<()> {
        let registry = Arc::new(Mutex::new(ServiceRegistry::new()));
        registry.lock().unwrap().register(Service {
            name: "users".to_string(),
            address: "localhost".to_string(),
            port: 8080,
        });

        HttpServer::new(move || {
            App::new()
                .service(find_service)
                .app_data(Data::new(Arc::clone(&registry)))
        })
        .bind("localhost:8000")?
        .run()
        .await
    }
}
