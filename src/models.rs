#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
}


#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}
