use libheif_sys as lh;
use std::ptr;

use crate::utils::get_non_null_ptr;
use crate::{HeifError, Image, ItemId, Result};

pub struct PointRegion(ptr::NonNull<lh::heif_region>);

impl PointRegion {
    /// Get the values for a point region.
    ///  
    /// This returns the coordinates in the reference coordinate space
    /// (from the parent region item).
    pub fn point(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            lh::heif_region_get_point(self.0.as_ptr(), &mut x, &mut y);
        }
        (x, y)
    }

    /// Get the transformed values for a point region.
    ///
    /// This returns the coordinates in pixels after all transformative
    /// properties have been applied.
    pub fn point_transformed(&self, image_id: ItemId) -> (f64, f64) {
        let mut x = 0.;
        let mut y = 0.;
        unsafe {
            lh::heif_region_get_point_transformed(self.0.as_ptr(), image_id, &mut x, &mut y);
        }
        (x, y)
    }
}

pub struct RectangleRegion(ptr::NonNull<lh::heif_region>);

/// Rectangle geometry.
///
/// The region is represented by a left top corner position,
/// and a size defined by width and height.
/// All the interior points and the edge are part of the region.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct RectangleTransformed {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl RectangleRegion {
    /// Get the values for a rectangle region.
    ///
    /// This returns the values in the reference coordinate space
    /// (from the parent region item).
    pub fn rectangle(&self) -> Rectangle {
        let mut res = Rectangle::default();
        unsafe {
            lh::heif_region_get_rectangle(
                self.0.as_ptr(),
                &mut res.left,
                &mut res.top,
                &mut res.width,
                &mut res.height,
            );
        }
        res
    }

    /// Get the transformed values for a rectangle region.
    ///
    /// This returns the coordinates in pixels after all transformative
    /// properties have been applied.
    pub fn rectangle_transformed(&self, image_id: ItemId) -> RectangleTransformed {
        let mut res = RectangleTransformed::default();
        unsafe {
            lh::heif_region_get_rectangle_transformed(
                self.0.as_ptr(),
                image_id,
                &mut res.left,
                &mut res.top,
                &mut res.width,
                &mut res.height,
            );
        }
        res
    }
}

pub struct EllipseRegion(ptr::NonNull<lh::heif_region>);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub x_radius: u32,
    pub y_radius: u32,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct EllipseTransformed {
    pub x: f64,
    pub y: f64,
    pub x_radius: f64,
    pub y_radius: f64,
}

impl EllipseRegion {
    /// Get the values for an ellipse region.
    ///
    /// This returns the values in the reference coordinate space (from
    /// the parent region item).
    /// The ellipse is represented by a center position, and a size defined
    /// by radii in the X and Y directions. All the interior points and
    /// the edge are part of the region.
    pub fn ellipse(&self) -> Ellipse {
        let mut res = Ellipse::default();
        unsafe {
            lh::heif_region_get_ellipse(
                self.0.as_ptr(),
                &mut res.x,
                &mut res.y,
                &mut res.x_radius,
                &mut res.y_radius,
            );
        }
        res
    }

    /// Get the transformed values for an ellipse region.
    ///
    /// This returns the coordinates in pixels after all transformative
    /// properties have been applied.
    /// The ellipse is represented by a center position, and a size defined
    /// by radii in the X and Y directions. All the interior points and
    /// the edge are part of the region.
    pub fn ellipse_transformed(&self, image_id: ItemId) -> EllipseTransformed {
        let mut res = EllipseTransformed::default();
        unsafe {
            lh::heif_region_get_ellipse_transformed(
                self.0.as_ptr(),
                image_id,
                &mut res.x,
                &mut res.y,
                &mut res.x_radius,
                &mut res.y_radius,
            );
        }
        res
    }
}

pub struct PolygonRegion(ptr::NonNull<lh::heif_region>);

