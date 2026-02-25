use libheif_rs::{ColorSpace, HeifContext, Result, RgbChroma};

#[cfg(feature = "v1_20")]
mod v1_20 {
    use libheif_rs::{track_types, Chroma};

    use super::*;

    #[test]
    fn test_sequence() -> Result<()> {
        let ctx = HeifContext::read_from_file("./data/star-8bpc.avifs")?;
        let handle = ctx.primary_image_handle()?;
        assert_eq!(handle.width(), 159);
        assert_eq!(handle.height(), 159);

        assert_eq!(ctx.image_ids().len(), 1);
        assert!(ctx.has_sequence());
        assert_eq!(ctx.sequence_timescale(), 1000);
        assert_eq!(ctx.sequence_duration(), 500);
        let track_ids = ctx.track_ids();
        assert_eq!(track_ids.len(), 1);

        let unavailable_track = ctx.track(1337);
        assert!(unavailable_track.is_none());

        let first_visual_track = ctx.track(0).unwrap();
        assert_eq!(first_visual_track.id(), track_ids[0]);

        let track = ctx.track(track_ids[0]).unwrap();
        let image_resolution = track.image_resolution()?;
        assert_eq!(image_resolution.width, 159);
        assert_eq!(image_resolution.height, 159);

        assert_eq!(track.handler_type(), track_types::IMAGE_SEQUENCE);
        assert_eq!(track.id(), 1);
        assert_eq!(track.timescale(), 10240);

        let image = track.decode_next_image(ColorSpace::Undefined, None)?;
        assert_eq!(image.duration(), 1024);
        assert_eq!(image.color_space(), Some(ColorSpace::YCbCr(Chroma::C420)));

        Ok(())
    }
}

#[cfg(feature = "v1_21")]
mod v1_21 {
    use super::*;

    #[test]
    fn track_has_alpha_channel() -> Result<()> {
        let ctx = HeifContext::read_from_file("./data/star-8bpc.avifs")?;
        let track = ctx.track(0).unwrap();
        assert!(!track.has_alpha_channel());
        Ok(())
    }
}
