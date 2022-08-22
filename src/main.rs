use std::{io::Cursor, env};

use cors::CORS;
use rocket::{serde::{Serialize}, get, launch, routes, Config};
use rocket::{http::{ContentType, Status}, response::Responder};

mod cors;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response<T> where T: Send {
    response: T
}

impl<'r, T> Responder<'r, 'static> for Response<T> where T: Serialize + Send {
    fn respond_to(self, _request: &rocket::Request) -> rocket::response::Result<'static> {
        let json_string = match serde_json::to_string_pretty(&self) {
            Ok(result) => result,
            Err(_) => return Result::Err(Status::InternalServerError)
        };

        let response = rocket::Response::build()
            .sized_body(json_string.len(), Cursor::new(json_string))
            .header(ContentType::new("application", "json"))
            .status(Status::Ok)
            .finalize();

        
        Result::Ok(response)
    }
}

#[get("/status")]
fn status() -> &'static str {
    "Started!"
}

#[get("/completions/<query>")]
fn completions(query: String) -> Result<Response<Vec<String>>, String> {
    let completions: Vec<String> = match ureq::get(&format!("https://www.zssk.sk/wp-admin/admin-ajax.php?query={}&action=autocomplete_st", query)).call() {
        Ok(x) => match x.into_json() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string())
    };


    return Ok(Response { response: completions })
}

#[launch]
fn rocket() -> _ {
    let port: i32 = i32::from_str_radix(&env::var("PORT").unwrap_or("8081".to_string()), 10).unwrap();
    let config = Config::figment().merge(("port", port)).merge(("address", "0.0.0.0"));

    println!("Running on port: {}", port);

    rocket::custom(config)
            .attach(CORS)
        .mount("/", routes![status, completions])
}