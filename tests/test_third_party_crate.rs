use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use image::Luma;
use qrcode::QrCode;
use std::{fs, path::PathBuf};
use url::{ParseError, Url};
#[test]
fn test_url() {
    assert!(Url::parse("http://[:::1]") == Err(ParseError::InvalidIpv6Address))
}

#[test]
fn test_pathbuf() {
    let pathbuf = PathBuf::from("res/snapshot.png".to_owned());
    let parent = pathbuf.parent();
    println!("{:?}", parent);
    // println!("exists : {}",parent.unwrap().exists());
    println!("is_file: {}", pathbuf.is_file());
    println!("is_dir: {}", pathbuf.is_dir());

    let filename = pathbuf.file_name().unwrap();
    println!("filename: {:?}", filename);
}
#[test]
fn test_pathbuf2() {
    let path = PathBuf::from("".to_owned());
    println!("{}", path.exists());
    println!("{}", path.is_dir());
}

#[test]
fn test_headless_chrome() -> anyhow::Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let viewport = tab
        .navigate_to("https://github.com/trymuch")?
        .wait_until_navigated()?
        .wait_for_element("body")?
        .get_box_model()?
        .margin_viewport();
    let data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        Some(100),
        Some(viewport),
        true,
    )?;
    fs::write("test.png", data)?;
    anyhow::Ok(())
}

#[test]
fn test_qrcode() -> anyhow::Result<()> {
    // Encode some data into bits.
    let code = QrCode::new("01234567").unwrap();

    // Render the bits into an image.
    let image: image::ImageBuffer<Luma<u8>, Vec<u8>> = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();

    // You can also render it into a string.
    let string = code.render().light_color(' ').dark_color('#').build();
    println!("{}", string);
    anyhow::Ok(())
}
