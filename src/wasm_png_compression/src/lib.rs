mod utils;

use wasm_bindgen::prelude::*;

use std::io::Cursor;
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;

use image::{EncodableLayout, ImageReader};
use base64::{Engine as _, engine::{general_purpose}};

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
        let base64_image = image.split(",").collect::<Vec<&str>>()[1];
        let bytes = general_purpose::STANDARD
        .decode(base64_image).unwrap();
        let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
        //shrink to max dimension of 32
        let img = img.thumbnail(32, 32);
        let mut buf = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();

        Ok(JsValue::from(Uint8Array::from(buf.as_slice())))
    }

}
