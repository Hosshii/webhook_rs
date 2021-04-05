// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

use github_webhook::event::Base;
use serde_json::Result;

fn main() {
    let data = r#"{
        "type": "WatchEvent",
        "public": false,
        "payload": {
        },
        "repo": {
            "id": 3,
            "name": "octocat/Hello-World",
            "url": "https://api.github.com/repos/octocat/Hello-World"
        },
        "actor": {
            "id": 1,
            "login": "octocat",
            "gravatar_id": "",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "url": "https://api.github.com/users/octocat",
            "display_login":"h"
        },
        "org": {
            "id": 1,
            "login": "github",
            "gravatar_id": "",
            "url": "https://api.github.com/orgs/github",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif"
        },
        "created_at": "2011-09-06T17:26:27Z",
        "id": "12345"
    }"#;

    match serde_json::from_str::<Base>(data) {
        Ok(e) => println!("{:?}", e),
        Err(e) => eprintln!("{}", e),
    }
}
