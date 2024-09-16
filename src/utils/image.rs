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
