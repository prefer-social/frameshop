use std::ops::Div;

use anyhow::Result;
use photon_rs::native::open_image_from_bytes;
use photon_rs::transform::{crop, resample};
use photon_rs::PhotonImage;

pub async fn get_image_size(image: &[u8]) -> Result<(u32, u32)> {
    let img = open_image_from_bytes(image).expect("Not getting image bytes");
    Ok((img.get_width(), img.get_height()))
}

pub async fn preview_image(image: &[u8]) -> Result<Vec<u8>> {
    let a = crop_image_for_ratio(&image, 640, 480).await.unwrap();
    let b = resample_image(image, 640, 480).await.unwrap();
    Ok(b)
}

async fn resample_image(image: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    let (current_width, current_height) = get_image_size(image).await.unwrap();
    let current_image_size_ratio = current_width.div_ceil(current_height);
    let img = open_image_from_bytes(image).expect("Not getting image bytes");
    let resampled_image = match current_image_size_ratio > 1 {
        // image is landscape
        true => resample(
            &img,
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
        ),
        // image is portrait
        false => resample(
            &img,
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
        ),
    };

    //let resized_image = resize(&img, width, hegiht, SamplingFilter::Nearest);
    //let resampled_image = resample(&img, 800, 0);
    Ok(resampled_image.get_bytes())
}

async fn crop_image_for_ratio(image: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    let (image_width, image_height) = get_image_size(image).await.unwrap();
    if image_width < width && image_height < height {
        return Ok(image.to_vec());
    }

    let a = greatest_common_divisor(
        usize::try_from(width).unwrap(),
        usize::try_from(height).unwrap(),
    )
    .await;

    let image_ratio: (usize, usize) = (
        usize::try_from(width).unwrap() / a,
        usize::try_from(height).unwrap() / a,
    );

    let (f1, f2) = image_ratio;
    let f3 = f1 + f2;

    let factor = (image_width + image_height).div_ceil(f3 as u32);

    let adjusted_width = f2 as u32 * factor;
    let adjusted_height = f1 as u32 * factor;

    if image_width == adjusted_width && image_height == adjusted_height {
        return Ok(image.to_vec());
    }

    let a = (image_width - adjusted_width) / 2;
    let b = (image_height - adjusted_height) / 2;
    let c = image_width - a;
    let d = image_height - b;

    let mut img = open_image_from_bytes(image).expect("Not getting image bytes");
    let cropped_image = crop(&mut img, a, c, b, d);
    Ok(cropped_image.get_bytes())
}

async fn greatest_common_divisor(left_val: usize, right_val: usize) -> usize {
    let mut a = left_val;
    let mut b = right_val;
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

async fn watermark() {}
