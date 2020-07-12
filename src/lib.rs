extern crate image;

pub mod smartcrop;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn crop(data: &[u8], width: u32, height: u32) -> Vec<u32> {
  match image::load_from_memory(data) {
    Ok(img) => {
      let opt = smartcrop::SmartCropOption::new(width, height);
      let crop = smartcrop::open(img.to_rgba(), opt);
      vec![crop.x, crop.y, crop.width, crop.height]
    }
    Err(_) => vec![],
  }
}
