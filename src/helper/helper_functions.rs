use druid::{BoxConstraints, Size};
use chrono::{ Datelike, TimeZone, Timelike};

// A convenience function to convert the width to a BoxConstraints that has the input width
/// as the max width, and the maximum height.
/// The minimum height and width are set to zero. 
pub fn to_full_height_area(width: f64, space_available: &BoxConstraints) -> BoxConstraints {
    BoxConstraints::new(
        Size::new(0.0, 0.0),
        Size::new(width, space_available.max().height)
    )
}

#[derive(Clone, Copy, PartialEq, druid::Data, num_derive::FromPrimitive)]

pub enum TimestampFormat {
    TimeOnlyAmPm = 0,
    TimeOnly12,
    TimeOnly24,
    Compact12,
    Compact24,
    Full12,
    Full24,
}

pub fn timestamp_to_display_msg(epoch: i64, format: TimestampFormat) -> String {
    // Helpful reference: https://help.gnome.org/users/gthumb/stable/gthumb-date-formats.html.en
    let now = chrono::offset::Local::now();

    let local_time = chrono::Local.timestamp_opt(epoch, 0);
    match local_time {
        chrono::LocalResult::Single(local_msg_time) => {
            let same_year = now.year() == local_msg_time.year();
            let day_diff = now.ordinal0() as i32 - local_msg_time.ordinal0() as i32;
            match format {
                TimestampFormat::TimeOnly12 => {
                    return local_msg_time.format("%l:%M").to_string();
                },
                TimestampFormat::TimeOnly24 => {
                    return local_msg_time.format("%H:%M").to_string();
                },
                TimestampFormat::TimeOnlyAmPm => {
                    return local_msg_time.format("%l:%M %p").to_string();
                },
                _ => {}
            }
            let is_full = format == TimestampFormat::Full24 || format == TimestampFormat::Full12;
            let is_24 = format == TimestampFormat::Full24 || format == TimestampFormat::Compact24
                || format == TimestampFormat::TimeOnly24;
            if same_year && day_diff <= 7
            {
                let mut result = String::new();

                if day_diff == 0 {
                    // Same day
                    if is_full {
                        result.push_str(" Today at");
                    }
                } else if day_diff == 1 && is_full {
                    result.push_str(" Yesterday at");
                } else {
                    result.push(' ');
                    result.push_str(local_msg_time.weekday().to_string().as_str());
                    result.push_str(" at");
                }
                // Account for it adding a space before single-digit results
                if is_24 || !is_24 && local_msg_time.hour12().1 > 9 {
                    result.push(' ');
                }

                let format_template: &str = if is_24 {
                    "%H:%M"
                } else {
                    "%l:%M %p"
                };

                result.push_str(local_msg_time.format(format_template).to_string().as_str());
                return result;
            } else {
                // A while ago, so just display date
                let mut result = String::new();
                result.push(' ');
                let format_template: &str = match format {
                    TimestampFormat::Full12 => {
                        "%D at %I:%M %P"
                    },
                    TimestampFormat::Full24 => {
                        "%D at %H:%M"
                    },
                    _ => {
                        "%D"
                    }
                };
                result.push_str(local_msg_time.format(format_template).to_string().as_str());
                return result;
            }
        },
        chrono::LocalResult::Ambiguous(_a, _b) => { return "Amiguous".to_string(); },
        chrono::LocalResult::None => { return "Invalid Time".to_string(); },
    }
}