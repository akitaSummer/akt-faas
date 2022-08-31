mod function_moudle;
mod router;
mod trigger;

use std::fs::create_dir_all;
use std::path::Path;
use std::{convert::Infallible, net::SocketAddr};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};

use router::Router;

type HttpError = Box<dyn std::error::Error + Send + Sync>;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    create_dir_all(Path::new("data/")).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let router = Router::new();

    let make_svc = make_service_fn(move |_| {
        let router = router.clone();

        async move {
            Ok::<_, HttpError>(service_fn(move |req| {
                let router = router.clone();

                async {
                    Ok::<_, HttpError>(match (req.method(), req.uri().path()) {
                        (&Method::GET, "/function/") => {
                            let mut body = String::from("<h1>Functions</h1>");

                            todo!()
                        }
                        (&Method::POST, "/function/") => {
                            let b = hyper::body::to_bytes(req.into_body()).await?;

                            todo!()
                        }

                        (&Method::DELETE, "/function/") => {
                            let b = hyper::body::to_bytes(req.into_body()).await?;

                            todo!()
                        }

                        (_, _) => {
                            let (parts, body) = req.into_parts();

                            Response::builder().status(404).body(Body::empty()).unwrap()
                        }
                    })
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening on http://{}", addr);

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
