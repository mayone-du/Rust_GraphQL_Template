use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

// 同階層のschemaをimport
mod graphql;

use crate::graphql::{create_schema, Schema};

// PlayGround用のGraphiQLを定義
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// POST用のGraphQL APIエンドポイントを定義
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

// main関数
#[actix_web::main]
async fn main() -> io::Result<()> {
    // juniperのschemaを作成
    let schema = std::sync::Arc::new(create_schema());

    // httpサーバーを起動
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::new() // CORSの設定
                    .allowed_methods(vec!["POST", "GET"]) // 許可するメソッド
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    // 8080番ポートで待機
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
