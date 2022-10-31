mod proxy;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use proxy::{get_proxy_client, ProxyClientProps};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RootBody {
    url: String,
    selectors: Vec<String>,
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    tag: String,
    key: String,
    value: String,
    inner_html: String,
}

#[derive(Serialize, Deserialize)]
struct SelectorRecord {
    selector: String,
    record: Vec<Record>,
}

#[post("/")]
async fn root(body: web::Json<RootBody>) -> impl Responder {
    let client = get_proxy_client(ProxyClientProps {
        host: body.host.clone(),
        port: body.port.clone(),
        username: body.username.clone(),
        password: body.password.clone(),
    });

    let response = client.get(&body.url).send().await;
    if response.is_err() {
        return HttpResponse::Ok().body("Error");
    }

    let html = response.unwrap().text().await.unwrap();
    let fragment = Html::parse_fragment(&html);

    println!("html: {:?}", html);

    let mut result: Vec<SelectorRecord> = Vec::new();

    for item in body.selectors.iter() {
        let selector = Selector::parse(item).unwrap();
        let mut record: Vec<Record> = Vec::new();

        for node in fragment.select(&selector) {
            let node_tag = node.html();
            let node_inner_html = node.inner_html().to_string();

            node.value().attrs().for_each(|attr| {
                record.push(Record {
                    tag: node_tag.clone(),
                    key: attr.0.to_string(),
                    value: attr.1.to_string(),
                    inner_html: node_inner_html.clone(),
                });
            });
        }

        result.push(SelectorRecord {
            selector: item.to_string(),
            record: record,
        });
    }

    HttpResponse::Ok().json(result)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
