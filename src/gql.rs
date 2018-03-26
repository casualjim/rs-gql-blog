use ::diesel::prelude::*;
use super::db::Pool;
use ::juniper::{Context as JuniperContext, FieldResult, RootNode};
use super::models::{User,Post,Comment,Follower,NewUser,NewPost,NewComment};

pub struct Context {
  pub db: Pool
}

impl Context {
  pub fn new() -> Context {
    Context {
      db: super::db::init_pool(),
    }
  }
}

impl JuniperContext for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

graphql_object!(User: Context |&self| {
  description: "A user who owns posts and comments on the site"

  field id() -> String as "the unique id of the user" {
    self.id.to_string()
  }

  field email() -> String as "the email address of the user" {
    self.email.to_owned()
  }

  field post(&executor, id: i32) -> FieldResult<Option<Post>> {
    use super::schema::posts::dsl;
    let conn = executor.context().db.get()?;
    dsl::posts
      .filter(dsl::id.eq(id))
      .first::<Post>(&*conn)
      .optional()
      .map_err(|err| {
      ::juniper::FieldError::new("Could not get post for user", ::juniper::Value::null())
    })
  }


  field posts(&executor) -> FieldResult<Vec<Post>> {
    use super::schema::posts::dsl;
    let conn = executor.context().db.get()?;
    dsl::posts.filter(dsl::user_id.eq(self.id)).load::<Post>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get posts for user", ::juniper::Value::null())
    })
  }

  field follower(&executor, id: i32) -> FieldResult<Option<User>> {
    use super::schema::users::dsl;
    use super::schema::followers::{dsl as fdsl};

    let q = dsl::users
      .filter(fdsl::followee_id.eq(self.id).and(fdsl::follower_id.eq(id)))
      .select((dsl::id, dsl::email))
      .inner_join(fdsl::followers.on(fdsl::follower_id.eq(dsl::id)));
    let conn = executor.context().db.get()?;
    q.first::<User>(&*conn).optional().map_err(|err| {
      ::juniper::FieldError::new("Could not get followers for user", ::juniper::Value::null())
    })
  }

  field followers(&executor) -> FieldResult<Vec<User>> {
    use super::schema::users::dsl;
    use super::schema::followers::{dsl as fdsl};

    let q = dsl::users
      .filter(fdsl::followee_id.eq(self.id))
      .select((dsl::id, dsl::email))
      .inner_join(fdsl::followers.on(fdsl::follower_id.eq(dsl::id)));
    let conn = executor.context().db.get()?;
    q.load::<User>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get followers for user", ::juniper::Value::null())
    })
  }

  field followees(&executor) -> FieldResult<Vec<User>> {
    use super::schema::users::dsl;
    use super::schema::followers::{dsl as fdsl};

    let q = dsl::users
      .filter(fdsl::follower_id.eq(self.id))
      .select((dsl::id, dsl::email))
      .inner_join(fdsl::followers.on(fdsl::followee_id.eq(dsl::id)));
    let conn = executor.context().db.get()?;
    q.load::<User>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get followers for user", ::juniper::Value::null())
    })
  }

  field followee(&executor, id: i32) -> FieldResult<Option<User>> {
    use super::schema::users::dsl;
    use super::schema::followers::{dsl as fdsl};

    let q = dsl::users
      .filter(fdsl::follower_id.eq(self.id).and(fdsl::followee_id.eq(id)))
      .select((dsl::id, dsl::email))
      .inner_join(fdsl::followers.on(fdsl::followee_id.eq(dsl::id)));
    let conn = executor.context().db.get()?;
    q.first::<User>(&*conn).optional().map_err(|err| {
      ::juniper::FieldError::new("Could not get followers for user", ::juniper::Value::null())
    })
  }
});

graphql_object!(Post: Context |&self| {
  description: "A post on the blog created by a user"

  field id() -> String as "the unique id of the post" {
    self.id.to_string()
  }

  field title() -> &str as "the title of the post" {
    self.title.as_str()
  }

  field body() -> &str as "the body of the post" {
    self.body.as_str()
  }

  field user(&executor) -> FieldResult<User> {
    use super::schema::users::dsl::*;
    let conn = executor.context().db.get()?;
    users.filter(id.eq(self.user_id)).first::<User>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get user for post", ::juniper::Value::null())
    })
  }

  field comment(&executor, id: i32) -> FieldResult<Option<Comment>> {
    use super::schema::comments::dsl;
    let conn = executor.context().db.get()?;
    dsl::comments
      .filter(dsl::post_id.eq(self.id).and(dsl::id.eq(id)))
      .first::<Comment>(&*conn)
      .optional()
      .map_err(|err| {
        ::juniper::FieldError::new("Could not get comment for post", ::juniper::Value::null())
      })
  }

  field comments(&executor) -> FieldResult<Vec<Comment>> {
    use super::schema::comments::dsl;
    let conn = executor.context().db.get()?;
    dsl::comments
      .filter(dsl::post_id.eq(self.id))
      .load::<Comment>(&*conn)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not get comment for post", ::juniper::Value::null())
      })
  }
});

