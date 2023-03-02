use libheif_rs::{
    Channel, ColorSpace, CompressionFormat, EncoderParameterValue, EncoderQuality, EncodingOptions,
    HeifContext, Image, Result, RgbChroma,
};

fn create_image(width: u32, height: u32) -> Result<Image> {
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

#[test]
fn create_and_encode_image() -> Result<()> {
    let width = 640;
    let height = 480;

    let image = create_image(width, height)?;
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
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
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
fn set_encoder_param() -> Result<()> {
    let width = 640;
    let height = 480;

    let image = create_image(width, height)?;

    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Av1)?;
    encoder.set_parameter_value("speed", EncoderParameterValue::Int(5))?;
    let encoding_options: EncodingOptions = Default::default();
    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width);
    assert_eq!(handle.height(), height);

    // Decode the image
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgb)));
    let planes = image.planes();
    let plan = planes.interleaved.unwrap();
    assert_eq!(plan.width, width);
    assert_eq!(plan.height, height);

    Ok(())
}

#[test]
fn add_metadata() -> Result<()> {
    let width = 640;
    let height = 480;
    let image = create_image(width, height)?;
    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;
    let handle = context.encode_image(&image, &mut encoder, None)?;

    let item_type = b"MyDt";
    let item_data = b"custom data";
    let exif_data = b"MM\0*FakeExif";
    let content_type = Some("text/plain");
    context.add_generic_metadata(&handle, item_data, item_type, content_type)?;
    context.add_exif_metadata(&handle, exif_data)?;
    context.add_xmp_metadata(&handle, item_data)?;

    // Write result HEIF file into vector
    let buf = context.write_to_bytes()?;

    // Check stored meta data in the encoded result
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;

    // Custom meta data block "MyDt"
    let mut item_ids = vec![0; 1];
    let count = handle.metadata_block_ids(&mut item_ids, item_type);
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, item_data);
    let md_content_type = handle.metadata_content_type(item_ids[0]);
    // content_type is stored in HEIF only for "mime" type of meta data.
    assert_eq!(md_content_type, Some(""));

    // Exif
    let count = handle.metadata_block_ids(&mut item_ids, b"Exif");
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, b"\0\0\0\0MM\0*FakeExif");

    // Xmp
    let count = handle.metadata_block_ids(&mut item_ids, b"mime");
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, item_data);
    let md_content_type = handle.metadata_content_type(item_ids[0]);
    assert_eq!(md_content_type, Some("application/rdf+xml"));

    Ok(())
}
