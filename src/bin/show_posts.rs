extern crate diesel;
extern crate diesel_sample;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel_sample::schema::*;
use diesel_sample::*;
use models::*;

fn main() {
    let connection = establish_connection();
    let results = search(&connection, Some("hoge".to_owned()), None, None).expect("error");

    println!("Displaying {} posts", results.1.len());
    println!("total posts: {}", results.0);
    for post in results.1 {
        println!("{}", post.0.title);
        println!("----------\n");
        println!("{}", post.0.body);
    }
}

fn search(
    conn: &PgConnection,
    title: Option<String>,
    body: Option<String>,
    user_name: Option<String>,
) -> Result<(i64, Vec<(Post, User)>), diesel::result::Error> {
    let posts = query(&title, &body, &user_name)
        .offset(0)
        .limit(100)
        .load::<(Post, User)>(conn)?;
    let total = query(&title, &body, &user_name)
        .select(count(posts::id))
        .group_by(posts::id)
        .first(conn)?;
    Ok((total, posts))
}

fn query<'a>(
    title: &'a Option<String>,
    body: &'a Option<String>,
    user_name: &'a Option<String>,
) -> diesel::query_builder::BoxedSelectStatement<
    'a,
    (posts::SqlType, users::SqlType),
    diesel::query_source::joins::JoinOn<
        diesel::query_source::joins::Join<
            posts::table,
            users::table,
            diesel::query_source::joins::Inner,
        >,
        diesel::expression::operators::Eq<
            diesel::expression::nullable::Nullable<posts::user_id>,
            diesel::expression::nullable::Nullable<users::id>,
        >,
    >,
    diesel::pg::Pg,
> {
    let mut query = posts::table
        .inner_join(users::table)
        .order(posts::id.desc())
        .into_boxed();

    if let Some(x) = title {
        query = query.filter(posts::title.eq(x))
    }

    if let Some(x) = body {
        query = query.filter(posts::body.eq(x))
    }

    if let Some(x) = user_name {
        query = query.filter(users::name.eq(x))
    }

    query
}
