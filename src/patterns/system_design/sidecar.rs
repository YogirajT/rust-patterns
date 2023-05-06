// Main Process

// use std::io::Result;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

// async fn handle_request(_: Request<Body>) -> Result<Response<Body>> {
//     Ok(Response::new(Body::from("Hello, World!")))
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let make_svc = make_service_fn(|_conn| async {
//         Ok::<_, hyper::Error>(service_fn(handle_request))
//     });
//     let addr = ([127, 0, 0, 1], 3000).into();
//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);

//     server.await?;
//     Ok(())
// }

// Sidecar Process

// use std::io::Result;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

// async fn handle_request(request: Request<Body>) -> Result<Response<Body>> {
//     // Log the incoming request
//     println!("Received request: {:?}", request);

//     let response = forward_request(request).await?;
//     println!("Received response: {:?}", response);

//     Ok(response)
// }

// async fn forward_request(request: Request<Body>) -> Result<Response<Body>> {
//     Ok(Response::new(Body::from("Hello, World!")))
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let make_svc = make_service_fn(|_conn| async {
//         Ok::<_, hyper::Error>(service_fn(handle_request))
//     });
//     let addr = ([127, 0, 0, 1], 3001).into();
//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);

//     server.await?;
//     Ok(())
// }
