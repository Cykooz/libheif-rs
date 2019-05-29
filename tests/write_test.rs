use failure;

use libheif_rs::{
    Channel, Chroma, ColorSpace, CompressionFormat, EncodingOptions, HeifContext, Image,
};

#[test]
fn create_and_encode_image() -> Result<(), failure::Error> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::RGB, Chroma::C444)?;

    image.create_plane(Channel::R, width, height, 8)?;
    image.create_plane(Channel::G, width, height, 8)?;
    image.create_plane(Channel::B, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_r = planes.r.unwrap();
    let stride = plane_r.stride;

    let data_r = plane_r.data;
    let data_g = planes.g.unwrap().data;
    let data_b = planes.b.unwrap().data;

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

    let mut context = HeifContext::new()?;
    let mut encoder = context.encoder_for_format(CompressionFormat::Hevc)?;

    encoder.set_lossless(true)?;
    encoder.set_lossy_quality(100)?;
    let encoding_options: EncodingOptions = Default::default();

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width);
    assert_eq!(handle.height(), height);

    // Decode the image
    let image = handle.decode(ColorSpace::RGB, Chroma::InterleavedRgb)?;
    assert_eq!(image.color_space(), ColorSpace::RGB);
    assert_eq!(image.chroma_format(), Chroma::InterleavedRgb);
    let planes = image.planes();
    let plan = planes.interleaved.unwrap();
    assert_eq!(plan.width, width);
    assert_eq!(plan.height, height);

    Ok(())
}
