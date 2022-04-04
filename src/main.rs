
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey};
use openssl::x509::{X509};

use hudsucker::{
    async_trait::async_trait,
    certificate_authority::OpensslAuthority,
    hyper::{Body, Request, Response},
    *,
};
use std::net::SocketAddr;
use tracing::*;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone)]
struct LogHandler {}

#[async_trait]
impl HttpHandler for LogHandler {

    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
       // println!("{:?}", req);
        // println!("context : {:?}", _ctx);

        // if req.method() == hyper::Method::POST{
        //         println!("post");
        //         use hyper::{body::HttpBody};
        //         let bytes = hyper::body::to_bytes(req.into_body()).await;
        //         println!("{:#?}", bytes)
        // } 

        // let request: Request<Body> = Request::default();
        
        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        // println!("dans handle response");
         println!("{:?}", res);
        res
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
  
  
    let private_key_bytes: &[u8] = include_bytes!("../rootca_key.key");
    let ca_cert_bytes: &[u8] = include_bytes!("../rootca_cert.cer");

    let private_key =
        PKey::private_key_from_pem(private_key_bytes).expect("Failed to parse private key");
    let ca_cert = X509::from_pem(ca_cert_bytes).expect("Failed to parse CA certificate");

    let ca = OpensslAuthority::new(private_key, ca_cert, MessageDigest::sha256(), 100_000);
    
    let proxy = ProxyBuilder::new()
        .with_addr(SocketAddr::from(([127, 0, 0, 1], 2000)))
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(LogHandler {})
        .build();

    if let Err(e) = proxy.start(shutdown_signal()).await {
        error!("{}", e);
    }
}