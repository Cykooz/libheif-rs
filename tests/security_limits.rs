mod write_test;

#[cfg(feature = "v1_19")]
mod v1_19 {
    use libheif_rs::{HeifContext, Result, SecurityLimits};

    #[test]
    fn get_default_security_limits() {
        let limits = SecurityLimits::new();
        assert_eq!(limits.max_image_size_pixels(), 1073741824);
        assert_eq!(limits.max_items(), 1000);
    }

    #[test]
    fn context_security_limits() -> Result<()> {
        let mut ctx = HeifContext::new()?;
        let mut limits = ctx.security_limits();
        assert_eq!(limits.max_image_size_pixels(), 1073741824);
        assert_eq!(limits.max_items(), 1000);

        limits.set_max_image_size_pixels(32768 ^ 3);
        limits.set_max_items(10000);
        ctx.set_security_limits(&limits)?;

        let limits = ctx.security_limits();
        assert_eq!(limits.max_image_size_pixels(), 32768 ^ 3);
        assert_eq!(limits.max_items(), 10000);

        Ok(())
    }
}
