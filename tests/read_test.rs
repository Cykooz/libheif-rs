use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use exif::parse_exif;

use libheif_rs::{
    check_file_type, color_profile_types, Chroma, ColorPrimaries, ColorProfile, ColorSpace,
    CompressionFormat, EncoderParameterValue, EncoderQuality, FileTypeResult, HeifContext,
    ImageHandle, ItemId, MatrixCoefficients, Result, RgbChroma, StreamReader,
    TransferCharacteristics,
};

#[test]
fn read_from_file() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 3024);
    assert_eq!(handle.height(), 4032);

    Ok(())
}

#[test]
fn read_from_reader() -> Result<()> {
    let mut file = BufReader::new(File::open("./data/test.heic").unwrap());
    let total_size = file.seek(SeekFrom::End(0)).unwrap();
    file.rewind().unwrap();
    let stream_reader = StreamReader::new(file, total_size);

    let ctx = HeifContext::read_from_reader(Box::new(stream_reader))?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 3024);
    assert_eq!(handle.height(), 4032);

    let src_img = handle.decode(ColorSpace::Undefined, None)?;
    assert_eq!(
        src_img.color_space(),
        Some(ColorSpace::Rgb(RgbChroma::C444))
    );

    Ok(())
}

#[test]
fn get_image_handler() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;

    // Get a handle to the primary image
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 3024);
    assert_eq!(handle.height(), 4032);
    assert_eq!(handle.ispe_width(), 4032);
    assert_eq!(handle.ispe_height(), 3024);
    assert!(!handle.has_alpha_channel());
    assert!(!handle.is_premultiplied_alpha());
    assert!(handle.is_primary());
    assert_eq!(handle.luma_bits_per_pixel(), 8);
    assert_eq!(handle.chroma_bits_per_pixel(), 8);
    assert!(!handle.has_depth_image());
    assert_eq!(handle.number_of_depth_images(), 0);
    let mut image_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.depth_image_ids(&mut image_ids);
    assert_eq!(count, 0);
    Ok(())
}

#[test]
fn get_thumbnail() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let thumb_handle: ImageHandle;

    // All instances of ImageHandle MUST have lifetime of their
    // parent HeifContext instance, but not lifetime of parent
    // ImageHandle instance.
    {
        let handle = ctx.primary_image_handle()?;
        // Thumbnails
        assert_eq!(handle.number_of_thumbnails(), 1);
        let mut thumb_ids: Vec<ItemId> = vec![0; 2];
        let count = handle.thumbnail_ids(&mut thumb_ids);
        assert_eq!(count, 1);
        thumb_handle = handle.thumbnail(thumb_ids[0])?;
    }

    assert_eq!(thumb_handle.width(), 240);
    assert_eq!(thumb_handle.height(), 320);
    Ok(())
}

#[test]
fn get_exif() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.primary_image_handle()?;

    // Metadata blocks
    assert_eq!(handle.number_of_metadata_blocks(0), 1);
    let mut meta_ids: Vec<ItemId> = vec![0; 2];
    let count = handle.metadata_block_ids(&mut meta_ids, 0);
    assert_eq!(count, 1);
    let meta_type = handle.metadata_type(meta_ids[0]);
    assert_eq!(meta_type, Some("Exif"));
    let meta_content_type = handle.metadata_content_type(meta_ids[0]);
    assert_eq!(meta_content_type, Some(""));
    assert_eq!(handle.metadata_size(meta_ids[0]), 2030);

    assert_eq!(handle.number_of_metadata_blocks(b"Unkn"), 0);
    let count = handle.metadata_block_ids(&mut meta_ids, b"Unkn");
    assert_eq!(count, 0);

    // Exif
    assert_eq!(handle.number_of_metadata_blocks(b"Exif"), 1);
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    assert_eq!(count, 1);
    let exif = handle.metadata(meta_ids[0])?;
    assert_eq!(exif.len(), 2030);
    assert_eq!(exif[0..4], [0, 0, 0, 6]);
    let tiff_exif = &exif[10..]; // Skip header
    let (exif_fields, is_le) = parse_exif(tiff_exif).unwrap();
    assert!(!is_le);
    assert_eq!(exif_fields.len(), 56);

    Ok(())
}

#[test]
fn decode_and_scale_image() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.primary_image_handle()?;

    // Decode the image
    let src_img = handle.decode(ColorSpace::YCbCr(Chroma::C420), None)?;
    assert_eq!(src_img.color_space(), Some(ColorSpace::YCbCr(Chroma::C420)));
    let planes = src_img.planes();
    let y_plane = planes.y.unwrap();
    assert_eq!(y_plane.width, 3024);
    assert_eq!(y_plane.height, 4032);

    let cb_plane = planes.cb.unwrap();
    assert_eq!(cb_plane.width, 3024 / 2);
    assert_eq!(cb_plane.height, 4032 / 2);

    let cr_plane = planes.cr.unwrap();
    assert_eq!(cr_plane.width, 3024 / 2);
    assert_eq!(cr_plane.height, 4032 / 2);

    // Scale the image
    let img = src_img.scale(1024, 800, None)?;
    let planes = img.planes();
    let y_plane = planes.y.unwrap();
    assert_eq!(y_plane.width, 1024);
    assert_eq!(y_plane.height, 800);
    assert!(!y_plane.data.is_empty());
    assert!(y_plane.stride > 0);

    let cb_plane = planes.cb.unwrap();
    assert_eq!(cb_plane.width, 1024 / 2);
    assert_eq!(cb_plane.height, 800 / 2);

    let cr_plane = planes.cr.unwrap();
    assert_eq!(cr_plane.width, 1024 / 2);
    assert_eq!(cr_plane.height, 800 / 2);

    Ok(())
}

