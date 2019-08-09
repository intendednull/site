use actix_files as fs;
use serde::Deserialize;
use image::GenericImageView;
use std::path::{Path, PathBuf};
use actix_web::{Result, web};

use super::{util::File};


#[derive(Deserialize)]
struct FileSize {
    width: Option<u32>,
    height: Option<u32>
}


pub fn asset_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/static")
            .route("assets/{path}", web::get().to(asset))
    );
}


fn asset((file, size): (web::Path<File>, web::Query<FileSize>)) -> Result<fs::NamedFile> {
    let asset_dir = Path::new("static/assets");
    let fpin = asset_dir.join(file.name());
    let fpout = match (size.width, size.height) {
        (None, None) => fpin.clone(),
        (w, h) => {
            // Generate a new file name in cache dir based on requested dimensions.
            // The actual image dimensions are unknown until after it's resized,
            // which means identical images may exist if the requested dimensions
            // are different.
            let fp = format!(".cache/{}-{}x{}", file.title(), w.unwrap_or(0), h.unwrap_or(0));
            let fp = asset_dir.join(fp)
                .with_extension(file.path.extension().unwrap());
            // Verify cache directory exists
            fp.parent().map(|p| std::fs::create_dir(p));
            fp
        }
    };
    // Open file if it exists, otherwise attempt to generate it.
    let f = std::fs::File::open(&fpout).or_else(
        |_| resize(&fpin, &size, &fpout))?;

    Ok(fs::NamedFile::from_file(f, fpout)?)
}


/// Generate an image that best fits `size` while preserving scale.
/// Returns generated file.
fn resize(fp: &PathBuf, size: &FileSize, out: &PathBuf) -> std::io::Result<std::fs::File> {
    let img = image::open(&fp).unwrap();
    let img = img.thumbnail(
        size.width.unwrap_or(img.width()),
        size.height.unwrap_or(img.height())
    );
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .read(true)
        .open(&out)?;
    let ext = out.extension()
        .and_then(|s| s.to_str())
        .map_or("".to_owned(), |s| s.to_ascii_lowercase());

    img.write_to(&mut file, guess_format(&ext).unwrap()).unwrap();
    Ok(file)
}


/// Attempt to get image format from file extension.
/// Stolen from https://docs.rs/image/0.22.1/src/image/dynimage.rs.html#755
fn guess_format(ext: &str) -> std::result::Result<image::ImageFormat, image::ImageError> {
    match ext {
        "jpg" | "jpeg" => Ok(image::ImageFormat::JPEG),
        "png" => Ok(image::ImageFormat::PNG),
        "gif" => Ok(image::ImageFormat::GIF),
        "webp" => Ok(image::ImageFormat::WEBP),
        "tif" | "tiff" => Ok(image::ImageFormat::TIFF),
        "tga" => Ok(image::ImageFormat::TGA),
        "bmp" => Ok(image::ImageFormat::BMP),
        "ico" => Ok(image::ImageFormat::ICO),
        "hdr" => Ok(image::ImageFormat::HDR),
        "pbm" | "pam" | "ppm" | "pgm" => Ok(image::ImageFormat::PNM),
        format => {
            return Err(image::ImageError::UnsupportedError(format!(
                "Image format image/{:?} is not supported.",
                format
            )))
        }
    }
}


// TODO remove oldest read cache files when it hits overflow
fn clean_cache() -> std::io::Result<()> {
    Ok(())
}
