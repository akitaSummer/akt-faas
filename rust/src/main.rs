mod function_moudle;
mod router;
mod trigger;

use std::fs::create_dir_all;
use std::path::Path;
use std::{convert::Infallible, net::SocketAddr};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};

use function_moudle::FunctionMoudle;
use router::Router;
use trigger::Trigger;

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

                async move {
                    Ok::<_, HttpError>(match (req.method(), req.uri().path()) {
                        // 查询faas
                        (&Method::GET, "/function/") => {
                            let mut body = String::from("<h1>Functions</h1>");

                            router.select().into_iter().for_each(|v| {
                                body += format!("<a href=\"{}\">{}</a><br>", v.1.path(), v.1.name())
                                    .as_str()
                            });

                            Response::builder()
                                .status(200)
                                .header("Content-type", "text/html; charset=utf-8")
                                .body(body.into())
                                .unwrap()
                        }
                        // 新建faas
                        (&Method::POST, "/function/") => {
                            let b = hyper::body::to_bytes(req.into_body()).await?;

                            match FunctionMoudle::from_json(&b).map(|f| f.build()) {
                                Some(Ok(f)) => {
                                    router.insert(f.trigger(), f);
                                    Response::new("Function Created".into())
                                }
                                Some(Err(e)) => {
                                    eprintln!("{}", e);
                                    Response::builder()
                                        .status(422)
                                        .body("Failed build process".into())
                                        .unwrap()
                                }
                                None => Response::builder()
                                    .status(422)
                                    .body("JSON error".into())
                                    .unwrap(),
                            }
                        }
                        // 删除faas
                        (&Method::DELETE, "/function/") => {
                            let b = hyper::body::to_bytes(req.into_body()).await?;

                            match FunctionMoudle::from_json(&b) {
                                Some(f) => f.delete(router),
                                None => Response::builder()
                                    .status(422)
                                    .body("JSON error".into())
                                    .unwrap(),
                            }
                        }
                        // 执行faas
                        (_, _) => {
                            let (parts, body) = req.into_parts();

                            match router.get(&Trigger::new(parts.method.as_str(), parts.uri.path()))
                            {
                                Some(f) => f.run(parts, body).await,
                                None => {
                                    Response::builder().status(404).body(Body::empty()).unwrap()
                                }
                            }
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
