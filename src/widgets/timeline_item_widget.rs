use druid::kurbo::{Circle, RoundedRect, Rect, BezPath};
use druid::widget::prelude::*;
use druid::{Widget, widget, WidgetExt};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid;
use crate::{MessageGroup, widgets::single_message_widget::SingleMessageWidget};
use crate::LayoutSettings;
use num_derive;

extern crate chrono;
use chrono::{ Datelike, TimeZone, Timelike};

pub struct TimelineItemWidget {
    msg_content_labels: WidgetPod<MessageGroup, Box<dyn Widget<MessageGroup>>>,
    sender_name_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
    datetime_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
}

const OTHER_MSG_COLOR: Color = Color::rgb8(74, 74, 76);
const SELF_MSG_COLOR: Color = Color::rgb8(12, 131, 242);
const IRC_STACK_WIDTH: f64 = 400.0; // How wide should be required for it to no longer be stacked.
const IRC_HEADER_WIDTH: f64 = 160.0; // How far should we push the text right to make it so they don't end up staggered.

#[derive(Clone, Copy, PartialEq, Data, num_derive::FromPrimitive)]
pub enum PictureShape {
    Rectangle = 0,
    RoundedRectangle,
    Circle,
    Hexagon,
    Octagon,
}

#[derive(Clone, Copy, PartialEq, Data, num_derive::FromPrimitive)]
pub enum TailShape {
    Straight = 0,
    ConcaveBottom,
    Fancy,
    Square,
    Symmetric,
    Hidden,
}

#[derive(Clone, Copy, PartialEq, Data, num_derive::FromPrimitive)]
pub enum ItemLayoutOption {
    BubbleExternBottomMeta = 0,
    BubbleInternalBottomMeta,
    BubbleInternalTopMeta,
    Bubbleless,
    IRCStyle,
}

fn make_tail_path(center_x: f64, y_position: f64, shape: TailShape, flip_x: bool, flip_y: bool, tail_size: f64) -> kurbo::BezPath {
    let x_translation = if flip_x { -1.0 } else { 1.0 };
    let y_translation = if flip_y { -1.0 } else { 1.0 };
    let mut path = kurbo::BezPath::new();
    // Comments are based on unflipped tail. Unflipped means it's pointing to a pic in the top left.

    if shape == TailShape::Symmetric
    {
        // Note: It's centered and symmetric, so no need to use y_translation
        path.move_to(Point::new(center_x, y_position - tail_size));
        path.line_to(Point::new(center_x - tail_size * x_translation, y_position));
        path.line_to(Point::new(center_x, y_position + tail_size));
        // Now move over to prevent a gap
        path.line_to(Point::new(center_x + 3.0 * x_translation, y_position + tail_size));
        path.line_to(Point::new(center_x + 3.0 * x_translation, y_position - tail_size));
    } else if shape == TailShape::Fancy {
        // Bottom right
        path.move_to(Point::new(center_x + tail_size * x_translation, y_position + tail_size * y_translation));
        // Move towards picture
        path.quad_to(
            Point::new(center_x, y_position - 2.0 * y_translation),
            Point::new(center_x - tail_size * x_translation, y_position + -0.2 * y_translation)
        );
        path.quad_to(
            Point::new(center_x - tail_size/4.0 * x_translation, y_position + tail_size/4.0 * y_translation),
            Point::new(center_x, y_position + tail_size * 1.3 * y_translation),
        );
    } else if shape == TailShape::Square {
        // Just make a triangle to remove the radius from this corner
        path.move_to(Point::new(center_x, y_position + -0.1 * y_translation));
        path.line_to(Point::new(center_x, y_position + tail_size * 2.0 * y_translation));
        path.line_to(Point::new(center_x + tail_size * 2.0 * x_translation, y_position));
    } else {
        // Start top middle. Aligned with top left of bubble if it had no radius
        path.move_to(Point::new(center_x, y_position + -0.1 * y_translation));
        // Flat across the top, towards the picture
        path.line_to(Point::new(center_x - tail_size * x_translation, y_position + -0.2 * y_translation));
        // Now to low point. + is down
        match shape {
            TailShape::ConcaveBottom => {
                path.quad_to(
                    Point::new(center_x - tail_size/4.0 * x_translation, y_position + tail_size/4.0 * y_translation),
                    Point::new(center_x, y_position + tail_size * 1.3 * y_translation),
                );
            }
            TailShape::Straight => {
                path.line_to(Point::new(center_x, y_position + tail_size * y_translation));
            },
            _ => {
                return BezPath::default();
            }
        }

        // To right to cover the curve of the bubble. Double size to ensure coverage of bubble.
        path.line_to(Point::new(center_x + tail_size * 2.0 * x_translation, y_position + 0.2));
    }
    path.close_path();
    path
}

