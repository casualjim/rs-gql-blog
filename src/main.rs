#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate juniper_demo;
extern crate rocket;
extern crate rocket_contrib;
extern crate juniper_rocket;
extern crate diesel;

use diesel::prelude::*;
use rocket::State;
use rocket::response::content;
use rocket::fairing::AdHoc;
use juniper_demo::db;
use juniper_demo::models::*;
use rocket_contrib::Json;
use diesel::pg::Pg;
use diesel::debug_query;

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

#[post("/users", format="application/json", data="<new_user>")]
fn create_user(conn: db::Conn, new_user: Json<NewUser>) -> QueryResult<Json<User>> {
  use juniper_demo::schema::users;
  diesel::insert_into(users::table).values(&*new_user).get_result(&*conn).map(|user| Json(user))
}

#[get("/users")]
fn get_users(conn: db::Conn) -> QueryResult<Json<Vec<User>>> {
  use juniper_demo::schema::users::dsl::*;
  users.load::<User>(&*conn).map(|user| Json(user))
}

#[get("/users/<uid>")]
fn get_user_by_id(conn: db::Conn, uid: i32) -> QueryResult<Json<User>> {
  use juniper_demo::schema::users::dsl::*;
  users.filter(id.eq(uid)).first::<User>(&*conn).map(|user| Json(user))
}

#[get("/users/<uid>/posts")]
fn get_posts_for_user(conn: db::Conn, uid: i32) -> QueryResult<Json<Vec<Post>>> {
  use juniper_demo::schema::posts::dsl::*;
  let q = posts.filter(user_id.eq(uid));
  println!("get posts for user {}", debug_query::<Pg, _>(&q));
  q.load::<Post>(&*conn).map(|post| Json(post))
}

#[get("/users/<uid>/followers")]
fn get_followers_for_user(conn: db::Conn, uid: i32) -> QueryResult<Json<Vec<User>>> {
  use juniper_demo::schema::users::dsl;
  use juniper_demo::schema::followers::{dsl as fdsl};

  let q = dsl::users
    .filter(fdsl::followee_id.eq(uid))
    .select((dsl::id, dsl::email))
    .inner_join(fdsl::followers.on(fdsl::follower_id.eq(dsl::id)));
  println!("get followers for user {}", debug_query::<Pg, _>(&q));
  q.load::<User>(&*conn).map(|user| Json(user))
}

fn main() {
  rocket::ignite()
    .manage(gql::Context::new())
    .manage(gql::Schema::new(
      gql::QueryRoot{},
      gql::MutationRoot{},
    ))
    .manage(db::init_pool())
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
        create_user, get_users,
        get_user_by_id,
        get_posts_for_user,
        get_followers_for_user,
      ],
    )
    .launch();
}
