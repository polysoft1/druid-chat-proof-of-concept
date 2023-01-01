use druid::{BoxConstraints, Size};

// A convenience function to convert the width to a BoxConstraints that has the input width
/// as the max width, and the maximum height.
/// The minimum height and width are set to zero. 
pub fn to_full_height_area(width: f64, space_available: &BoxConstraints) -> BoxConstraints {
    BoxConstraints::new(
        Size::new(0.0, 0.0),
        Size::new(width, space_available.max().height)
    )
}