fn make_hexagon_path(start_x: f64, vertical_trim: f64, inset: f64, pic_width: f64) -> kurbo::BezPath {
    let mut path = kurbo::BezPath::new();
    let second_x = pic_width * inset;
    let third_x = pic_width * (1.0 - inset);
    let top_y = pic_width * vertical_trim;
    let middle_y = pic_width / 2.0;
    let bottom_y = pic_width * (1.0 - vertical_trim);
    path.move_to(Point::new(0.0, middle_y)); // Start
    path.line_to(Point::new( second_x, top_y));
    path.line_to(Point::new( third_x, top_y));
    path.line_to(Point::new( pic_width, middle_y));
    path.line_to(Point::new( third_x, bottom_y));
    path.line_to(Point::new( second_x, bottom_y));
    path.line_to(Point::new(0.0, middle_y));
    path.close_path();
    path.apply_affine(druid::Affine::translate(druid::kurbo::Vec2::new(start_x, 0.0)));
    path
}

fn make_octagon_path(start_x: f64, fraction_from_corner: f64, pic_width: f64) -> kurbo::BezPath {
    let dist_from_corner = pic_width * fraction_from_corner;
    let other_side_pos = pic_width - dist_from_corner;

    let mut path = kurbo::BezPath::new();
    path.move_to(Point::new(0.0, dist_from_corner)); // Start
    path.line_to(Point::new( dist_from_corner, 0.0));
    path.line_to(Point::new( other_side_pos, 0.0));
    path.line_to(Point::new( pic_width, dist_from_corner));
    path.line_to(Point::new( pic_width, other_side_pos));
    path.line_to(Point::new( other_side_pos, pic_width));
    path.line_to(Point::new( dist_from_corner, pic_width));
    path.line_to(Point::new( 0.0, other_side_pos));
    path.close_path();
    path.apply_affine(druid::Affine::translate(druid::kurbo::Vec2::new(start_x, 0.0)));
    path
}

fn timestamp_to_display_msg(epoch: i64, compact: bool) -> String {
    // Helpful reference: https://help.gnome.org/users/gthumb/stable/gthumb-date-formats.html.en
    let now = chrono::offset::Local::now();

    let local_time = chrono::Local.timestamp_opt(epoch, 0);
    match local_time {
        chrono::LocalResult::Single(local_msg_time) => {
            let same_year = now.year() == local_msg_time.year();
            let day_diff = now.ordinal0() as i32 - local_msg_time.ordinal0() as i32;
            if same_year && day_diff <= 7
            {
                let mut result = String::new();

                if day_diff == 0 {
                    // Same day
                    if !compact {
                        result.push_str(" Today at");
                    }
                } else if day_diff == 1 {
                    result.push_str(" Yesterday at");
                } else {
                    result.push(' ');
                    result.push_str(local_msg_time.weekday().to_string().as_str());
                    result.push_str(" at");
                }
                // Account for it adding a space before single-digit results
                if local_msg_time.hour12().1 > 9 {
                    result.push(' ');
                }

                result.push_str(local_msg_time.format("%l:%M %P").to_string().as_str());
                return result;
            } else {
                // A while ago, so just display date
                let mut result = String::new();
                result.push(' ');
                let format: &str;
                if compact {
                    format = "%D";
                } else {
                    format = "%D at %I:%M %P";
                }
                result.push_str(local_msg_time.format(format).to_string().as_str());
                return result;
            }
        },
        chrono::LocalResult::Ambiguous(_a, _b) => { return "Amiguous".to_string(); },
        chrono::LocalResult::None => { return "Invalid Time".to_string(); },
    }

}