graphql_object!(Comment: Context |&self| {
  description: "A comment from a user on a post"

  field id() -> String as "the unique id of the comment" {
    self.id.to_string()
  }


  field title() -> &str as "the title of the comment" {
    self.title.as_str()
  }

  field body() -> &str as "the body of the comment" {
    self.body.as_str()
  }

  field user(&executor) -> FieldResult<User> {
    use super::schema::users::dsl::*;
    let conn = executor.context().db.get()?;
    users.filter(id.eq(self.user_id)).first::<User>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get user for comment", ::juniper::Value::null())
    })
  }

  field post(&executor) -> FieldResult<Post> {
    use super::schema::posts::dsl;
    let conn = executor.context().db.get()?;
    dsl::posts.find(self.post_id).first::<Post>(&*conn).map_err(|err| {
      ::juniper::FieldError::new("Could not get post for comment", ::juniper::Value::null())
    })
  }
});

pub struct QueryRoot;
graphql_object!(QueryRoot: Context |&self| {
  field user(&executor, id: i32) -> FieldResult<Option<User>> {
    use super::schema::users::dsl;
    let conn = executor.context().db.get()?;
    dsl::users
      .filter(dsl::id.eq(id))
      .first::<User>(&*conn)
      .optional()
      .map_err(|err| {
      ::juniper::FieldError::new("Could not get user", ::juniper::Value::null())
    })
  }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: Context |&self| {

  field createUser(&executor, email: String) -> FieldResult<Option<User>> {
    use super::schema::users;
    let conn = executor.context().db.get()?;
    let params = NewUser{
      email: email.to_owned()
    };
    ::diesel::insert_into(users::table)
      .values(&params)
      .get_result(&*conn)
      .optional()
      .map_err(|err| {
      ::juniper::FieldError::new("Could not create user", ::juniper::Value::null())
    })
  }

  field removeUser(&executor, id: i32) -> FieldResult<bool> {
    use super::schema::users::dsl;
    let conn = executor.context().db.get()?;
    ::diesel::delete(dsl::users.filter(dsl::id.eq(id)))
      .execute(&*conn)
      .map(|row_count| row_count > 0)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not remove user", ::juniper::Value::null())
      })
  }

  field follow(&executor, follower: i32, followee: i32) -> FieldResult<bool> {
    use super::schema::followers;
    let conn = executor.context().db.get()?;
    let params = Follower {
      follower_id: follower,
      followee_id: followee,
    };
    ::diesel::insert_into(followers::table)
      .values(&params)
      .execute(&*conn)
      .map(|row_count| row_count > 0)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not remove user", ::juniper::Value::null())
      })
  }

  field unfollow(&executor, follower: i32, followee: i32) -> FieldResult<bool> {
    use super::schema::followers::dsl::*;
    let conn = executor.context().db.get()?;
    ::diesel::delete(followers.filter(follower_id.eq(follower).and(followee_id.eq(followee))))
      .execute(&*conn)
      .map(|row_count| row_count > 0)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not remove user", ::juniper::Value::null())
      })
  }

  field createPost(&executor, user: i32, title: String, body: String) -> FieldResult<Option<Post>> {
    use super::schema::posts;
    let conn = executor.context().db.get()?;
    let params = NewPost{
      user_id: user,
      title: title,
      body: body,
    };
    ::diesel::insert_into(posts::table)
      .values(&params)
      .get_result(&*conn)
      .optional()
      .map_err(|err| {
      ::juniper::FieldError::new("Could not create post", ::juniper::Value::null())
    })
  }

  field removePost(&executor, id: i32) -> FieldResult<bool> {
    use super::schema::posts::dsl;
    let conn = executor.context().db.get()?;
    ::diesel::delete(dsl::posts.filter(dsl::id.eq(id)))
      .execute(&*conn)
      .map(|row_count| row_count > 0)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not remove post", ::juniper::Value::null())
      })
  }

  field createComment(&executor, user: i32, post: i32, title: String, body: String) -> FieldResult<Option<Comment>> {
    use super::schema::comments;
    let conn = executor.context().db.get()?;
    let params = NewComment{
      user_id: user,
      post_id: post,
      title: title,
      body: body,
    };
    ::diesel::insert_into(comments::table)
      .values(&params)
      .get_result(&*conn)
      .optional()
      .map_err(|err| {
      ::juniper::FieldError::new("Could not create comment", ::juniper::Value::null())
    })
  }

  field removeComment(&executor, id: i32) -> FieldResult<bool> {
    use super::schema::comments::dsl;
    let conn = executor.context().db.get()?;
    ::diesel::delete(dsl::comments.filter(dsl::id.eq(id)))
      .execute(&*conn)
      .map(|row_count| row_count > 0)
      .map_err(|err| {
        ::juniper::FieldError::new("Could not remove comment", ::juniper::Value::null())
      })
  }
});
