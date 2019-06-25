# Change Log

## [Unreleased] - ReleaseDate

- Specified lifetime of ``ImageHandle``. Now it depends from ``HeifContext``.
- ``HeifContext`` implements the ``Send`` trait now.

## [0.2.1] - 2019-06-24

- Fixed filtering of metadata blocks by type.

## [0.2.0] - 2019-06-18

- Changed URL of the crate documentation.
- Added small example of usage into README.md.
- Changed some enum values and name of methods to comply with the Rust
  naming conventions.
- Methods ``Encoder::set_lossless()`` and ``Encoder::set_lossy_quality()``
  replaced by ``Encoder::set_quality()``.
- Added methods ``Image::planes()`` and ``Image::planes_mut()``.

## [0.1.0]

- Initial version.