#[test]
fn top_level_images() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/4_chunks-wo_exif.heic")?;
    assert_eq!(ctx.number_of_top_level_images(), 5);
    let mut image_ids: Vec<ItemId> = vec![0; 6];
    let count = ctx.top_level_image_ids(&mut image_ids);
    assert_eq!(count, 5);

    for &image_id in image_ids[0..4].iter() {
        let handle = ctx.image_handle(image_id)?;
        assert_eq!(handle.width(), 480);
    }

    // Primary image
    let handle = ctx.image_handle(image_ids[4])?;
    assert_eq!(handle.width(), 960);

    Ok(())
}

#[test]
fn test_encoder() -> Result<()> {
    let ctx = HeifContext::new()?;

    let mut encoder = ctx.encoder_for_format(CompressionFormat::Hevc)?;
    assert!(encoder.name().starts_with("x265 HEVC encoder"));

    let mut params = encoder.parameters_names();
    params.sort();
    assert_eq!(params.len(), 7);
    let expect = vec![
        "chroma".to_string(),
        "complexity".to_string(),
        "lossless".to_string(),
        "preset".to_string(),
        "quality".to_string(),
        "tu-intra-depth".to_string(),
        "tune".to_string(),
    ];
    assert_eq!(params, expect);

    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(false))
    );

    encoder.set_quality(EncoderQuality::LossLess)?;
    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(true))
    );

    //    let expect = vec!{
    //        "quality".to_string() => EncoderParameterValue::Int(50),
    //        "lossless".to_string() => EncoderParameterValue::Bool(false),
    //        "preset".to_string() => EncoderParameterValue::String("slow".to_string()),
    //        "tune".to_string() => EncoderParameterValue::String("ssim".to_string()),
    //        "tu-intra-depth".to_string() => EncoderParameterValue::Int(2),
    //        "complexity".to_string() => EncoderParameterValue::Int(0),
    //    };

    //    encoder.set_lossless(true)?;
    //
    //    assert_eq!(encoder.get_parameter("lossless")?, Some(&EncoderParameterValue::Bool(true)));

    Ok(())
}

#[test]
fn test_check_file_type() {
    let mut data = vec![0u8; 16];
    assert_eq!(check_file_type(&data), FileTypeResult::No);

    let mut f = File::open("./data/test.heic").unwrap();
    let len = f.read(&mut data).unwrap();
    assert_eq!(len, 16);

    assert_eq!(check_file_type(&data[..7]), FileTypeResult::MayBe);
    assert_eq!(check_file_type(&data[..8]), FileTypeResult::MayBe);
    assert_eq!(check_file_type(&data[..12]), FileTypeResult::Supported);
}

#[test]
fn test_raw_color_profile_of_image_handle() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.primary_image_handle()?;

    let raw_profile = handle.color_profile_raw().unwrap();
    assert_eq!(raw_profile.profile_type(), color_profile_types::PROF);
    assert_eq!(raw_profile.data.len(), 548);

    let nclx_profile = handle.color_profile_nclx();
    assert!(nclx_profile.is_none());
    Ok(())
}

#[test]
fn test_nclx_color_profile_of_image_handle() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test_nclx.heif")?;
    let handle = ctx.primary_image_handle()?;

    let raw_profile = handle.color_profile_raw();
    assert!(raw_profile.is_none());

    let nclx_profile = handle.color_profile_nclx().unwrap();
    assert_eq!(nclx_profile.profile_type(), color_profile_types::NCLX);
    assert_eq!(
        nclx_profile.color_primaries(),
        ColorPrimaries::ITU_R_BT_2020_2_and_2100_0
    );
    assert_eq!(
        nclx_profile.transfer_characteristics(),
        TransferCharacteristics::ITU_R_BT_2100_0_PQ
    );
    assert_eq!(
        nclx_profile.matrix_coefficients(),
        MatrixCoefficients::RGB_GBR
    );
    assert_eq!(nclx_profile.full_range_flag(), 1);
    assert_eq!(nclx_profile.color_primary_red_x(), 0.708);
    assert_eq!(nclx_profile.color_primary_red_y(), 0.292);
    assert_eq!(nclx_profile.color_primary_green_x(), 0.17);
    assert_eq!(nclx_profile.color_primary_green_y(), 0.797);
    assert_eq!(nclx_profile.color_primary_blue_x(), 0.131);
    assert_eq!(nclx_profile.color_primary_blue_y(), 0.046);
    assert_eq!(nclx_profile.color_primary_white_x(), 0.3127);
    assert_eq!(nclx_profile.color_primary_white_y(), 0.329);
    Ok(())
}

#[test]
fn test_raw_color_profile_of_image() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.primary_image_handle()?;
    let image = handle.decode(ColorSpace::Undefined, None)?;
    // Hmm... It's strange. Decoded image doesn't have color profile.
    assert!(image.color_profile_raw().is_none());
    assert!(image.color_profile_nclx().is_none());
    Ok(())
}

#[test]
fn test_read_avif_image() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test_nclx.avif")?;
    let handle = ctx.primary_image_handle()?;

    assert_eq!(handle.width(), 2048);
    assert_eq!(handle.height(), 1440);
    assert_eq!(handle.luma_bits_per_pixel(), 12);
    assert_eq!(handle.chroma_bits_per_pixel(), 12);

    let nclx_profile = handle.color_profile_nclx();
    assert!(nclx_profile.is_some());

    let image = handle.decode(ColorSpace::Undefined, None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::C444)));
    let planes = image.planes();
    let r_plane = planes.r.unwrap();
    assert_eq!(r_plane.width, 2048);
    assert_eq!(r_plane.height, 1440);

    Ok(())
}
