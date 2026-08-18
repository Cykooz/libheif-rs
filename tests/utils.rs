use libheif_rs::{Channel, ColorSpace, Image, RgbChroma};

pub fn create_image(width: u32, height: u32) -> libheif_rs::Result<Image> {
    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::Rgb))?;
    image.create_plane(Channel::Interleaved, width, height, 24)?;

    let planes = image.planes_mut();
    let plane = planes.interleaved.unwrap();
    let stride = plane.stride;
    let data = plane.data;

    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = x * y;
            data[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data[row_start + 1] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data[row_start + 2] = (color & 0x00_00_00_ff) as u8;
            row_start += 3;
        }
    }
    Ok(image)
}
