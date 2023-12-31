#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(FromForm, Debug)]
struct Book {
  title: String,
  author: String,
  isbn: String
}

#[get("/")]
fn index() -> Template {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    last_name: String
  }
  let context = Context {
    first_name: String::from("Rob"),
    last_name: String::from("Williamson")
  };
  Template::render("home", context)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

fn main() {
     rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .mount("/api", routes![hello, new_book])
        .attach(Template::fairing())
        .launch();
}
