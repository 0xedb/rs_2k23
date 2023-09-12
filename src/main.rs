use actix_web::{
    get,
    http::{self, header},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Scope,
};
use std::{
    io::Result,
    sync::{Arc, Mutex},
};

use mime;

#[derive(Debug, Default)]
struct MyCounter {
    count: Mutex<usize>,
}

#[get("/")]
async fn home(req: HttpRequest, data: web::Data<usize>) -> HttpResponse {
    // let d = req.app_data::<usize>().unwrap_or(|| &3030339);
    let d = data.get_ref();
    let name = req.app_data::<&str>();
    println!(
        "req,  {name:?} {:?}",
        req.version() == actix_web::http::Version::HTTP_11
    );

    // let val = req.app_data::<Arc<MyCounter>>().unwrap();
    // println!("{val:?}");

    let mut val = req
        .app_data::<Arc<MyCounter>>()
        .unwrap()
        .count
        .lock()
        .unwrap();
    *val += 10;

    // let h = HttpResponse::new(http::StatusCode::ACCEPTED);

    let p = format!("<h1>ans {d:?} COUNT: {}</h1>", *val);
    // let mmmm = req.headers().get(key)
    HttpResponse::Accepted()
        .content_type(header::ContentType(mime::APPLICATION_JSON))
        .body(r#"{"bruno": 0xedb}"#)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let m = Arc::new(MyCounter {
        count: Mutex::new(666),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(m.clone())
            .app_data("bruno's server")
            .app_data(web::Data::new(0xedbusize))
            .service(home)
            .service(rs_2k23::api)
    })
    .bind(("127.0.0.1", 2023))?
    .run()
    .await
}
