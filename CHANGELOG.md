# Change Log

## [0.9.0] - 2020-02-24

- Updated versions of dependencies.

### Breaking changes

- Added argument ``ignore_transformations`` into method ``ImageHandle::decode()``.

## [0.8.0] - 2019-10-03

- Added method ``ImageHandle::set_primary()``.

### Breaking changes

- Removed dependency from ``failure`` crate.
- Added type ``Result<T>`` as alias for ``std::result::Result<T, HeifError>``.
- ``ImageHandle::is_primary_image`` method renamed to ``ImageHandle::is_primary``.

## [0.7.0] - 2019-08-28

- Separate enums ``ColorSpace`` and ``Chroma`` replaced by one
  complex enum ``ColorSpace``.
- ``libheif-sys`` updated to version 1.5.

## [0.6.0] - 2019-07-17

- Added function ``check_file_type`` that checks file type by it first bytes.

## [0.5.0] - 2019-07-16

- Renamed some values of ``HeifErrorCode`` and ``HeifErrorSubCode`` enums.

## [0.4.0] - 2019-06-28

- Added method ``HeifContext::read_from_reader()`` to create context 
  form any object that implements the ``Reader`` trait.

## [0.3.0] - 2019-06-25

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
