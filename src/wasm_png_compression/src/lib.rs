
use wasm_bindgen::prelude::*;

use std::io::Cursor;
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;

use image::ImageReader;
use base64::{Engine as _, engine::{general_purpose}};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ImageUtil {}

#[wasm_bindgen]
impl ImageUtil {

    pub fn compress_image(image: &str) -> Result< JsValue, JsValue>{
        let base64_image = image.split(",").collect::<Vec<&str>>()[1];
        let bytes = general_purpose::STANDARD
            .decode(base64_image)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .decode()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        //shrink to max dimension of 32
        let img = img.thumbnail(32, 32);
        let mut buf = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(JsValue::from(Uint8Array::from(buf.as_slice())))
    }

}