impl PolygonRegion {
    /// Get the points in a polygon region.
    ///
    /// This returns the values in the reference coordinate space (from
    /// the parent region item).
    ///
    /// A polygon is a sequence of points that form a closed shape.
    /// The first point does not need to be repeated as the last point.
    /// All the interior points and the edge are part of the region.
    /// The points are returned as pairs of (X, Y) coordinates.
    pub fn polygon_points(&self) -> Vec<(i32, i32)> {
        let num_points = unsafe { lh::heif_region_get_polygon_num_points(self.0.as_ptr()) };
        let size = num_points.max(0) as usize;
        let mut points: Vec<(i32, i32)> = Vec::with_capacity(size);
        if size > 0 {
            unsafe {
                lh::heif_region_get_polygon_points(self.0.as_ptr(), points.as_mut_ptr() as _);
                points.set_len(size);
            }
        }
        points
    }

    /// Get the transformed points in a polygon region.
    ///
    /// This returns the coordinates in pixels after all transformative
    /// properties have been applied.
    ///
    /// A polygon is a sequence of points that form a closed shape.
    /// The first point does not need to be repeated as the last point.
    /// All the interior points and the edge are part of the region.
    /// The points are returned as pairs of (X, Y) coordinates.
    pub fn polygon_points_transformed(&self, image_id: ItemId) -> Vec<(f64, f64)> {
        let num_points = unsafe { lh::heif_region_get_polygon_num_points(self.0.as_ptr()) };
        let size = num_points.max(0) as usize;
        let mut points: Vec<(f64, f64)> = Vec::with_capacity(size);
        if size > 0 {
            unsafe {
                lh::heif_region_get_polygon_points_transformed(
                    self.0.as_ptr(),
                    image_id,
                    points.as_mut_ptr() as _,
                );
                points.set_len(size);
            }
        }
        points
    }
}

pub struct PolylineRegion(ptr::NonNull<lh::heif_region>);

impl PolylineRegion {
    /// Get the points in a polyline region.
    ///
    /// This returns the values in the reference coordinate space (from
    /// the parent region item).
    ///
    /// A polyline is a sequence of points that does not form a closed shape.
    /// Even if the polyline is closed, the only points that are part of
    /// the region are those that intersect (even minimally) a one-pixel line
    /// drawn along the polyline.
    /// The points are returned as pairs of (X, Y) coordinates.
    pub fn polyline_points(&self) -> Vec<(i32, i32)> {
        let num_points = unsafe { lh::heif_region_get_polyline_num_points(self.0.as_ptr()) };
        let size = num_points.max(0) as usize;
        let mut points: Vec<(i32, i32)> = Vec::with_capacity(size);
        if size > 0 {
            unsafe {
                lh::heif_region_get_polyline_points(self.0.as_ptr(), points.as_mut_ptr() as _);
                points.set_len(size);
            }
        }
        points
    }

    /// Get the transformed points in a polyline region.
    ///
    /// This returns the coordinates in pixels after all transformative
    /// properties have been applied.
    ///
    /// A polyline is a sequence of points that does not form a closed shape.
    /// Even if the polyline is closed, the only points that are part of the
    /// region are those that intersect (even minimally) a one-pixel line
    /// drawn along the polyline.
    /// The points are returned as pairs of (X, Y) coordinates.
    pub fn polyline_points_transformed(&self, image_id: ItemId) -> Vec<(f64, f64)> {
        let num_points = unsafe { lh::heif_region_get_polyline_num_points(self.0.as_ptr()) };
        let size = num_points.max(0) as usize;
        let mut points: Vec<(f64, f64)> = Vec::with_capacity(size);
        if size > 0 {
            unsafe {
                lh::heif_region_get_polyline_points_transformed(
                    self.0.as_ptr(),
                    image_id,
                    points.as_mut_ptr() as _,
                );
                points.set_len(size);
            }
        }
        points
    }
}

pub struct ReferencedMaskRegion(ptr::NonNull<lh::heif_region>);

