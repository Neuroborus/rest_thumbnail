#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::request;
use rocket::Outcome;
use rocket::http::{RawStr, Status, ContentType};

#[macro_use]
extern crate curl;
use curl::easy;

use image::{DynamicImage, load_from_memory};

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
}

struct ApiKey(String);
impl<'a, 'r> request::FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("url").collect();
        match keys.len(){
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1=> Outcome::Success(ApiKey(keys[0].to_string())),
            _=> Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }

    }
}

#[tokio::main]
async fn get_image(url: String) -> DynamicImage{
    /*let im = reqwest::get(url.as_str())
        .await.unwrap()
        .bytes()
        .await.unwrap();*/
    let mut data = Vec::new();
    let mut handle = easy::Easy::new();
    handle.url(url.as_str()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    load_from_memory(&data[..]).unwrap()
}

#[tokio::main]
async fn miniature(img: &DynamicImage, width: u32) -> DynamicImage{
    let (w, h) = img.as_rgb8().unwrap().dimensions();
    let del = w/width;
    img.thumbnail(width, h/del)
}

/*#[get("/check")]
fn retrieve(key: ApiKey) -> String{
    String::from(key.0)
}*/

//memory
#[get("/img")]
fn retrieve(key: ApiKey) -> content::Content<Vec<u8>>{
    let mut buf = Vec::new();
    miniature(&get_image(key.0), 100).
        write_to(&mut buf, image::ImageOutputFormat::PNG).
            unwrap();
    content::Content(ContentType::PNG, buf)
}

#[get("/<t>")]
fn index(t: &RawStr) -> String {
    let mut s = String::from("Hello, ");
    s.push_str(t.as_str());
    s.push_str(".!.");
    
    s
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, retrieve])
        .launch();
}
