use std::error::Error;
use std::io::{Seek, SeekFrom};

use image::error::{DecodingError, ImageFormatHint};
use image::hooks::GenericReader;
use image::{ColorType, ImageError, ImageResult};

use crate::{ColorSpace, HeifContext, HeifError, ImageHandle, LibHeif, RgbChroma, StreamReader};

macro_rules! magick {
    ($v1:literal, $v2:literal, $v3:literal, $v4:literal) => {
        &[
            0, 0, 0, 0, b'f', b't', b'y', b'p', 0, 0, 0, 0, $v1, $v2, $v3, $v4,
        ]
    };
}

/// HEVC image (`heic`) brand.
///
/// Image conforms to HEVC (H.265) Main or Main Still profile.
static HEIC_BRAND: &[u8] = magick!(b'h', b'e', b'i', b'c');
/// HEVC image (`heix`) brand.
///
/// Image conforms to HEVC (H.265) Main 10 profile.
static HEIX_BRAND: &[u8] = magick!(b'h', b'e', b'i', b'x');
/// AV1 image (`avif`) brand.
static AVIF_BRAND: &[u8] = magick!(b'a', b'v', b'i', b'f');
/// JPEG image sequence (`jpgs`) brand.
static JPGS_BRAND: &[u8] = magick!(b'j', b'p', b'g', b's');
/// JPEG 2000 image (`j2ki`) brand.
static J2KI_BRAND: &[u8] = magick!(b'j', b'2', b'k', b'i');
/// HEIF image structural brand (`mif1`).
///
/// This does not imply a specific coding algorithm.
static MIF1_BRAND: &[u8] = magick!(b'm', b'i', b'f', b'1');
/// HEIF image structural brand (`mif2`).
///
/// This does not imply a specific coding algorithm. `mif2` extends
/// the requirements of `mif1` to include the `rref` and `iscl` item
/// properties.
static MIF2_BRAND: &[u8] = magick!(b'm', b'i', b'f', b'2');

static MASK: Option<&'static [u8]> = Some(&[
    0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff,
]);

/// Registers the decoder with the `image` crate for heif-files.
pub fn register_heif_decoding_hook() -> bool {
    let registered = image::hooks::register_decoding_hook(
        "heif".into(),
        Box::new(|r| Ok(Box::new(HeifDecoder::new(r)?))),
    );
    if registered {
        for brand in [MIF1_BRAND, MIF2_BRAND, JPGS_BRAND, J2KI_BRAND] {
            image::hooks::register_format_detection_hook("heif".into(), brand, MASK);
        }
    }
    registered
}

/// Registers the decoder with the `image` crate for heic-files.
pub fn register_heic_decoding_hook() -> bool {
    let registered = image::hooks::register_decoding_hook(
        "heic".into(),
        Box::new(|r| Ok(Box::new(HeifDecoder::new(r)?))),
    );
    if registered {
        for brand in [HEIC_BRAND, HEIX_BRAND] {
            image::hooks::register_format_detection_hook("heic".into(), brand, MASK);
        }
    }
    registered
}

/// Registers the decoder with the `image` crate for avif-files.
pub fn register_avif_decoding_hook() -> bool {
    let registered = image::hooks::register_decoding_hook(
        "avif".into(),
        Box::new(|r| Ok(Box::new(HeifDecoder::new(r)?))),
    );
    if registered {
        image::hooks::register_format_detection_hook("avif".into(), AVIF_BRAND, MASK);
    }
    registered
}

/// Registers the decoder with the `image` crate for all file types
/// supported by the crate.
pub fn register_all_decoding_hooks() {
    register_heif_decoding_hook();
    register_heic_decoding_hook();
    register_avif_decoding_hook();
}

fn image_error(err: impl Into<Box<dyn Error + Send + Sync>>) -> ImageError {
    ImageError::Decoding(DecodingError::new(
        ImageFormatHint::Name("heif".into()),
        err,
    ))
}

impl From<HeifError> for ImageError {
    fn from(e: HeifError) -> Self {
        image_error(e)
    }
}

struct HeifDecoder<'a> {
    _context: HeifContext<'a>,
    image_handle: ImageHandle,
    color_type: ColorType,
}