impl TimelineItemWidget {
    pub fn new() -> Self {
        let sender_name_label = WidgetPod::new(
            widget::Label::new(|item: &MessageGroup, _env: &_| {
                let mut username = "User".to_string();
                username.push_str(item.user_id.to_string().as_str());
                username
        })
            .with_line_break_mode(widget::LineBreaking::WordWrap)
        );
        let datetime_label = WidgetPod::new(
            widget::Label::new(|item: &MessageGroup, env: &Env| {
                if item.messages.len() > 0 {
                    timestamp_to_display_msg(item.messages[0].timestamp_epoch_seconds,
                        env.get(crate::COMPACT_DATETIME_KEY)).to_string()
                } else {
                    "Invalid".to_string()
                }
            }
        )
            .with_line_break_mode(widget::LineBreaking::WordWrap)
        );
        let msg_content_labels_list = widget::List::new(|| {
            SingleMessageWidget::new()
        }).with_spacing(crate::SINGLE_MESSAGE_SPACING_KEY);
        let msg_content_labels = WidgetPod::new(
            // Boxed is needed to make it so you don't get buried in type annotations.
            msg_content_labels_list.lens(MessageGroup::messages).boxed()
        );
        Self {
            msg_content_labels: msg_content_labels,
            sender_name_label: sender_name_label,
            datetime_label: datetime_label,
        }
    }

}

impl LayoutSettings {
    
