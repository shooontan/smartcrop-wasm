extern crate image;

use image::{imageops, ImageBuffer};

#[derive(Debug)]
pub struct SmartCropOption {
  width: u32,
  height: u32,

  crop_width: u32,
  crop_height: u32,

  detail_weight: f64,

  skin_color: [f64; 3],
  skin_bias: f64,
  skin_threshold: f64,
  skin_weight: f64,
  skin_brightness_min: f64,
  skin_brightness_max: f64,

  saturation_brightness_min: f64,
  saturation_brightness_max: f64,
  saturation_threshold: f64,
  saturation_bias: f64,
  saturation_weight: f64,

  score_down_sample: u32,

  step: u32,
  scale_step: f64,
  max_scale: f64,
  min_scale: f64,

  edge_radius: f64,
  edge_weight: f64,
  outside_importance: f64,
  rule_of_thirds: bool,
  prescale: bool,
}

impl SmartCropOption {
  pub fn new(width: u32, height: u32) -> SmartCropOption {
    SmartCropOption {
      width,
      height,

      crop_width: 0,
      crop_height: 0,

      detail_weight: 0.2,

      skin_color: [0.78, 0.57, 0.44],
      skin_bias: 0.01,
      skin_threshold: 0.8,
      skin_weight: 1.8,
      skin_brightness_min: 0.2,
      skin_brightness_max: 1.,

      saturation_brightness_min: 0.05,
      saturation_brightness_max: 0.9,
      saturation_threshold: 0.4,
      saturation_bias: 0.2,
      saturation_weight: 0.1,

      score_down_sample: 8,

      step: 8,
      scale_step: 0.1,
      min_scale: 1.,
      max_scale: 1.,

      edge_radius: 0.4,
      edge_weight: -20.,
      outside_importance: -0.5,
      rule_of_thirds: true,
      prescale: true,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Crop {
  pub x: u32,
  pub y: u32,
  pub width: u32,
  pub height: u32,
}

impl Crop {
  fn new(x: u32, y: u32, width: u32, height: u32) -> Crop {
    Crop {
      x,
      y,
      width,
      height,
    }
  }
}

#[derive(Debug)]
struct Score {
  detail: f64,
  saturation: f64,
  skin: f64,
  boost: f64,
  total: f64,
}

#[derive(Debug)]
struct ScoredCrop {
  crop: Crop,
  score: Score,
}

#[derive(Copy, Clone, Debug)]
pub struct RGBA {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

impl RGBA {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
    RGBA { r, g, b, a }
  }

  fn cie(self) -> f64 {
    0.5126 * self.b as f64 + 0.7152 * self.g as f64 + 0.0722 * self.r as f64
  }

  fn saturation(self) -> f64 {
    let r = self.r as f64;
    let g = self.g as f64;
    let b = self.b as f64;

    let cv = vec![r / 255., g / 255., b / 255.];
    let max = cv.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let min = cv.iter().fold(0.0 / 0.0, |m, v| v.min(m));

    if max == min {
      0.;
    }

    let l = (max + min) / 2.;
    let d = max - min;

    if l > 0.5 {
      d / (2. - max - min)
    } else {
      d / (max + min)
    }
  }
}

#[derive(Debug)]
pub struct Image {
  width: u32,
  height: u32,

  data: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Image {
  fn new(width: u32, height: u32, data: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>) -> Image {
    let data = match data {
      Some(d) => d,
      None => ImageBuffer::new(width, height),
    };

    Image {
      width,
      height,
      data,
    }
  }

  fn get_rgba(&self, x: u32, y: u32) -> RGBA {
    let pixel = self.data.get_pixel(x, y);
    RGBA {
      r: pixel[0],
      g: pixel[1],
      b: pixel[2],
      a: pixel[3],
    }
  }

  fn set_rgba(&mut self, x: u32, y: u32, rgba: RGBA) {
    self
      .data
      .put_pixel(x, y, image::Rgba([rgba.r, rgba.g, rgba.b, rgba.a]));
  }

  fn cie(&self, x: u32, y: u32) -> f64 {
    let pixel = self.data.get_pixel(x, y);
    let r = pixel[0] as f64;
    let g = pixel[1] as f64;
    let b = pixel[2] as f64;
    0.5126 * b + 0.7152 * g + 0.0722 * r
  }

  fn down_sample(self, factor: u32) -> Image {
    let width = (self.width as f64 / factor as f64).floor() as u32;
    let height = (self.height as f64 / factor as f64).floor() as u32;
    let mut output = Image::new(width, height, None);
    let ifactor = 1. / (factor * factor) as f64;

    for y in 0..height {
      for x in 0..width {
        let mut r = 0.;
        let mut g = 0.;
        let mut b = 0.;
        let mut a = 0.;
        let mut mr = 0.;
        let mut mg = 0.;

        for v in 0..factor {
          for u in 0..factor {
            let ix = x * factor + u;
            let iy = y * factor + v;

            let rgba = self.get_rgba(ix, iy);
            r += rgba.r as f64;
            g += rgba.g as f64;
            b += rgba.b as f64;
            a += rgba.a as f64;
            mr = f64::max(mr, rgba.r as f64);
            mg = f64::max(mg, rgba.g as f64);
          }
        }

        output.set_rgba(
          x,
          y,
          RGBA::new(
            (r * ifactor * 0.5 + mr * 0.5) as u8,
            (g * ifactor * 0.7 + mg * 0.3) as u8,
            (b * ifactor) as u8,
            (a * ifactor) as u8,
          ),
        )
      }
    }

    output
  }
}

pub fn open(input: ImageBuffer<image::Rgba<u8>, Vec<u8>>, mut opt: SmartCropOption) -> Crop {
  let (width, height) = input.dimensions();

  let mut image = Image::new(width, height, Some(input));

  let scale = f64::min(
    width as f64 / opt.width as f64,
    height as f64 / opt.height as f64,
  );
  let mut prescale = 1.;

  opt.crop_width = (opt.width as f64 * scale) as u32;
  opt.crop_height = (opt.height as f64 * scale) as u32;
  opt.min_scale = f64::min(opt.max_scale, f64::max(1. / scale, opt.min_scale));

  if opt.prescale {
    prescale = f64::min(
      f64::max(256. / image.width as f64, 256. / image.height as f64),
      1.,
    );
    if (prescale < 1.) {
      let rinput = imageops::resize(
        &image.data,
        ((width as f64) * prescale) as u32,
        ((height as f64) * prescale) as u32,
        imageops::FilterType::Lanczos3,
      );
      image = Image::new(rinput.width(), rinput.height(), Some(rinput));
      opt.crop_width = ((opt.crop_width as f64) * prescale) as u32;
      opt.crop_height = ((opt.crop_height as f64) * prescale) as u32;
    } else {
      prescale = 1.;
    }
  }

  let result = analyse(&image, &opt);

  Crop {
    x: (result.crop.x as f64 / prescale) as u32,
    y: (result.crop.y as f64 / prescale) as u32,
    width: (result.crop.width as f64 / prescale) as u32,
    height: (result.crop.height as f64 / prescale) as u32,
  }
}

fn analyse(input: &Image, opt: &SmartCropOption) -> ScoredCrop {
  let mut output = Image::new(input.width, input.height, None);

  edge_detect(&input, &mut output);
  skin_detect(&input, &mut output, &opt);
  saturation_detect(&input, &mut output, &opt);

  let score_output = output.down_sample(opt.score_down_sample);

  let crops = generate_crops(&input, &opt);

  let top_crop: Option<ScoredCrop> = crops
    .iter()
    .map(|crop| ScoredCrop {
      crop: crop.clone(),
      score: get_score(crop, &score_output, opt),
    })
    .fold(None, |result, scored_crop| {
      Some(match result {
        None => scored_crop,
        Some(result) => {
          if result.score.total > scored_crop.score.total {
            result
          } else {
            scored_crop
          }
        }
      })
    });

  top_crop.unwrap()
}

fn get_score(crop: &Crop, image: &Image, opt: &SmartCropOption) -> Score {
  let mut detail = 0.;
  let mut skin = 0.;
  let mut saturation = 0.;
  let mut _boost = 0.;

  let down_sample = opt.score_down_sample;
  let inv_down_sample = 1. / down_sample as f64;
  let output_height_down_sample = image.height * down_sample;
  let output_width_down_sample = image.width * down_sample;

  for y in (0..output_height_down_sample).filter(|y| y % down_sample == 0) {
    for x in (0..output_width_down_sample).filter(|x| x % down_sample == 0) {
      let dx = (x as f64 * inv_down_sample) as u32;
      let dy = (y as f64 * inv_down_sample) as u32;
      let i = importance(crop, x, y, opt);
      let rgb = image.get_rgba(dx, dy);
      let r = rgb.r as f64;
      let g = rgb.g as f64;
      let b = rgb.b as f64;
      let d = g / 255.;
      skin += r / 255. * (d + opt.skin_bias) * i;
      detail += detail * i;
      saturation += (b / 255.) * (d + opt.saturation_bias) * i;
    }
  }

  let total =
    (detail * opt.detail_weight + skin * opt.skin_weight + saturation * opt.saturation_weight)
      / (crop.width * crop.height) as f64;

  Score {
    detail,
    saturation,
    skin,
    boost: 0.,
    total,
  }
}

fn importance(crop: &Crop, x: u32, y: u32, opt: &SmartCropOption) -> f64 {
  if crop.x > x || x >= crop.x + crop.width || crop.y > y || y >= crop.y + crop.height {
    return opt.outside_importance;
  }

  let ix = (x - crop.x) as f64 / crop.width as f64;
  let iy = (y - crop.y) as f64 / crop.height as f64;
  let px = (0.5 - ix).abs() * 2.;
  let py = (0.5 - iy).abs() * 2.;
  // Distance from edge
  let dx = f64::max(px - 1.0 + opt.edge_radius, 0.);
  let dy = f64::max(py - 1.0 + opt.edge_radius, 0.);
  let d = (dx * dx + dy * dy) * opt.edge_weight;
  let mut s = 1.41 - (px * px + py * py).sqrt();
  if opt.rule_of_thirds {
    s += (f64::max(0., s + d + 0.5) * 1.2) * (thirds(px) + thirds(py));
  }
  s + d
}

fn generate_crops(image: &Image, opt: &SmartCropOption) -> Vec<Crop> {
  let mut crops: Vec<Crop> = Vec::new();

  let min_dim = u32::min(image.width, image.height);

  let crop_width = if opt.crop_width != 0 {
    opt.crop_width
  } else {
    min_dim
  };
  let crop_height = if opt.crop_height != 0 {
    opt.crop_height
  } else {
    min_dim
  };

  let mut scale = opt.max_scale;

  while scale >= opt.min_scale {
    let mut y = 0;
    while (y + crop_height) as f64 * scale <= image.height.into() {
      let mut x = 0;
      while (x + crop_width) as f64 * scale <= image.width.into() {
        crops.push(Crop::new(
          x,
          y,
          (crop_width as f64 * scale) as u32,
          (crop_height as f64 * scale) as u32,
        ));
        x += opt.step;
      }
      y += opt.step;
    }
    scale -= opt.scale_step;
  }

  crops
}

fn edge_detect(input: &Image, output: &mut Image) {
  let (w, h) = input.data.dimensions();

  for y in 0..h {
    for x in 0..w {
      let lightness = if x == 0 || x >= w - 1 || y == 0 || y >= h - 1 {
        input.cie(x, y)
      } else {
        input.cie(x, y) * 4.
          - input.cie(x, y - 1)
          - input.cie(x - 1, y)
          - input.cie(x + 1, y)
          - input.cie(x, y + 1)
      };
      let rgba = RGBA {
        g: lightness as u8,
        ..output.get_rgba(x, y)
      };
      output.set_rgba(x, y, rgba);
    }
  }
}

fn skin_detect(input: &Image, output: &mut Image, opt: &SmartCropOption) {
  let w = input.width;
  let h = input.height;

  for y in 0..h {
    for x in 0..w {
      let rgba = input.get_rgba(x, y);

      let lightness = rgba.cie() / 255.;
      let skin = skin_color(&rgba, opt);
      let is_skin_color = skin > opt.skin_threshold;
      let is_skin_brightness =
        lightness >= opt.skin_brightness_min && lightness <= opt.skin_brightness_max;

      let rgba = if is_skin_color && is_skin_brightness {
        let r = (skin - opt.skin_threshold) * (255. / (1. - opt.skin_threshold));
        RGBA {
          r: r as u8,
          ..output.get_rgba(x, y)
        }
      } else {
        RGBA {
          r: 0,
          ..output.get_rgba(x, y)
        }
      };

      output.set_rgba(x, y, rgba);
    }
  }
}

fn skin_color(color: &RGBA, opt: &SmartCropOption) -> f64 {
  let r = color.r as f64;
  let g = color.g as f64;
  let b = color.b as f64;

  let mag = f64::sqrt(r * r + g * g + b * b);
  let rd = r / mag - opt.skin_color[0];
  let gd = g / mag - opt.skin_color[1];
  let bd = b / mag - opt.skin_color[2];
  let d = f64::sqrt(rd * rd + gd * gd + bd * bd);

  1. - d
}

fn saturation_detect(input: &Image, output: &mut Image, opt: &SmartCropOption) {
  let w = input.width;
  let h = input.height;

  for y in 0..h {
    for x in 0..w {
      let rgba = input.get_rgba(x, y);
      let lightness = rgba.cie() / 255.;
      let sat = rgba.saturation();

      let acceptable_saturation = sat > opt.saturation_threshold;
      let acceptable_lightness =
        lightness >= opt.saturation_brightness_min && lightness <= opt.saturation_brightness_max;

      let rgba = if acceptable_saturation && acceptable_lightness {
        let b = (sat - opt.saturation_threshold) * (255. / (1. - opt.saturation_threshold));
        RGBA {
          b: b as u8,
          ..output.get_rgba(x, y)
        }
      } else {
        RGBA {
          b: 0,
          ..output.get_rgba(x, y)
        }
      };

      output.set_rgba(x, y, rgba);
    }
  }
}

fn thirds(x: f64) -> f64 {
  let y = ((x - (1. / 3.) + 1.0) % 2.0 * 0.5 - 0.5) * 16.;
  f64::max(1.0 - y * y, 0.0)
}
