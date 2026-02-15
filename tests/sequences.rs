#[cfg(feature = "v1_20")]
mod v1_20 {
    use libheif_rs::{
        HeifContext, Result, ColorSpace, RgbChroma, track_types
    };

    #[test]
    fn read_from_file() -> Result<()> {
        let ctx = HeifContext::read_from_file("./data/star-8bpc.avifs")?;
        let handle = ctx.primary_image_handle()?;
        assert_eq!(handle.width(), 159);
        assert_eq!(handle.height(), 159);

        assert_eq!(ctx.number_of_top_level_images(), 1);
        assert!(ctx.has_sequence());
        assert_eq!(ctx.sequence_timescale(), 1000);
        assert_eq!(ctx.sequence_duration(), 500);
        assert_eq!(ctx.track_ids().len(), 1);

        let unavailable_track_option = ctx.track(1337);
        assert!(unavailable_track_option.is_none());

        let track_option = ctx.track(0);
        assert!(track_option.is_some());

        let track = track_option.unwrap();
        let image_resolution = track.image_resolution()?;
        assert_eq!(image_resolution.width, 159);
        assert_eq!(image_resolution.height, 159);

        assert_eq!(track.track_handler_type(), track_types::IMAGE_SEQUENCE);
        assert_eq!(track.id(), 1);
        assert_eq!(track.timescale(), 10240);

        let image = track.decode_next_image(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
        assert_eq!(image.duration(), 1024);

        Ok(())
    }
}

#[cfg(feature = "v1_21")]
mod v1_21 {
    use libheif_rs::{
        HeifContext, Result
    };

    #[test]
    fn read_from_file() -> Result<()> {
        let ctx = HeifContext::read_from_file("./data/star-8bpc.avifs")?;
        let track_option = ctx.track(0);
        assert!(track_option.is_some());

        let track = track_option.unwrap();
        assert!(!track.has_alpha_channel());

        Ok(())
    }
}