    /// Gets the font for the title
    /// 
    /// It is semi-bolded when the settings specify that it should be.
    fn get_metadata_font_descriptor(&self) -> druid::FontDescriptor {
        druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI)
            .with_weight(
                if self.metadata_font_bolded {
                    druid::FontWeight::SEMI_BOLD
                } else {
                    druid::FontWeight::REGULAR
                }
            )
    }

    /// It returns true when the layout specifies that it is a bubble
    /// A bubble has padding on all sides of the content, and a background is drawn behind it.
    fn is_bubble(&self) -> bool {
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta | ItemLayoutOption::BubbleInternalBottomMeta
                | ItemLayoutOption::BubbleInternalTopMeta => {
                true
            },
            _ => false
        }
    }

    /// A bubble is flipped when the tail and profile picture is on the
    /// bottom instead of the top.
    fn is_bubble_flipped(&self, is_self_user: bool) -> bool {
        if is_self_user {
            self.right_bubble_flipped
        } else {
            self.left_bubble_flipped
        }
    }

    /// Gets the proper width of a profile picture
    /// If there is no profile picture given the current situation or setting, it returns 0.0
    fn actual_profile_pic_width(&self, is_self_user: bool) -> f64 {
        if self.show_picture(is_self_user) {
            // profile pic is shown always when not self, and when configured when self.
            self.picture_size
        } else {
            // Not shown, so zero.
            0.0
        }
    }

    /// Offset in the case of tiny flipped bubbles with tails, since tiny
    /// messages cause the tail to not align with the picture properly
    fn get_top_y_offset(&self, is_self_user: bool, sender_label_size: &Size, msg_label_size: &Size) -> f64 {
        if self.is_bubble() && self.is_bubble_flipped(is_self_user) {
            let mut space_taken = 2.0 * self.bubble_padding + msg_label_size.height;
            if self.item_layout != ItemLayoutOption::BubbleExternBottomMeta {
                space_taken += sender_label_size.height + self.metadata_content_spacing
            };
            let profile_pic_width = self.actual_profile_pic_width(is_self_user);
            if space_taken >= profile_pic_width {
                0.0
            } else {
                profile_pic_width - space_taken
            }
        } else {
            0.0
        }
    }

    /// Gets the width of the profile pic and the space between it and the message or bubble.
    fn get_profile_pic_area_width(&self, is_self_user: bool) -> f64 {
        self.actual_profile_pic_width(is_self_user) + self.chat_bubble_picture_spacing
    }

    /// Returns true when the content will not be vertically stacked, but instead horizontally aligned.
    fn is_side_by_side(&self, space_available: &BoxConstraints) -> bool {
        self.item_layout == ItemLayoutOption::IRCStyle && space_available.max().width > IRC_STACK_WIDTH
    }

    /// The area that the content can take up.
    /// 
    /// Under most layouts, that's the total width minus the space taken up by
    /// the profile pic and the space between it and the content.
    /// 
    /// In side by side, it's the total width minus the width of the IRC header.
    fn get_available_content_width(&self, space_available: &BoxConstraints, is_self_user: bool) -> f64 {
        let mut width: f64 = space_available.max().width - self.left_spacing;
        width -= if self.is_side_by_side(space_available) {
            IRC_HEADER_WIDTH
        } else {
            self.get_profile_pic_area_width(is_self_user)
        };
        if self.is_bubble() {
            width -= self.bubble_padding * 2.0;
        }
        width
    }

    /// Returns the available bounding area for the content.
    /// 
    /// The min is set to zero space, and the max is the max height and
    /// the width is the width provided by [get_available_content_width()]
    fn get_available_content_area(&self, space_available: &BoxConstraints, is_self_user: bool) -> BoxConstraints {
        to_full_height_area(self.get_available_content_width(space_available, is_self_user), space_available)
    }

    /// Returns the available bounding area for the content.
    /// 
    /// The min is set to zero space, and the max is the max height and
    /// the width is either the total width minus the left spacing, or the IRC width minus the picture size
    fn get_sender_label_area(&self, space_available: &BoxConstraints) -> BoxConstraints {
        let width = if self.is_side_by_side(space_available) {
            IRC_HEADER_WIDTH - self.picture_size
        } else {
            space_available.max().width - self.left_spacing
        };
        to_full_height_area(width, space_available)
    }

    /// Gets the unpadded content x left position
    /// This is how far right the content needs to move right
    /// Depending on the layout and content, it can be left or right aligned.
    /// 
    /// If left aligned, it is just pushed to the right of the profile pic and its padding.
    /// If right aligned, it subtracts the content size from the available space.
    fn get_unpadded_content_x_left_position(&self, is_self_user: bool, space_available: &BoxConstraints,
        actual_max_content_width: f64, total_metadata_width: f64) -> f64
    {
        if is_self_user && self.is_bubble() { // Only shift if using a bubble layout
            let required_width = if self.item_layout != ItemLayoutOption::BubbleExternBottomMeta
                && total_metadata_width > actual_max_content_width
            {
                // For ExternBottomMeta, the content is on the outside, so the bubble doesn't affect the size
                total_metadata_width
            } else {
                actual_max_content_width
            };
            // Offset so that the profile pic is pushed all the way to the right
            space_available.max().width - required_width
                - self.bubble_padding * 2.0 - self.get_profile_pic_area_width(is_self_user)
        } else {
            // Push to right of profile pic
            self.get_profile_pic_area_width(is_self_user)
        }
    }

    /// Gets the origin position for the content, taking into account things including padding,
    /// layout, and the size of other items.
    fn get_content_origin(&self, is_self_user: bool, space_available: &BoxConstraints, y_top_offset: f64,
        widest_msg_content: f64, total_metadata_width: f64, metadata_height: f64) -> Point
    {
        let content_x_start = self.get_unpadded_content_x_left_position(
            is_self_user, space_available, widest_msg_content, total_metadata_width
        ) + self.left_spacing;
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta | ItemLayoutOption::BubbleInternalBottomMeta => {
                Point::new(
                    content_x_start + self.bubble_padding,
                    y_top_offset + self.bubble_padding
                )
            },
            ItemLayoutOption::BubbleInternalTopMeta => {
                Point::new(
                    // Align to left inside of bubble
                    content_x_start + self.bubble_padding,
                    // Near the top, below metadata and padding
                    y_top_offset + self.bubble_padding * 2.0 + metadata_height
                )
            },
            ItemLayoutOption::IRCStyle => {
                // Allow having msg and name on same axis if wide enough
                // else stack them
                if self.is_side_by_side(space_available) {
                    // The msg content is to the right of the metadata
                    Point::new(self.left_spacing + IRC_HEADER_WIDTH, y_top_offset)
                } else {
                    // Stacked, with picture above instead of to side, since this is the most compact layout
                    Point::new(self.left_spacing, metadata_height + self.metadata_content_spacing + y_top_offset)
                }
            },
            _ => {
                // Allow text to move all the way to left if the picture's size
                // is less than the height of the meta label
                Point::new(
                    if self.align_to_picture {
                        content_x_start // Nothing special. Just aligned to context x start.
                    } else {
                        0.0 // All the way to the left
                    },
                    // Just below the metadata
                    metadata_height + self.metadata_content_spacing + y_top_offset
                )
            }
        }
    }

    /// Gets the origin position for the sender, taking into account things including padding,
    /// layout, and the size of other items.
    fn get_sender_origin(&self, is_self_user: bool, space_available: &BoxConstraints,
        total_metadata_width: f64, total_msg_height: f64, widest_msg_width: f64, y_top_offset: f64) -> Point
    {
        let msg_x_start = self.get_unpadded_content_x_left_position(is_self_user, space_available, 
            widest_msg_width, total_metadata_width);
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta => {
                // Outside the bubble, under it.
                // Do not let it cut off the screen to right if it's self user
                let metadata_x_start = if is_self_user && total_metadata_width > widest_msg_width {
                    msg_x_start + widest_msg_width - total_metadata_width + self.bubble_padding
                } else {
                    msg_x_start
                };
                Point::new(metadata_x_start, total_msg_height
                    + self.bubble_padding * 2.0 + self.metadata_content_spacing + y_top_offset)
            },
            ItemLayoutOption::BubbleInternalTopMeta => {
                // Inside the bubble, near the top, offset by just the padding
                Point::new(msg_x_start + self.bubble_padding, self.bubble_padding + y_top_offset)
            },
            ItemLayoutOption::BubbleInternalBottomMeta => {
                // Near the bottom of the bubble, but inside it. Offset by padding.
                Point::new(msg_x_start + self.bubble_padding,
                    total_msg_height + self.bubble_padding + self.metadata_content_spacing + y_top_offset)
            },
            _ => {
                // Non-bubble
                Point::new(msg_x_start, 0.0)
            }
        }
    }

    /// Gets the total height of a timeline item widget.
    /// Accounts for everything, including layout, content sizes, other label sizes, and padding.
    fn get_total_height(&self, space_available: &BoxConstraints, sender_label_size: &Size,
        msg_label_size: &Size, y_top_offset: f64) -> f64
    {
        if self.is_side_by_side(space_available) {
            sender_label_size.height.max(msg_label_size.height) + y_top_offset
        } else {
            y_top_offset + msg_label_size.height + sender_label_size.height + self.metadata_content_spacing + if self.is_bubble() {
                2.0 * self.bubble_padding // total height of padding (both top and bottom).
            } else {
                0.0 // Not a bubble, so no padding.
            }
        }
    }

    fn show_picture(&self, is_self_user: bool) -> bool {
        !is_self_user || self.show_self_pic || !self.is_bubble()
    }

    /// Used to position the profile pic
    fn profile_pic_x_offset(&self, is_self_user: bool, width_available: f64) -> f64 {
        if is_self_user && self.is_bubble() {
            width_available - self.picture_size
        } else {
            0.0
        }
    }

    /// Used to account for cases when the tail will not
    /// just be pinned to the top or bottom
    fn get_tail_y_offset(&self, is_self_user: bool) -> f64 {
        if self.chat_bubble_tail_shape == TailShape::Symmetric {
            // It's symmetric and centered at the profile picture
            let mut offset = self.picture_size / 2.0;
            // If the bubble is flipped, the reference point changes to the
            // bottom, so it needs to be negative.
            if self.is_bubble_flipped(is_self_user) {
                offset *= -1.0;
            }
            offset
        } else {
            0.0
        }
    }
}

