use axum::extract::Query;
use axum::response::Html;
use axum::{routing::get,Router};
use std::net::SocketAddr;
use serde::Deserialize;
use rand::{thread_rng,Rng};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127,0,0,1],80));

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();


   
}

#[derive(Deserialize)]
struct RangeParameters {
    start:usize,
    end:usize
}
async fn handler(Query(range):Query<RangeParameters>)->Html<String>{
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number: {}</h1>", random_number))
    
} 