impl ReferencedMaskRegion {
    /// Get a referenced item mask region.
    pub fn referenced_mask(&self) -> (Rectangle, ItemId) {
        let mut rectangle = Rectangle::default();
        let mut item_id = 0;
        unsafe {
            lh::heif_region_get_referenced_mask_ID(
                self.0.as_ptr(),
                &mut rectangle.left,
                &mut rectangle.top,
                &mut rectangle.width,
                &mut rectangle.height,
                &mut item_id,
            );
        }
        (rectangle, item_id)
    }
}

pub struct InlineMaskRegion(ptr::NonNull<lh::heif_region>);

impl InlineMaskRegion {
    /// Get data for an inline mask region.
    ///
    /// This returns the values in the reference coordinate space
    /// (from the parent region item).
    /// The mask location is represented by a left top corner position,
    /// and a size defined by a width and height.
    ///
    /// The mask is held as inline data on the region, one bit per pixel,
    /// the most significant bit first pixel, no padding. If the bit value is
    /// `1`, the corresponding pixel is part of the region. If the bit value
    /// is `0`, the corresponding pixel is not part of the region.
    pub fn inline_mask_data(&self) -> (Rectangle, Vec<u8>) {
        let mut rectangle = Rectangle::default();
        let size = unsafe { lh::heif_region_get_inline_mask_data_len(self.0.as_ptr()) };
        let mut data: Vec<u8> = Vec::with_capacity(size);
        unsafe {
            lh::heif_region_get_inline_mask_data(
                self.0.as_ptr(),
                &mut rectangle.left,
                &mut rectangle.top,
                &mut rectangle.width,
                &mut rectangle.height,
                data.as_mut_ptr(),
            );
            data.set_len(size);
        }
        (rectangle, data)
    }
}

#[non_exhaustive]
pub enum Region {
    /// Point geometry.
    ///
    /// The region is represented by a single point.
    Point(PointRegion),
    /// Rectangle geometry.
    ///
    /// The region is represented by a top left position,
    /// and a size defined by a width and height.
    /// All the interior points and the edge are
    /// part of the region.
    Rectangle(RectangleRegion),
    /// Ellipse geometry.
    ///
    /// The region is represented by a center point,
    /// and radii in the X and Y directions.
    /// All the interior points and the edge are part of the region.
    Ellipse(EllipseRegion),
    /// Polygon geometry.
    ///
    /// The region is represented by a sequence of points,
    /// which is considered implicitly closed.
    /// All the interior points and the edge are part of the region.
    Polygon(PolygonRegion),
    /// Polyline geometry.
    ///
    /// The region is represented by a sequence of points, which are not
    /// considered to form a closed surface. Only the edge is part of
    /// the region.
    Polyline(PolylineRegion),
    /// Referenced mask.
    ///
    /// The mask location is represented by a left-top corner position,
    /// and a size defined by a width and height. The value of each sample
    /// in that mask identifies whether the corresponding pixel is
    /// part of the region.
    ///
    /// The region geometry is described by the pixels in another image item,
    /// which has an item reference of type `mask` from the region item to
    /// the image item containing the mask.
    ///
    /// The image item containing the mask is one of:
    /// - a mask item (see ISO/IEC 23008-12:2022 Section 6.10.2), or a derived
    ///   image from a mask item;
    /// - an image item in monochrome format (4:0:0 chroma);
    /// - an image item in format of color with luma and chroma planes
    ///   (e.g. 4:2:0).
    ///
    /// If the pixel value is equal to the minimum sample
    /// value (e.g. 0 for unsigned integer), the pixel is not part of
    /// the region. If the pixel value is equal to the maximum sample
    /// value (e.g. 255 for 8 bit unsigned integer), the pixel is part of
    /// the region. If the pixel value is between the minimum sample value
    /// and maximum sample value, the pixel value represents
    /// an (application defined) probability that the pixel is part of
    /// the region, where higher pixel values correspond to higher
    /// probability values.
    ReferencedMask(ReferencedMaskRegion),
    /// Inline mask.
    ///
    /// The region geometry is described by a sequence of bits stored in inline
    /// in the region, one bit per pixel. If the bit value is `1`, the pixel is
    /// part of the region. If the bit value is `0`, the pixel is not part of
    /// the region.
    InlineMask(InlineMaskRegion),
}

