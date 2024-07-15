use std::path::{Path, PathBuf};

use anyhow::anyhow;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use image::{imageops, DynamicImage, ExtendedColorType, Luma};
use qrcode::QrCode;
pub(crate) fn run(cli: crate::Cli) -> anyhow::Result<()> {
    let url = cli.url.as_ref();
    let output = match cli.output {
        Some(path) => path,
        None => PathBuf::from("snapshot.png"),
    };
    let mut bottom = url2image(url, image_format(output.clone())?)?;
    let top = url2qrcode(cli.url.as_str())?;
    let width = bottom.width();
    let height = bottom.height();
    do_overlay(&mut bottom, &top)?;
    image::save_buffer(
        output,
        bottom.as_bytes(),
        width,
        height,
        ExtendedColorType::Rgb8,
    )?;
    anyhow::Ok(())
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) -> anyhow::Result<()> {
    let x = (bottom.width() - top.width() - 10) as i64;
    let y = (bottom.height() - top.height() - 10) as i64;
    imageops::overlay(bottom, top, x, y);
    anyhow::Ok(())
}

fn url2image(
    url: &str,
    image_format: CaptureScreenshotFormatOption,
) -> anyhow::Result<DynamicImage> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let viewport = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .wait_for_element("body")?
        .get_box_model()?
        .margin_viewport();
    let data = tab.capture_screenshot(image_format, Some(100), Some(viewport), true)?;
    image::load_from_memory(&data).map_err(|err| anyhow!(err))
}

fn image_format(path: impl AsRef<Path>) -> anyhow::Result<CaptureScreenshotFormatOption> {
    let path = path.as_ref();
    let ext = match path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => Err(anyhow!("The extension of the output file is incorrect. The extension should be png, jpg, or webp."))?,
        };
    match ext {
        "jpeg" | "jpg" => anyhow::Ok(CaptureScreenshotFormatOption::Jpeg),
        "png" => anyhow::Ok(CaptureScreenshotFormatOption::Png),
        "webp" => anyhow::Ok(CaptureScreenshotFormatOption::Webp),
        _ => Err(anyhow!("Unrecognized image format."))?,
    }
}

fn url2qrcode(url: &str) -> anyhow::Result<DynamicImage> {
    let code = QrCode::new(url)?;
    let image_buffer = code.render::<Luma<u8>>().build();
    let image = DynamicImage::from(image_buffer);
    anyhow::Ok(image)
}
