#[macro_use] 
extern crate rocket;

use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::http::{ContentType, Status};
use simple_crypt::{encrypt, decrypt};
use base64::prelude::*;

#[derive(FromForm)]
struct Crypt {
    pub data: String,
    pub password: String
}

#[post("/encrypt", data = "<data>")]
fn encrypt_path(data: Form<Crypt>) -> (Status, (ContentType, String)) {
    let inner = data.into_inner();
    let data = inner.data;
    let password = inner.password;

    match encrypt(data.as_bytes(), password.as_bytes()) {
        Ok(dat) => return (Status::Ok, (ContentType::JSON, BASE64_STANDARD.encode(dat.as_slice()))),
        Err(_) => return (Status::BadGateway, (ContentType::JSON, "".to_string()))
    }

    
}

#[post("/decrypt", data = "<data>")]
fn decrypt_path(data: Form<Crypt>) -> (Status, (ContentType, String)) {
    let inner = data.into_inner();
    let data = inner.data;
    let password = inner.password;

    let data = match BASE64_STANDARD.decode(data) {
        Ok(dat) => dat,
        Err(_) => return (Status::BadGateway, (ContentType::JSON, "".to_string()))
    };

    match decrypt(data.as_slice(), password.as_bytes()) {
        Ok(dat) => {
            let dat = match String::from_utf8(dat) {
                Ok(dat) => dat,
                Err(_) => return (Status::BadGateway, (ContentType::JSON, "".to_string()))
            };

            return (Status::Ok, (ContentType::JSON, dat));
        },
        Err(_) => return (Status::BadGateway, (ContentType::JSON, "".to_string()))
    }

    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![encrypt_path, decrypt_path])
    .mount("/", FileServer::from("static"))
}
