use actix_files as fs;
use std::path::{Path, PathBuf};
use actix_web::{Result, web};

use serde::Deserialize;
use image::GenericImageView;
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
    let fpout = asset_dir.join(
        format!("{}-{}x{}", file.title(), size.width.unwrap(), size.height.unwrap())
    ).with_extension(file.path.extension().unwrap());

    let f = std::fs::File::open(&fpout).or_else(|_| {
        let fpin = asset_dir.join(file.name());
        resize(&fpin, &size, &fpout)
    })?;

    Ok(fs::NamedFile::from_file(f, fpout)?)
}


fn resize(fp: &PathBuf, size: &FileSize, out: &PathBuf) -> std::io::Result<std::fs::File> {
    let img = image::open(&fp).unwrap();
    let img = img.thumbnail(
        size.width.unwrap_or(img.width()),
        size.height.unwrap_or(img.height())
    );
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&out)?;

    let ext = out.extension()
        .and_then(|s| s.to_str())
        .map_or("".to_string(), |s| s.to_ascii_lowercase());

    img.write_to(&mut file, guess_format(&ext).unwrap()).unwrap();
    Ok(file)
}


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
