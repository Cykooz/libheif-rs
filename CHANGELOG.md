# Change Log

## [Unreleased] - ReleaseDate

- Added method ``Encoder::set_parameter_value()``.

## [0.15.0] - 2021-05-12

- ``libheif-sys`` updated to version 1.12.
- Added new value of ``HeifErrorSubCode`` enum -
  ``WrongTileImagePixelDepth``.  
- Added methods:
  ``Image::set_premultiplied_alpha()``, ``Image::is_premultiplied_alpha()``,
  ``ImageHandle::is_premultiplied_alpha()``.

# [0.14.0] - 2021-03-17

- Added new methods:
  ``ImageHandle::depth_image_ids()``, ``ImageHandle::thumbnail_ids()``,
  ``ImageHandle::metadata_block_ids()``.
- Deprecated some methods:
  ``ImageHandle::list_of_depth_image_ids()``, ``ImageHandle::list_of_thumbnail_ids()``,
  ``ImageHandle::list_of_metadata_block_ids()``.
- Added new methods for getting top level images from ``HeifContext``:
  ``HeifContext::top_level_image_ids()``, ``HeifContext::image_handle()``.

## [0.13.1] - 2021-02-03

- ``libheif-sys`` updated to version 1.11.
- Added methods: 
  ``EncodingOptions::mac_os_compatibility_workaround_no_nclx_profile()``,
  ``EncodingOptions::set_mac_os_compatibility_workaround_no_nclx_profile()``.

## [0.13.0] - 2021-01-15

### Breaking changes

- Added new value of ``HeifErrorCode`` enum -
  ``ColorProfileDoesNotExist``.

## [0.12.0] - 2021-01-14

- ``libheif-sys`` updated to version 1.10.

### Breaking changes

- All fields of ``EncodingOptions`` struct are made private. Added 
  corresponding methods for access to this fields.
- Method ``HeifContext::encode_image()`` now returns ``Result<ImageHandle>``.

## [0.11.0] - 2020-09-26

- ``Image`` has marked as ``Send``.

## [0.10.0] - 2020-08-29

- ``libheif-sys`` updated to version 1.8.
- Added new compression format - ``CompressionFormat::Av1``.
- Added new values of ``HeifErrorSubCode`` enum:
  ``InvalidFractionalNumber``, ``InvalidImageSize``,
  ``InvalidPixiBox``,  ``NoAv1cBox``.

## [0.9.2] - 2020-08-15

- Implemented ``std::error::Error`` for ``HeifError`` (paolobarbolini).

## [0.9.1] - 2020-06-16

- Removed ``num``, ``num-traits`` and ``num-derive`` from dependencies.
- Added ``enumn`` as dependency.

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
