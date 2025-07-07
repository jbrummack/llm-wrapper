use std::io::Cursor;

use crate::{
    error::LlmError,
    message::{MessageContent, TYPE_IMAGE_URL},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use image::{ImageFormat, guess_format, load_from_memory};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImageUrl {
    url: String,
}
#[derive(Debug, Serialize)]
pub struct Image {
    r#type: &'static str,
    image_url: ImageUrl,
}

pub enum ImageMime {
    Jpeg,
    Png,
    WebP,
}
impl ImageMime {
    fn get(&self) -> &'static str {
        match self {
            ImageMime::Jpeg => "data:image/jpeg;base64,",
            ImageMime::Png => "data:image/png;base64,",
            ImageMime::WebP => "data:image/webp;base64,",
        }
    }
}
impl MessageContent {
    pub fn b64_image_passthrough(b64: String) -> Self {
        Self::Image {
            r#type: TYPE_IMAGE_URL,
            image_url: ImageUrl { url: b64 },
        }
    }
    pub fn image_data(data: &[u8], force_mime: Option<ImageMime>) -> Result<Self, LlmError> {
        let format = guess_format(data)?;
        let capacity = 32 + data.len() + (data.len() / 3);
        let mut payload = String::with_capacity(capacity);
        match force_mime {
            Some(ImageMime::Jpeg) => {
                payload.push_str(ImageMime::Jpeg.get());
                if let ImageFormat::Jpeg = format {
                    tracing::debug!("Encoding Jpeg into b64");
                    BASE64_STANDARD.encode_string(data, &mut payload);
                } else {
                    tracing::debug!("Converting {format:?} into Jpeg");
                    let img = load_from_memory(data)?;
                    let mut jpeg_data = Cursor::new(Vec::with_capacity(data.len() * 2));
                    img.write_to(&mut jpeg_data, ImageFormat::Jpeg)?;
                    tracing::debug!("Encoding Jpeg into b64");
                    BASE64_STANDARD.encode_string(jpeg_data.into_inner(), &mut payload);
                }
            }
            Some(ImageMime::Png) => {
                payload.push_str(ImageMime::Jpeg.get());
                if let ImageFormat::Png = format {
                    tracing::debug!("Encoding Png into b64");
                    BASE64_STANDARD.encode_string(data, &mut payload);
                } else {
                    tracing::debug!("Converting {format:?} into Png");
                    let img = load_from_memory(data)?;
                    let mut jpeg_data = Cursor::new(Vec::with_capacity(data.len() * 2));
                    img.write_to(&mut jpeg_data, ImageFormat::Png)?;
                    tracing::debug!("Encoding Png into b64");
                    BASE64_STANDARD.encode_string(jpeg_data.into_inner(), &mut payload);
                }
            }
            Some(ImageMime::WebP) => {
                payload.push_str(ImageMime::WebP.get());
                if let ImageFormat::WebP = format {
                    tracing::debug!("Encoding Webp into b64");
                    BASE64_STANDARD.encode_string(data, &mut payload);
                } else {
                    tracing::debug!("Converting {format:?} into Webp");
                    let img = load_from_memory(data)?;
                    let mut jpeg_data = Cursor::new(Vec::with_capacity(data.len() * 2));
                    img.write_to(&mut jpeg_data, ImageFormat::WebP)?;
                    tracing::debug!("Encoding Webp into b64");
                    BASE64_STANDARD.encode_string(jpeg_data.into_inner(), &mut payload);
                }
            }
            None => match format {
                image::ImageFormat::Png => {
                    payload.push_str(ImageMime::Png.get());
                    BASE64_STANDARD.encode_string(data, &mut payload);
                }
                image::ImageFormat::Jpeg => {
                    payload.push_str(ImageMime::Jpeg.get());
                    BASE64_STANDARD.encode_string(data, &mut payload);
                }
                image::ImageFormat::WebP => {
                    payload.push_str(ImageMime::WebP.get());
                    BASE64_STANDARD.encode_string(data, &mut payload);
                }
                //If not standard format encode into webp
                _ => {
                    tracing::debug!("Converting image into WebP");
                    payload.push_str(ImageMime::WebP.get());
                    let img = load_from_memory(data)?;
                    let mut jpeg_data = Cursor::new(Vec::with_capacity(data.len() * 2));
                    img.write_to(&mut jpeg_data, ImageFormat::WebP)?;
                    BASE64_STANDARD.encode_string(jpeg_data.into_inner(), &mut payload);
                }
            },
        }

        Ok(Self::Image {
            r#type: TYPE_IMAGE_URL,
            image_url: ImageUrl { url: payload },
        })
    }
    /*pub fn new(image: DynamicImage) -> Result<Self, LlmError> {
        let r = Self {
            r#type: TYPE_IMAGE_URL,
        };
        Ok(r)
    }*/
}
