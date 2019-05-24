# libheif-rs

Safe wrapper to libheif-sys for parsing heif/heic files

## Installing

Ubuntu
```
sudo apt-get install libheif-dev
```

## Example of usage

```rust
use failure;
use libheif_rs::{Channel, Chroma, ColorSpace, HeifContext};

fn main() -> Result<(), failure::Error> {
    let ctx = HeifContext::read_from_file("./data/test.heic")?;
    let handle = ctx.get_primary_image_handle()?;

    // Decode the image
    let image = handle.decode(ColorSpace::Rgb, Chroma::InterleavedRgb)?;
    assert_eq!(image.get_color_space(), ColorSpace::Rgb);
    assert_eq!(image.get_chroma_format(), Chroma::InterleavedRgb);
    assert_eq!(image.width(Channel::Interleaved), 3024);
    assert_eq!(image.height(Channel::Interleaved), 4032);

    // Scale the image
    let small_img = image.scale(1024, 800, None)?;
    assert_eq!(small_img.width(Channel::Interleaved), 1024);
    assert_eq!(small_img.height(Channel::Interleaved), 800);

    Ok(())
}
```
