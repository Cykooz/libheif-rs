use failure;

use libheif_rs::{
    Channel, Chroma, ColorSpace, CompressionFormat, EncodingOptions, HeifContext, Image,
};

#[test]
fn create_and_encode_image() -> Result<(), failure::Error> {
    let width = 640;
    let height = 480;
    let bit_depth = 24;

    let mut image = Image::new(width, height, ColorSpace::RGB, Chroma::InterleavedRgb)?;
    image.add_plane(Channel::Interleaved, width, height, bit_depth)?;

    let (data, stride) = image.plane_mut(Channel::Interleaved);
    for y in 0..height {
        let mut row_start = (y * stride) as usize;
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

    encoder.set_lossless(true)?;
    encoder.set_lossy_quality(100)?;
    let encoding_options: EncodingOptions = Default::default();

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width as u32);
    assert_eq!(handle.height(), height as u32);

    // Decode the image
    let image = handle.decode(ColorSpace::RGB, Chroma::InterleavedRgb)?;
    assert_eq!(image.color_space(), ColorSpace::RGB);
    assert_eq!(image.chroma_format(), Chroma::InterleavedRgb);
    assert_eq!(image.width(Channel::Interleaved), width);
    assert_eq!(image.height(Channel::Interleaved), height);

    Ok(())
}
