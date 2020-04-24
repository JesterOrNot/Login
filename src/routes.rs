use rocket::request::{Form, FromForm};
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[derive(FromForm)]
pub struct UserInput {
    username: String,
    password: String,
    age: i32,
}

#[post("/submit", data = "<user_input>")]
pub fn submit_task(user_input: Form<UserInput>) -> String {
    format!(
        "Hello {}, you are {} years old. and your password is {}",
        user_input.username, user_input.age, user_input.password
    )
}

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/<file..>", rank = 2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
