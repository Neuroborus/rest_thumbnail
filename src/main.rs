#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::response::content::Content;
use rocket::http::ContentType;
use std::fs::File;
//use std::io::Cursor;
//use bytes::Bytes;
use image::{DynamicImage, load_from_memory};

#[tokio::main]
async fn get_image(url: &RawStr) -> DynamicImage{
    let im = reqwest::get(url.as_str())
        .await.unwrap()
        .bytes()
        .await.unwrap();
    load_from_memory(&im.slice(..)).unwrap()
}

#[tokio::main]
async fn miniature(img: &DynamicImage, width: u32) -> DynamicImage{
    let (w, h) = img.as_rgba8().unwrap().dimensions();
    let del = w/width;
    img.thumbnail(width, h/del)
}

//file
/*#[get("/<url>")]
fn retrieve(url: &RawStr) -> Option<File>{
    let pic = miniature(&get_image(&url), 100);
    pic.save("tmp.png").unwrap();
    File::open("tmp.png").ok()
}*/

//memory
#[get("/<url>")]
fn retrieve(url: &RawStr) -> Option<Content<Vec<u8>>>{
    let mut buf = Vec::new();
    miniature(&get_image(&url), 100).
        write_to(&mut buf, image::ImageOutputFormat::PNG).
            unwrap();

    let v = Content(ContentType::PNG, buf);
    Some(v)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![retrieve])
        .launch();
}