/// A convenience function to convert the width to a BoxConstraints that has the input width
/// as the max width, and the maximum height.
/// The minimum height and width are set to zero. 
fn to_full_height_area(width: f64, space_available: &BoxConstraints) -> BoxConstraints {
    BoxConstraints::new(
        Size::new(0.0, 0.0),
        Size::new(width, space_available.max().height)
    )
}

impl Widget<MessageGroup> for TimelineItemWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MessageGroup, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(crate::REFRESH_UI_SELECTOR) => {
                ctx.request_layout();
                ctx.request_paint();
            }
            _ => {
                self.msg_content_labels.event(ctx, event, data, env);
                self.sender_name_label.event(ctx, event, data, env);
                self.datetime_label.event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &MessageGroup,
        env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            },
            _ => {}
        }
        self.msg_content_labels.lifecycle(ctx, event, data, env);
        self.sender_name_label.lifecycle(ctx, event, data, env);
        self.datetime_label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &MessageGroup, data: &MessageGroup, env: &Env) {
        self.msg_content_labels.update(ctx, data, env);
        self.sender_name_label.update(ctx, data, env);
        self.datetime_label.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &MessageGroup,
        env: &Env,
    ) -> Size {
        let settings = LayoutSettings::from_env(env);
        let is_self_user: bool = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;

        self.sender_name_label.widget_mut().set_font(settings.get_metadata_font_descriptor());
        self.datetime_label.widget_mut().set_font(settings.get_metadata_font_descriptor());
        self.sender_name_label.widget_mut().set_text_size(crate::SENDER_FONT_SIZE_KEY);
        self.datetime_label.widget_mut().set_text_size(crate::DATETIME_FONT_SIZE_KEY);
        self.sender_name_label.widget_mut().set_text_color(crate::SENDER_COLOR_KEY);
        self.datetime_label.widget_mut().set_text_color(crate::DATETIME_COLOR_KEY);


        // Do the label layouts first since we need to know their sizes
        let sender_label_size = self.sender_name_label.layout(
            layout_ctx,
            &settings.get_sender_label_area(bc),
            data, env
        );
        let datetime_label_size = self.datetime_label.layout(
            layout_ctx, &settings.get_sender_label_area(bc),
            data, env
        );

        let msg_label_list_size = self.msg_content_labels.layout(
            layout_ctx, &settings.get_available_content_area(bc, is_self_user),
            data, env);
        let total_metadata_width = sender_label_size.width + datetime_label_size.width;

        // Offset in the case of tiny flipped bubbles with tails, since tiny
        // messages cause the tail to not align with the picture properly
        let y_top_offset = settings.get_top_y_offset(is_self_user, &sender_label_size, &msg_label_list_size);

        self.msg_content_labels.set_origin(layout_ctx, data, env,
            settings.get_content_origin(
                is_self_user,
                bc,
                y_top_offset,
                msg_label_list_size.width,
                total_metadata_width,
                sender_label_size.height
            )
        );

        let sender_label_origin = settings.get_sender_origin(is_self_user, bc, total_metadata_width, 
            msg_label_list_size.height, msg_label_list_size.width, y_top_offset);

        // Position to right of sender. Also account for differences in height.
        let datetime_label_origin = Point::new(sender_label_origin.x + sender_label_size.width,
            sender_label_origin.y + (sender_label_size.height - datetime_label_size.height) * 0.75);
        
        self.sender_name_label.set_origin(layout_ctx, data, env, sender_label_origin);
        self.datetime_label.set_origin(layout_ctx, data, env, datetime_label_origin);

        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        Size::new(bc.max().width, settings.get_total_height(bc, &sender_label_size, &msg_label_list_size, y_top_offset))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MessageGroup, env: &Env) {
        let settings = LayoutSettings::from_env(env);
        let is_self_user = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
    
        // First, do the calculations and variables
        self.draw_bubble_background(ctx, &settings, is_self_user);
        // Draw hot background (for when user's mouse is hovering over it)
        if ctx.is_hot() {
            ctx.fill(self.msg_content_labels.layout_rect(), &Color::rgba8(255, 255, 255, 20));
        }

        // Draw text
        self.msg_content_labels.paint(ctx, data, env);
        self.sender_name_label.paint(ctx, data, env);
        self.datetime_label.paint(ctx, data, env);

        // Next, the profile pic
        self.draw_profile_pic(ctx, data, &settings, is_self_user);
        // Now the little arrow/tail that goes from the image to the bubble
        self.draw_bubble_tail(ctx, &settings, is_self_user);
        // Now draw the line to left of content, if enabled
        self.draw_left_line(ctx, &settings);
    }

}

