mod utils;

use wasm_bindgen::prelude::*;

use std::io::Cursor;
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;

use image::{EncodableLayout, ImageFormat, ImageFormat::Png, ImageReader};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct ImageUtil {}

#[wasm_bindgen]
impl ImageUtil {

    pub fn compress_image(image: &str) -> Result< JsValue, JsValue>{

        //check if string starts with data:image/
        if !image.starts_with("data:image/") {
            alert("Invalid image format");
            return Err(JsValue::from_str("Invalid image format"));
        }

        let base64_image = image.split(",").collect::<Vec<&str>>()[1];
        let bytes = general_purpose::STANDARD
        .decode(base64_image).unwrap();
        let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
        //let img2 = ImageReader::new(Cursor::new(bytes)).set_format(Png).unwrap().decode().unwrap();
        //shrink to max dimension of 32
        let img = img.thumbnail(32, 32);
        //compress to PNG with 0.5 quality
        let mut buf = Vec::new();

        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();

        Ok(JsValue::from(Uint8Array::from(buf.as_slice())))
    }

}
