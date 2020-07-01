use rocket::request::{Form, FromForm};
use rocket::response::NamedFile;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::fmt::{self, Display, Formatter};

#[derive(FromForm)]
pub struct UserInput {
    username: String,
    password: String,
    age: i32,
}

#[post("/submit", data = "<user_input>")]
pub fn submit_task(user_input: Form<UserInput>) -> rusqlite::Result<String> {
    let connection = Connection::open("src/sql/test.db")?;
    connection.execute(
        "INSERT INTO USER_REGISTRY (USER_NAME, PASSWORD, AGE)
                                VALUES (?1, ?2, ?3);",
        params![user_input.username, user_input.password, user_input.age],
    )?;
    Ok(format!(
        "Hello {}, you are {} years old. and your password is {}",
        user_input.username, user_input.age, user_input.password
    ))
}

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/<file..>", rank = 2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

impl Display for UserInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is {} years old and their password is {}",
            self.username, self.age, self.password
        )
    }
}
