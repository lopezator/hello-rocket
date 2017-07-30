#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/hello", routes![index]).launch();
}

/*
Notas:
La función index se carga mediante el path "/world".
Después se monta el path "/hello" sobre la ruta index (que corresponde al path "/world").
La el path completo resultante será /hello/world
*/

