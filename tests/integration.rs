#[cfg(feature = "image")]
mod image_integration_tests {
    use image::{ColorType, ImageReader};
    use libheif_rs::integration::image::register_all_decoding_hooks;

    #[test]
    fn test_image_integration() {
        let reader = ImageReader::open("./data/test.heif").unwrap();
        assert!(matches!(reader.decode(), Err(_)));

        register_all_decoding_hooks();
        let reader = ImageReader::open("./data/test.heif").unwrap();
        let image = reader.decode().unwrap();
        assert_eq!(image.width(), 1652);
        assert_eq!(image.height(), 1791);
        assert!(matches!(image.color(), ColorType::Rgb8));

        image.save("/home/cykooz/test.png").unwrap();
    }
}
