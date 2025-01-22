use libheif_rs::{regions, HeifContext, Result};

#[test]
fn write_and_read_regions() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/test.heif")?;
    let mut handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 1652);
    assert_eq!(handle.height(), 1791);

    let mut region_item = handle.add_region_item(300, 200)?;
    region_item.add_point(1, 2)?;
    region_item.add_ellipse(3, 4, 5, 6)?;
    region_item.add_polygon(&[(7, 8), (9, 10), (11, 12)])?;
    region_item.add_polyline(&[(13, 14), (15, 16), (17, 18)])?;
    region_item.add_rectangle(regions::Rectangle {
        left: 19,
        top: 20,
        width: 21,
        height: 22,
    })?;
    let bytes = ctx.write_to_bytes()?;

    // Read the result file and check regions
    let ctx = HeifContext::read_from_bytes(&bytes)?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 1652);
    assert_eq!(handle.height(), 1791);
    let region_items = handle.region_items();
    assert_eq!(region_items.len(), 1);
    let region_item = &region_items[0];
    let regions = region_item.regions();
    assert_eq!(regions.len(), 5);
    assert!(matches!(&regions[0], regions::Region::Point(region) if region.point() == (1, 2)));
    let expected = regions::Ellipse {
        x: 3,
        y: 4,
        x_radius: 5,
        y_radius: 6,
    };
    assert!(
        matches!(&regions[1], regions::Region::Ellipse(region) if region.ellipse() == expected)
    );
    assert!(
        matches!(&regions[2], regions::Region::Polygon(region) if region.polygon_points() == [(7, 8), (9, 10), (11, 12)])
    );
    assert!(
        matches!(&regions[3], regions::Region::Polyline(region) if region.polyline_points() == [(13, 14), (15, 16), (17, 18)])
    );
    let expected = regions::Rectangle {
        left: 19,
        top: 20,
        width: 21,
        height: 22,
    };
    assert!(
        matches!(&regions[4], regions::Region::Rectangle(region) if region.rectangle() == expected)
    );

    Ok(())
}