impl<'a> HeifDecoder<'a> {
    fn new(mut reader: GenericReader<'a>) -> ImageResult<HeifDecoder<'a>> {
        reader.seek(SeekFrom::End(0))?;
        let total_size = reader.stream_position()?;
        reader.seek(SeekFrom::Start(0))?;
        let stream_reader = StreamReader::new(reader, total_size);
        let context = HeifContext::read_from_reader(Box::new(stream_reader))?;
        let image_handle = context.primary_image_handle()?;
        let color_type = get_color_type(&image_handle)?;

        Ok(Self {
            _context: context,
            image_handle,
            color_type,
        })
    }
}

fn get_color_type(image_handle: &ImageHandle) -> ImageResult<ColorType> {
    let has_alpha = image_handle.has_alpha_channel();
    let color_space = image_handle.preferred_decoding_colorspace()?;
    let (is_monochrome, is_hdr) = match color_space {
        ColorSpace::YCbCr(_) => {
            let bit_depth = image_handle.luma_bits_per_pixel();
            (false, bit_depth > 8)
        }
        ColorSpace::Rgb(chroma) => match chroma {
            RgbChroma::C444 | RgbChroma::Rgb | RgbChroma::Rgba => (false, false),
            RgbChroma::HdrRgbBe
            | RgbChroma::HdrRgbaBe
            | RgbChroma::HdrRgbLe
            | RgbChroma::HdrRgbaLe => (false, true),
        },
        ColorSpace::Monochrome => {
            let bit_depth = image_handle.luma_bits_per_pixel();
            (true, bit_depth > 8)
        }
        ColorSpace::Undefined => {
            let bit_depth = image_handle.luma_bits_per_pixel();
            (false, bit_depth > 8)
        }
        ColorSpace::NonVisual => {
            return Err(image_error("Container doesn't have image data."));
        }
    };
    let color_type = match (is_monochrome, has_alpha, is_hdr) {
        (false, false, false) => ColorType::Rgb8,
        (false, false, true) => ColorType::Rgb16,
        (false, true, false) => ColorType::Rgba8,
        (false, true, true) => ColorType::Rgba16,
        (true, false, false) => ColorType::L8,
        (true, false, true) => ColorType::L16,
        (true, true, false) => ColorType::La8,
        (true, true, true) => ColorType::La16,
    };
    Ok(color_type)
}

fn get_color_space(color_type: ColorType) -> ColorSpace {
    let is_target_little_endian = u16::from_ne_bytes([1, 0]) == 1;
    match color_type {
        ColorType::L8 | ColorType::La8 | ColorType::L16 | ColorType::La16 => ColorSpace::Monochrome,
        ColorType::Rgb8 => ColorSpace::Rgb(RgbChroma::Rgb),
        ColorType::Rgba8 => ColorSpace::Rgb(RgbChroma::Rgba),
        ColorType::Rgb16 => {
            if is_target_little_endian {
                ColorSpace::Rgb(RgbChroma::HdrRgbLe)
            } else {
                ColorSpace::Rgb(RgbChroma::HdrRgbBe)
            }
        }
        ColorType::Rgba16 => {
            if is_target_little_endian {
                ColorSpace::Rgb(RgbChroma::HdrRgbaLe)
            } else {
                ColorSpace::Rgb(RgbChroma::HdrRgbaBe)
            }
        }
        _ => ColorSpace::Rgb(RgbChroma::Rgb),
    }
}

impl<'a> image::ImageDecoder for HeifDecoder<'a> {
    fn dimensions(&self) -> (u32, u32) {
        (self.image_handle.width(), self.image_handle.height())
    }

    fn color_type(&self) -> ColorType {
        self.color_type
    }

    fn read_image(self, buf: &mut [u8]) -> ImageResult<()>
    where
        Self: Sized,
    {
        let color_space = get_color_space(self.color_type);
        let img = LibHeif::new().decode(&self.image_handle, color_space, None)?;
        if !matches!(img.color_space(), Some(c) if c == color_space) {
            return Err(image_error("Color space mismatch."));
        }
        let planes = img.planes();
        let Some(plane) = planes.interleaved else {
            return Err(image_error("Image is not interleaved."));
        };

        let row_size = plane.width as usize * (plane.storage_bits_per_pixel / 8) as usize;
        if row_size > plane.stride {
            return Err(image_error("Row size is greater than stride."));
        }
        let dst_rows = buf.chunks_exact_mut(row_size);
        let src_rows = plane
            .data
            .chunks_exact(plane.stride)
            .take(plane.height as usize)
            .map(|row| &row[..row_size]);
        for (dst_row, src_row) in dst_rows.zip(src_rows) {
            dst_row.copy_from_slice(src_row);
        }
        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        (*self).read_image(buf)
    }
}
