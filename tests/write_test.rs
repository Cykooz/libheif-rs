use libheif_rs::{
    Channel, ColorSpace, CompressionFormat, EncoderQuality, EncodingOptions, HeifContext, Image,
    Result, RgbChroma,
};

#[test]
fn create_and_encode_image() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::Rgb))?;
    image.create_plane(Channel::Interleaved, width, height, 24)?;

    let planes = image.planes_mut();
    let plane = planes.interleaved.unwrap();
    let stride = plane.stride;
    let data = plane.data;

    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = (x * y) as u32;
            data[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data[row_start + 1] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data[row_start + 2] = (color & 0x00_00_00_ff) as u8;
            row_start += 3;
        }
    }

    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;

    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options: EncodingOptions = Default::default();

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width);
    assert_eq!(handle.height(), height);

    // Decode the image
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb))?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgb)));
    let planes = image.planes();
    let plan = planes.interleaved.unwrap();
    assert_eq!(plan.width, width);
    assert_eq!(plan.height, height);

    Ok(())
}

#[test]
fn create_and_encode_monochrome_image() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Monochrome)?;

    image.create_plane(Channel::Y, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_a = planes.y.unwrap();
    let stride = plane_a.stride;
    let data_a = plane_a.data;

    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = ((x + y) % 255) as u8;
            data_a[row_start] = color;
            row_start += 1;
        }
    }

    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;

    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options: EncodingOptions = Default::default();

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;
    let _buf = context.write_to_bytes()?;

    Ok(())
}

#[test]
#[ignore] // for manual running
fn create_and_encode_lossless_image() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::Rgb))?;

    image.create_plane(Channel::Interleaved, width, height, 24)?;

    let planes = image.planes_mut();
    let plane = planes.interleaved.unwrap();
    let stride = plane.stride;
    let data = plane.data;
    let byte_pp = plane.bits_pre_pixel / 8;

    let mut color: u32;
    for y in 0..height {
        let mut row_start = stride * y as usize;
        color = if y % 2 == 0 {
            0x00_00_00_FF
        } else {
            0x00_00_FF_00
        };
        for _ in 0..width {
            data[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data[row_start + 1] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data[row_start + 2] = (color & 0x00_00_00_ff) as u8;
            row_start += byte_pp as usize;
            //color = color.rotate_right(8);
        }
    }

    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options: EncodingOptions = Default::default();

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;
    context.write_to_file("/home/cykooz/t2.heic")?;

    Ok(())
}

#[test]
fn write_test_from_readme_file() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::C444))?;

    image.create_plane(Channel::R, width, height, 8)?;
    image.create_plane(Channel::G, width, height, 8)?;
    image.create_plane(Channel::B, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_r = planes.r.unwrap();
    let stride = plane_r.stride;

    let data_r = plane_r.data;
    let data_g = planes.g.unwrap().data;
    let data_b = planes.b.unwrap().data;

    // Fill data of planes by some "pixels"
    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = (x * y) as u32;
            data_r[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data_g[row_start] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data_b[row_start] = (color & 0x00_00_00_ff) as u8;
            row_start += 1;
        }
    }

    // Encode image and save it into file.
    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    context.encode_image(&image, &mut encoder, None)?;
    // Uncomment this in README.md
    //context.write_to_file("./data/new.heif")?;

    Ok(())
}
