use std::env;

use image::ImageReader;

use crate::error::UsageError;

pub(crate) mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        convert_tiles(&args[1], &args[2])?;
    } else {
        return Err(UsageError::new().into());
    }
    Ok(())
}

fn convert_tiles(
    in_image_path: &str,
    out_base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = ImageReader::open(in_image_path)?.decode()?;
    Ok(())
}
