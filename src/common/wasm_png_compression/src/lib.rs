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
        let parts: Vec<&str> = image.split(",").collect();
        let base64_image = parts[1];
        let mime_type = parts[0].split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>()[0];

        let bytes = general_purpose::STANDARD
        .decode(base64_image)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if mime_type == "image/svg+xml" {
            // Directly return the SVG data without compression
            return Ok(JsValue::from(Uint8Array::from(bytes.as_slice())));
        }

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
