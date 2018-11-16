extern crate diesel;
extern crate diesel_sample;

use diesel::prelude::*;
use diesel_sample::*;
use models::*;

fn main() {
    let connection = establish_connection();
    let results = search(&connection, Some("hoge".to_owned()), None).expect("error");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

fn search(
    conn: &PgConnection,
    title: Option<String>,
    body: Option<String>,
) -> Result<Vec<Post>, diesel::result::Error> {
    use diesel_sample::schema::posts;
    let mut query = posts::table.order(posts::id.desc()).into_boxed();

    if let Some(x) = title {
        query = query.filter(posts::title.eq(x))
    }

    if let Some(x) = body {
        query = query.filter(posts::body.eq(x))
     }

    query.load::<Post>(conn)
}
