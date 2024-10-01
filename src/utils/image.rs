use anyhow::{bail, Result};

/// 猜测图片类型
pub fn guess_image_format(buffer: &[u8]) -> String {
    let img = image::guess_format(buffer).unwrap();
    match img {
        image::ImageFormat::Png => "image/png",
        image::ImageFormat::Jpeg => "image/jpeg",
        image::ImageFormat::Gif => "image/gif",
        image::ImageFormat::WebP => "image/webp",
        image::ImageFormat::Pnm => "image/x-portable-anymap",
        image::ImageFormat::Tiff => "image/tiff",
        image::ImageFormat::Tga => "image/x-tga",
        image::ImageFormat::Dds => "image/vnd.ms-dds",
        image::ImageFormat::Bmp => "image/bmp",
        image::ImageFormat::Ico => "image/x-icon",
        image::ImageFormat::Hdr => "image/vnd.radiance",
        image::ImageFormat::OpenExr => "image/x-exr",
        image::ImageFormat::Farbfeld => "image/x-farbfeld",
        image::ImageFormat::Avif => "image/avif",
        image::ImageFormat::Qoi => "image/x-qoi",
        _ => "unknown",
    }
    .into()
}

/// 猜测图片类型以及返回图片对应base64编码字符串
pub async fn get_image_type_and_base64_string(image_path: String) -> Result<(String, String)> {
    use base64::{engine::general_purpose, Engine as _};
    use image::EncodableLayout;
    use std::{fs::File, io::Read};

    use crate::utils::image::guess_image_format;

    let client = reqwest::Client::new();

    if image_path.starts_with("https://") || image_path.starts_with("http://") {
        let response = client.get(image_path).send().await?;
        if response.status().is_success() {
            let bytes = response.bytes().await?; // 读取整个响应体为字节
            let base64_string = general_purpose::STANDARD.encode(&bytes);
            Ok((guess_image_format(bytes.as_bytes()), base64_string))
        } else {
            bail!("Failed to download image, status: {}", response.status());
        }
    } else {
        let mut buffer = Vec::new();
        let mut file = File::open(image_path)?;
        file.read_to_end(&mut buffer)?;
        let base64_string = general_purpose::STANDARD.encode(&buffer);
        Ok((guess_image_format(buffer.as_slice()), base64_string))
    }
}

pub mod blocking {
    use super::*;

    /// 猜测图片类型以及返回图片对应base64编码字符串
    pub fn get_image_type_and_base64_string(image_path: String) -> Result<(String, String)> {
        use base64::{engine::general_purpose, Engine as _};
        use image::EncodableLayout;
        use std::{fs::File, io::Read};

        use crate::utils::image::guess_image_format;

        let client = reqwest::blocking::Client::new();

        if image_path.starts_with("https://") || image_path.starts_with("http://") {
            let response = client.get(image_path).send()?;
            if response.status().is_success() {
                let bytes = response.bytes()?; // 读取整个响应体为字节
                let base64_string = general_purpose::STANDARD.encode(&bytes);
                Ok((guess_image_format(bytes.as_bytes()), base64_string))
            } else {
                bail!("Failed to download image, status: {}", response.status());
            }
        } else {
            let mut buffer = Vec::new();
            let mut file = File::open(image_path)?;
            file.read_to_end(&mut buffer)?;
            let base64_string = general_purpose::STANDARD.encode(&buffer);
            Ok((guess_image_format(buffer.as_slice()), base64_string))
        }
    }
}
