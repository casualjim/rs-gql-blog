#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate juniper_demo;
extern crate rocket;
extern crate rocket_contrib;
extern crate juniper_rocket;
extern crate diesel;

use rocket::response::content;
use rocket::fairing::AdHoc;
use rocket::State;
use juniper_demo::gql;


#[get("/")]
fn graphiql() -> content::Html<String> {
  juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
  context: State<gql::Context>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<gql::Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
  context: State<gql::Context>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<gql::Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}

fn main() {
  rocket::ignite()
    .manage(gql::Context::new())
    .manage(gql::Schema::new(
      gql::QueryRoot{},
      gql::MutationRoot{},
    ))
    .attach(AdHoc::on_response(|_, resp| {
      if resp.headers().get_one("Content-Type") == Some("application/json") {
        resp.set_raw_header("Content-Type", "application/json;charset=utf-8");
      }
    }))
    .mount(
      "/",
      routes![
        graphiql,
        get_graphql_handler,
        post_graphql_handler,
      ],
    )
    .launch();
}
