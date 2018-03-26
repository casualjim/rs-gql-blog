use super::schema::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
  pub email: String
}

#[derive(Queryable,Identifiable,Serialize,Deserialize)]
pub struct Post {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
  pub user_id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Queryable,Identifiable,Serialize,Deserialize)]
pub struct Comment {
  pub id: i32,
  pub user_id: i32,
  pub post_id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="comments"]
pub struct NewComment {
  pub user_id: i32,
  pub post_id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="followers"]
pub struct Follower {
  pub follower_id: i32,
  pub followee_id: i32,
}