impl Region {
    fn new(region_ptr: ptr::NonNull<lh::heif_region>) -> Option<Self> {
        let region_type = unsafe { lh::heif_region_get_type(region_ptr.as_ref()) };
        match region_type {
            lh::heif_region_type_heif_region_type_point => {
                Some(Region::Point(PointRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_rectangle => {
                Some(Region::Rectangle(RectangleRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_ellipse => {
                Some(Region::Ellipse(EllipseRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_polygon => {
                Some(Region::Polygon(PolygonRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_polyline => {
                Some(Region::Polyline(PolylineRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_referenced_mask => {
                Some(Region::ReferencedMask(ReferencedMaskRegion(region_ptr)))
            }
            lh::heif_region_type_heif_region_type_inline_mask => {
                Some(Region::InlineMask(InlineMaskRegion(region_ptr)))
            }
            _ => None,
        }
    }

    fn inner(&self) -> *mut lh::heif_region {
        match self {
            Region::Point(region) => region.0.as_ptr(),
            Region::Rectangle(region) => region.0.as_ptr(),
            Region::Ellipse(region) => region.0.as_ptr(),
            Region::Polygon(region) => region.0.as_ptr(),
            Region::Polyline(region) => region.0.as_ptr(),
            Region::ReferencedMask(region) => region.0.as_ptr(),
            Region::InlineMask(region) => region.0.as_ptr(),
        }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { lh::heif_region_release(self.inner()) }
    }
}

/// Region item.
///
/// See ISO/IEC 23008-12:2022 Section 6.10 "Region items and region annotations"
pub struct RegionItem {
    pub(crate) inner: *mut lh::heif_region_item,
}

impl Drop for RegionItem {
    fn drop(&mut self) {
        unsafe {
            lh::heif_region_item_release(self.inner);
        }
    }
}

impl RegionItem {
    pub(crate) fn new(inner: ptr::NonNull<lh::heif_region_item>) -> Self {
        Self {
            inner: inner.as_ptr(),
        }
    }

    pub fn id(&self) -> ItemId {
        unsafe { lh::heif_region_item_get_id(self.inner) }
    }

    /// Get the reference size for a region item.
    ///
    /// The reference size specifies the coordinate space used for
    /// the region items.
    /// When the reference size does not match the image size, the regions
    /// need to be scaled to correspond.
    pub fn reference_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            lh::heif_region_item_get_reference_size(self.inner, &mut width, &mut height);
        }
        (width, height)
    }

    /// Get the regions that are part of a region item.
    pub fn regions(&self) -> Vec<Region> {
        let num_regions = unsafe { lh::heif_region_item_get_number_of_regions(self.inner) };
        let size = num_regions.max(0) as usize;
        let mut region_ptrs: Vec<*mut lh::heif_region> = Vec::with_capacity(size);
        let mut regions: Vec<Region> = Vec::with_capacity(size);
        if size > 0 {
            unsafe {
                lh::heif_region_item_get_list_of_regions(
                    self.inner,
                    region_ptrs.as_mut_ptr(),
                    num_regions,
                );
                region_ptrs.set_len(size);
            }
            for region_ptr in region_ptrs.into_iter().filter_map(ptr::NonNull::new) {
                if let Some(region) = Region::new(region_ptr) {
                    regions.push(region);
                }
            }
        }
        regions
    }

    /// Add a point region to the region item.
    pub fn add_point(&mut self, x: i32, y: i32) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err =
            unsafe { lh::heif_region_item_add_region_point(self.inner, x, y, &mut region_ptr) };
        HeifError::from_heif_error(err)?;
        Ok(Region::Point(PointRegion(get_non_null_ptr(region_ptr)?)))
    }

    /// Add a rectangle region to the region item.
    pub fn add_rectangle(&mut self, rectangle: Rectangle) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_rectangle(
                self.inner,
                rectangle.left,
                rectangle.top,
                rectangle.width,
                rectangle.height,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::Rectangle(RectangleRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }

    /// Add an ellipse region to the region item.
    pub fn add_ellipse(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_x: u32,
        radius_y: u32,
    ) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_ellipse(
                self.inner,
                center_x,
                center_y,
                radius_x,
                radius_y,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::Ellipse(EllipseRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }

    /// Add a polygon region to the region item.
    ///
    /// A polygon is a sequence of points that form a closed shape.
    /// The first point does not need to be repeated as the last point.
    /// The points are provided as pairs of (X, Y) coordinates.
    pub fn add_polygon(&mut self, points: &[(i32, i32)]) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_polygon(
                self.inner,
                points.as_ptr() as _,
                points.len() as _,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::Polygon(PolygonRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }

    /// Add a polyline region to the region item.
    ///
    /// A polyline is a sequence of points that does not form a closed shape.
    /// Even if the polyline is closed, the only points that are part of
    /// the region are those that intersect (even minimally) a one-pixel
    /// line drawn along the polyline.
    /// The points are provided as pairs of (X, Y) coordinates.
    pub fn add_polyline(&mut self, points: &[(i32, i32)]) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_polyline(
                self.inner,
                points.as_ptr() as _,
                points.len() as _,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::Polyline(PolylineRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }

    /// Add a referenced mask region to the region item.
    pub fn add_referenced_mask(
        &mut self,
        rectangle: Rectangle,
        mask_item_id: ItemId,
    ) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_referenced_mask(
                self.inner,
                rectangle.left,
                rectangle.top,
                rectangle.width,
                rectangle.height,
                mask_item_id,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::ReferencedMask(ReferencedMaskRegion(
            get_non_null_ptr(region_ptr)?,
        )))
    }

    /// Add an inline mask region to the region item.
    ///
    /// The region geometry is described by a left top corner position,
    /// and a size defined by width and height.
    ///
    /// The mask is held as inline data on the region, one bit per pixel,
    /// the most significant bit first pixel, no padding. If the bit value
    /// is `1`, the corresponding pixel is part of the region. If the bit
    /// value is `0`, the corresponding pixel is not part of the region.
    pub fn add_inline_mask_data(
        &mut self,
        rectangle: Rectangle,
        mask_data: &[u8],
    ) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_inline_mask_data(
                self.inner,
                rectangle.left,
                rectangle.top,
                rectangle.width,
                rectangle.height,
                mask_data.as_ptr(),
                mask_data.len() as _,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::InlineMask(InlineMaskRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }

    /// Add an inline mask region image to the region item.
    ///
    /// The region geometry is described by a left top corner position,
    /// and a size defined by width and height.
    ///
    /// The mask data is held as inline data in the region, one bit per pixel.
    /// The provided image is converted to inline data, where any pixel with
    /// a value >= 0x80 becomes part of the mask region. If the image width
    /// is less than the specified `rectangle.width`, it is expanded
    /// to match the `rectangle.width` of the region (zero fill on the right).
    /// If the image height is less than the specified `rectangle.height`,
    /// it is expanded to match the `rectangle.height` of the region
    /// (zero fill on the bottom). If the image width or height is greater
    /// than the `rectangle.width` or `rectangle.height` (correspondingly)
    /// of the region, the image is cropped.
    pub fn add_inline_mask(&mut self, rectangle: Rectangle, image: &Image) -> Result<Region> {
        let mut region_ptr: *mut lh::heif_region = ptr::null_mut();
        let err = unsafe {
            lh::heif_region_item_add_region_inline_mask(
                self.inner,
                rectangle.left,
                rectangle.top,
                rectangle.width,
                rectangle.height,
                image.inner,
                &mut region_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Region::InlineMask(InlineMaskRegion(get_non_null_ptr(
            region_ptr,
        )?)))
    }
}
