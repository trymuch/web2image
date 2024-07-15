use clap::Parser;

use std::{path::PathBuf, time::Instant};
use url::Url;
mod web2image;

/// A command-line tool that can take screenshots of web pages and add a QR code on top of the captured image.
/// The URL link of the captured webpage can be obtained by scanning the QR code image.
#[derive(Debug, Parser)]
#[command(version = "0.1.0",author,about,long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// input url
    url: Url,
    /// output file
    #[arg(short, long, value_name = "FILE",value_parser = valid_path)]
    output: Option<PathBuf>,
}
fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli = Cli::parse();
    web2image::run(cli)?;
    let duration = start.elapsed().as_millis();
    println!("花费的时间： {:?}",duration);
    anyhow::Ok(())
}

// validate output path
fn valid_path(value: &str) -> Result<PathBuf, String> {
    
    let path = PathBuf::from(value.to_owned());
    let parent_option = path.parent();

    match parent_option {
        Some(parent) if parent.to_str().unwrap() != "" && !parent.exists() => {
            return Err("The specified parent directory does not exist.".into());
        }
        _ => {}
    }
    let extension = path.extension().and_then(|ext| {
        let ext = ext.to_str().unwrap().to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" => Some(ext),
            _ => None,
        }
    });
    match extension {
        Some(_) => Ok(path),
        None => Err(
            "Please specify the correct format through the extension, such as png, jpg, jpeg etc.",
        )?,
    }
}