impl TimelineItemWidget {

    /// Gets the total space taken up by all labels in the bubble, minus the padding. 
    /// Return order: x0, x1, y0, y1
    fn get_bubble_dimensions(&self, settings: &LayoutSettings) -> (f64, f64, f64, f64) {
        let content_label_rect = self.msg_content_labels.layout_rect();
        let sender_label_rect = self.sender_name_label.layout_rect();
        let datetime_label_rect = self.datetime_label.layout_rect();
        let mut bubble_x0 = content_label_rect.x0 - settings.left_spacing;
        let mut bubble_x1 = content_label_rect.x1;

        let mut unpadded_bubble_height = content_label_rect.y1 - content_label_rect.y0;
        if settings.item_layout == ItemLayoutOption::BubbleInternalBottomMeta || settings.item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            unpadded_bubble_height += sender_label_rect.height();
            unpadded_bubble_height += settings.metadata_content_spacing;
            bubble_x0 = bubble_x0.min(sender_label_rect.x0).min(datetime_label_rect.x0);
            bubble_x1 = bubble_x1.max(sender_label_rect.x1).max(datetime_label_rect.x1);
        }
        let bubble_y0 = if settings.item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            sender_label_rect.y0
        } else {
            content_label_rect.y0
        };

        let bubble_y1 = bubble_y0 + unpadded_bubble_height;

        (bubble_x0 - settings.bubble_padding, bubble_x1 + settings.bubble_padding,
            bubble_y0 - settings.bubble_padding, bubble_y1 + settings.bubble_padding)
    }

    fn get_bubble_color(&self, is_self_user: bool) -> druid::Color {
        if is_self_user {
            SELF_MSG_COLOR
        } else {
            OTHER_MSG_COLOR
        }
    }

    fn draw_bubble_background(&self, ctx: &mut PaintCtx, settings: &LayoutSettings, is_self_user: bool) {
        let (bubble_x0, bubble_x1, bubble_y0, bubble_y1) = self.get_bubble_dimensions(settings);

        let bubble_color = self.get_bubble_color(is_self_user);
        // Draw background
        if settings.is_bubble() {
            let background_rect = RoundedRect::new(
                bubble_x0, bubble_y0, bubble_x1, bubble_y1, settings.chat_bubble_radius
            );
            ctx.fill(background_rect, &(bubble_color));
        }
    }

    fn draw_bubble_tail(&self, ctx: &mut PaintCtx, settings: &LayoutSettings, is_self_user: bool) {
        if settings.is_bubble() {
            let (bubble_x0, bubble_x1, bubble_y0, bubble_y1) = self.get_bubble_dimensions(settings);

            let is_flipped = settings.is_bubble_flipped(is_self_user);
            let tail_y_position = if is_flipped { bubble_y1 } else { bubble_y0 };
            let tail_x_position = if is_self_user { bubble_x1 } else { bubble_x0 };
            let bubble_color = self.get_bubble_color(is_self_user);
            
            if settings.chat_bubble_tail_shape != TailShape::Hidden {
                ctx.fill(make_tail_path(
                    tail_x_position,
                    tail_y_position + settings.get_tail_y_offset(is_self_user),
                    settings.chat_bubble_tail_shape,
                    is_self_user,
                    is_flipped,
                    settings.chat_bubble_tail_size,
                ), &bubble_color);
            }
        }
    }

    fn draw_profile_pic(&self, ctx: &mut PaintCtx, data: &MessageGroup, settings: &LayoutSettings, is_self_user: bool) {
        if !settings.show_picture(is_self_user) {
            return;
        }
        let profile_pic_x_offset = settings.profile_pic_x_offset(is_self_user, ctx.region().bounding_box().width());
        let piet_image = {
            let image_data = data.profile_pic.clone();
            image_data.to_image(ctx.render_ctx)
        };
        ctx.with_save(|ctx| { // Makes it so the clip doesn't mess up the following draws
            let pic_y_offset = if settings.is_bubble_flipped(is_self_user) && settings.is_bubble() {
                let (_, _, _, bubble_y1) = self.get_bubble_dimensions(settings);

                0.0f64.max(bubble_y1 - settings.picture_size) - 0.3
            } else {
                0.3 // For preventing some of the profile pic from showing over the tail
            };
            match settings.picture_shape {
                PictureShape::Rectangle => {},
                PictureShape::RoundedRectangle => {
                    ctx.clip(
                        RoundedRect::new(profile_pic_x_offset, 0.0, 
                            profile_pic_x_offset + settings.picture_size, settings.picture_size, 4.0)
                    )
                },
                PictureShape::Circle => {
                    ctx.clip(Circle::new(
                        Point::new(
                            profile_pic_x_offset + settings.picture_size / 2.0,
                            settings.picture_size / 2.0 + pic_y_offset), settings.picture_size / 2.0
                        )
                    )
                },
                PictureShape::Hexagon => {
                    ctx.clip(make_hexagon_path(profile_pic_x_offset, 0.08, 0.25, settings.picture_size))
                },
                PictureShape::Octagon => {
                    ctx.clip(make_octagon_path(profile_pic_x_offset, 0.25, settings.picture_size))
                },
            }
            ctx.draw_image(&piet_image,
                druid::Rect::new(profile_pic_x_offset, pic_y_offset,
                    settings.picture_size + profile_pic_x_offset, settings.picture_size + pic_y_offset),
                    druid::piet::InterpolationMode::Bilinear
            );
        });
    }

    fn draw_left_line(&self, ctx: &mut PaintCtx, settings: &LayoutSettings) {
        if settings.show_left_line {
            let content_label_rect = self.msg_content_labels.layout_rect();
            let line_x0 = content_label_rect.x0 - settings.left_spacing;
            let line_rect = Rect::new(line_x0, content_label_rect.y0, line_x0 + 1.0, content_label_rect.y1);
            ctx.fill(line_rect, &Color::GRAY);
        }
    }
}