use druid::kurbo::{Circle, RoundedRect, Rect, BezPath};
use druid::widget::prelude::*;
use druid::{Widget, widget};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid;
use crate::MessageGroup;
use num_traits;
use num_derive;

extern crate chrono;
use chrono::{ Datelike, TimeZone, Timelike};

pub struct TimelineItemWidget {
    msg_content_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
    sender_name_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
    datetime_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
}

const OTHER_MSG_COLOR: Color = Color::rgb8(74, 74, 76);
const SELF_MSG_COLOR: Color = Color::rgb8(12, 131, 242);
const ARROW_SIZE: f64 = 7.0;
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

fn make_tail_path(center_x: f64, y_position: f64, shape: TailShape, flip_x: bool, flip_y: bool) -> kurbo::BezPath {
    let x_translation = if flip_x { -1.0 } else { 1.0 };
    let y_translation = if flip_y { -1.0 } else { 1.0 };
    let mut path = kurbo::BezPath::new();
    // Comments are based on unflipped tail. Unflipped means it's pointing to a pic in the top left.

    if shape == TailShape::Fancy {
        // Bottom right
        path.move_to(Point::new(center_x + ARROW_SIZE * x_translation, y_position + ARROW_SIZE * y_translation));
        // Move towards picture
        path.quad_to(
            Point::new(center_x, y_position - 2.0 * y_translation),
            Point::new(center_x - ARROW_SIZE * x_translation, y_position + -0.2 * y_translation)
        );
        path.quad_to(
            Point::new(center_x - ARROW_SIZE/4.0 * x_translation, y_position + ARROW_SIZE/4.0 * y_translation),
            Point::new(center_x, y_position + ARROW_SIZE * 1.3 * y_translation),
        );
    } else if shape == TailShape::Square {
        // Just make a triangle to remove the radius from this corner
        path.move_to(Point::new(center_x, y_position + -0.1 * y_translation));
        path.line_to(Point::new(center_x, y_position + ARROW_SIZE * 2.0 * y_translation));
        path.line_to(Point::new(center_x + ARROW_SIZE * 2.0 * x_translation, y_position));
    } else {
        // Start top middle. Aligned with top left of bubble if it had no radius
        path.move_to(Point::new(center_x, y_position + -0.1 * y_translation));
        // Flat across the top, towards the picture
        path.line_to(Point::new(center_x - ARROW_SIZE * x_translation, y_position + -0.2 * y_translation));
        // Now to low point. + is down
        match shape {
            TailShape::ConcaveBottom => {
                path.quad_to(
                    Point::new(center_x - ARROW_SIZE/4.0 * x_translation, y_position + ARROW_SIZE/4.0 * y_translation),
                    Point::new(center_x, y_position + ARROW_SIZE * 1.3 * y_translation),
                );
            }
            TailShape::Straight => {
                path.line_to(Point::new(center_x, y_position + ARROW_SIZE * y_translation));
            },
            _ => {
                return BezPath::default();
            }
        }

        // To right to cover the curve of the bubble. Double size to ensure coverage of bubble.
        path.line_to(Point::new(center_x + ARROW_SIZE * 2.0 * x_translation, y_position + 0.2));
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
        let msg_content_label = WidgetPod::new(
            widget::Label::new(|item: &MessageGroup, _env: &_| item.message.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .with_text_size(crate::CONTENT_FONT_SIZE_KEY)
            );
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
                timestamp_to_display_msg(item.timestamp_epoch_seconds,
                    env.get(crate::COMPACT_DATETIME_KEY)).to_string()
        })
            .with_line_break_mode(widget::LineBreaking::WordWrap)
        );
        Self {
            msg_content_label: msg_content_label,
            sender_name_label: sender_name_label,
            datetime_label: datetime_label,
        }
    }

}

impl Widget<MessageGroup> for TimelineItemWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MessageGroup, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(crate::REFRESH_UI_SELECTOR) => {
                ctx.request_layout();
                ctx.request_paint();
            }
            _ => {
                self.msg_content_label.event(ctx, event, data, env);
                self.sender_name_label.event(ctx, event, data, env);
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
        self.msg_content_label.lifecycle(ctx, event, data, env);
        self.sender_name_label.lifecycle(ctx, event, data, env);
        self.datetime_label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &MessageGroup, data: &MessageGroup, env: &Env) {
        self.msg_content_label.update(ctx, data, env);
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
        let is_self_user: bool = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
        let left_bubble_flipped: bool = env.get(crate::LEFT_BUBBLE_FLIPPED_KEY);
        let right_bubble_flipped: bool = env.get(crate::RIGHT_BUBBLE_FLIPPED_KEY);
        let is_bubble_flipped = if is_self_user {
            right_bubble_flipped
        } else {
            left_bubble_flipped
        };
        let item_layout: ItemLayoutOption = num_traits::FromPrimitive::from_u64(env.get(crate::ITEM_LAYOUT_KEY)).expect("Invalid layout index");
        let has_bubble = item_layout == ItemLayoutOption::BubbleExternBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalTopMeta;
        let profile_pic_width: f64 = if !is_self_user || env.get(crate::SHOW_SELF_PROFILE_PIC) || !has_bubble{
            // profile pic is shown always when not self, and when configured when self.
            env.get(crate::IMAGE_SIZE_KEY)
        } else {
            // Not shown, so zero.
            0.0
        };
        let content_bubble_padding: f64 = env.get(crate::BUBBLE_PADDING_KEY);
        let metadata_content_spacing: f64 = env.get(crate::METADATA_CONTENT_SPACING_KEY);
        let profile_pic_bubble_spacing: f64 = env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY);
        let content_left_spacing: f64 = env.get(crate::LEFT_SPACING_KEY);
        let profile_pic_area: f64 = profile_pic_width + profile_pic_bubble_spacing;
        let is_side_by_side = item_layout == ItemLayoutOption::IRCStyle && bc.max().width > IRC_STACK_WIDTH;
        let sender_font_size = env.get(crate::SENDER_FONT_SIZE_KEY);
        let datetime_font_size = env.get(crate::DATETIME_FONT_SIZE_KEY);
        let font_bolded = env.get(crate::HEADER_FONT_BOLDED_KEY);

        let mut font_descriptor = druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
        font_descriptor = font_descriptor.with_weight( if font_bolded { druid::FontWeight::SEMI_BOLD } else { druid::FontWeight::REGULAR });
        self.sender_name_label.widget_mut().set_font(font_descriptor.clone());
        self.datetime_label.widget_mut().set_font(font_descriptor);
        self.sender_name_label.widget_mut().set_text_size(sender_font_size);
        self.datetime_label.widget_mut().set_text_size(datetime_font_size);
        self.sender_name_label.widget_mut().set_text_color(crate::SENDER_COLOR_KEY);
        self.datetime_label.widget_mut().set_text_color(crate::DATETIME_COLOR_KEY);


        // Do the label first since we need to know its size
        let full_width_bounding_box = BoxConstraints::new(
            Size::new(0.0, 0.0),
            Size::new(bc.max().width - profile_pic_area - 2.0 * content_bubble_padding, bc.max().height)
        );
        let msg_content_bounding_box = BoxConstraints::new(
            Size::new(0.0, 0.0),
            if is_side_by_side {
                Size::new(0.0f64.max(full_width_bounding_box.max().width - IRC_HEADER_WIDTH - content_left_spacing), bc.max().height)
            } else {
                Size::new(0.0f64.max(full_width_bounding_box.max().width - content_left_spacing), bc.max().height)
            }
        );
        let sender_bounding_box = if is_side_by_side {
            BoxConstraints::new(
                Size::new(0.0, 0.0),
                Size::new(IRC_HEADER_WIDTH - profile_pic_width, bc.max().height)
            )
        } else {
            full_width_bounding_box.clone()
        };
        // Call layout higher up so we have their sizes.
        let sender_label_size = self.sender_name_label.layout(layout_ctx, &sender_bounding_box, data, env);
        let datetime_label_size = self.datetime_label.layout(layout_ctx, &sender_bounding_box, data, env);
        let msg_label_size = self.msg_content_label.layout(layout_ctx, &msg_content_bounding_box, data, env);
        let total_metadata_width = sender_label_size.width + datetime_label_size.width;

        // Offset in the case of tiny flipped bubbles with tails, since tiny
        // messages cause the tail to not align with the picture properly
        let y_top_offset = if has_bubble && is_bubble_flipped {
            let mut space_taken = 2.0 * content_bubble_padding + msg_label_size.height;
            if item_layout != ItemLayoutOption::BubbleExternBottomMeta {
                space_taken += sender_label_size.height + metadata_content_spacing
            };
            if space_taken >= profile_pic_width {
                0.0
            } else {
                profile_pic_width - space_taken
            }
        } else {
            0.0
        };

        let msg_x_start: f64 = if is_self_user && has_bubble { // Only shift if using a bubble layout
            let mut bubble_content_width = msg_label_size.width;
            if item_layout != ItemLayoutOption::BubbleExternBottomMeta {
                bubble_content_width = bubble_content_width.max(total_metadata_width);
            }
            // Offset so that the profile pic is pushed all the way to the right
            bc.max().width - bubble_content_width - content_bubble_padding * 2.0 - profile_pic_area
        } else {
            // Push to right of profile pic
            profile_pic_area
        };
        let content_x_start = content_left_spacing + msg_x_start;
        let msg_content_origin = if item_layout == ItemLayoutOption::BubbleExternBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalBottomMeta {
            Point::new(content_x_start + content_bubble_padding, content_bubble_padding + y_top_offset)
        } else if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            Point::new(content_x_start + content_bubble_padding, content_bubble_padding + sender_label_size.height + metadata_content_spacing + y_top_offset)
        } else if item_layout == ItemLayoutOption::IRCStyle{
            // Allow having msg and name on same axis if wide enough
            // else stack them
            if is_side_by_side {
                // The msg content is to the right of the metadata
                Point::new(content_left_spacing + IRC_HEADER_WIDTH + content_bubble_padding, y_top_offset)
            } else {
                // Stacked, with no room for picture, since this is the most compact layout
                Point::new(content_left_spacing, metadata_content_spacing + sender_label_size.height + y_top_offset)
            }
        } else {
            // Allow text to move all the way to left if the picture's size
            // is less than the height of the meta label
            if profile_pic_width - 5.0 < sender_label_size.height {
                Point::new(0.0, content_left_spacing + metadata_content_spacing + sender_label_size.height + y_top_offset)
            } else {
                Point::new(content_x_start, metadata_content_spacing + sender_label_size.height + y_top_offset)
            }
        };

        self.msg_content_label.set_origin(layout_ctx, data, env, msg_content_origin);

        let sender_label_origin: Point = if item_layout == ItemLayoutOption::BubbleExternBottomMeta {
            // Outside the bubble, under it.
            // Do not let it cut off the screen to right if it's self user
            let metadata_x_start = if is_self_user && total_metadata_width > msg_label_size.width {
                msg_x_start + msg_label_size.width - total_metadata_width + content_bubble_padding
            } else {
                msg_x_start
            };
            Point::new(metadata_x_start, msg_label_size.height
                + content_bubble_padding * 2.0 + metadata_content_spacing + y_top_offset)
        } else if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            // Inside the bubble, near the top, offset by just the padding
            Point::new(msg_x_start + content_bubble_padding, content_bubble_padding + y_top_offset)
        } else if item_layout == ItemLayoutOption::BubbleInternalBottomMeta {
            // Near the bottom of the bubble, but inside it. Offset by padding.
            Point::new(msg_x_start + content_bubble_padding, 
                msg_label_size.height + content_bubble_padding + metadata_content_spacing + y_top_offset)
        } else {
            // Non-bubble
            Point::new(msg_x_start, 0.0)
        };
        // Position to right of sender. Also account for differences in height.
        let datetime_label_origin = Point::new(sender_label_origin.x + sender_label_size.width,
            sender_label_origin.y + (sender_label_size.height - datetime_label_size.height) * 0.75);
        
        self.sender_name_label.set_origin(layout_ctx, data, env, sender_label_origin);
        self.datetime_label.set_origin(layout_ctx, data, env, datetime_label_origin);

        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        let total_height = if is_side_by_side {
            sender_label_size.height.max(msg_label_size.height) + y_top_offset
        } else {
            y_top_offset + msg_label_size.height + sender_label_size.height + metadata_content_spacing + if has_bubble {
                2.0 * content_bubble_padding
            } else {
                0.0
            }
        };
        Size::new(bc.max().width, total_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MessageGroup, env: &Env) {
        let is_self_user = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
        let left_bubble_flipped: bool = env.get(crate::LEFT_BUBBLE_FLIPPED_KEY);
        let right_bubble_flipped: bool = env.get(crate::RIGHT_BUBBLE_FLIPPED_KEY);
        let is_bubble_flipped = if is_self_user {
            right_bubble_flipped
        } else {
            left_bubble_flipped
        };
        let item_layout: ItemLayoutOption = num_traits::FromPrimitive::from_u64(env.get(crate::ITEM_LAYOUT_KEY)).expect("Invalid layout index");
        let has_bubble = item_layout == ItemLayoutOption::BubbleExternBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalTopMeta;
        let show_self_pic = env.get(crate::SHOW_SELF_PROFILE_PIC);
        let show_left_line = env.get(crate::SHOW_LEFT_LINE_KEY);
        let content_left_line_spacing = env.get(crate::LEFT_SPACING_KEY);
        let bubble_radius = env.get(crate::CHAT_BUBBLE_RADIUS_KEY);
        let content_bubble_padding: f64 = env.get(crate::BUBBLE_PADDING_KEY);
        let metadata_content_spacing: f64 = env.get(crate::METADATA_CONTENT_SPACING_KEY);
        let show_pic = !is_self_user || show_self_pic || !has_bubble;
        let bubble_color = if is_self_user {
            SELF_MSG_COLOR
        } else {
            OTHER_MSG_COLOR
        };

        // For bubbled chats, the styles I've seen are:
        // - Point arrow attached alongside profile pic
        // - Point arrow attached without profile pic
        // - No arrow with profile pic to the side
        // - Profile pic as circle centered alongside border of bubble
        //   (padding needs to be large enough, and profile pic needs to be small enough)
        // There are several arrow styles:
        // - Flat along top/bottom, and straight angled down/up
        // - Flat along top/bottom, but curved angled down/up
        // - Curved on both edges (this is what iMessage uses)

        // First, do the calculations and variables
        let profile_pic_width: f64 = if show_pic {
            // profile pic is shown always when not self, and when configured when self.
            env.get(crate::IMAGE_SIZE_KEY)
        } else {
            // Not shown, so zero.
            0.0
        };
        let total_width = ctx.size().width;
        let profile_pic_spacing= env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY);
        let profile_pic_x_offset = if is_self_user && has_bubble {
            total_width - profile_pic_width
        } else {
            0.0
        };
        let tail_x_center = if is_self_user {
            profile_pic_x_offset - profile_pic_spacing
        } else {
            profile_pic_width + profile_pic_spacing
        };

        let content_label_rect = self.msg_content_label.layout_rect();
        let sender_label_rect = self.sender_name_label.layout_rect();
        let datetime_label_rect = self.datetime_label.layout_rect();
        let bubble_y_origin = if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            self.sender_name_label.layout_rect().y0
        } else {
            self.msg_content_label.layout_rect().y0
        };
        let mut bubble_x0 = content_label_rect.x0 - content_left_line_spacing;
        let mut bubble_x1 = content_label_rect.x1;

        let mut unpadded_bubble_height = content_label_rect.y1 - content_label_rect.y0;
        if item_layout == ItemLayoutOption::BubbleInternalBottomMeta || item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            unpadded_bubble_height += sender_label_rect.height();
            unpadded_bubble_height += metadata_content_spacing;
            bubble_x0 = bubble_x0.min(sender_label_rect.x0).min(datetime_label_rect.x0);
            bubble_x1 = bubble_x1.max(sender_label_rect.x1).max(datetime_label_rect.x1);
        }
        let bubble_y1 = bubble_y_origin + unpadded_bubble_height + content_bubble_padding;
        // Draw background
        if has_bubble {
            let background_rect = RoundedRect::new(bubble_x0 - content_bubble_padding, bubble_y_origin - content_bubble_padding,
                bubble_x1 + content_bubble_padding, bubble_y1, bubble_radius);
            ctx.fill(background_rect, &(bubble_color));
        }
        // Draw hot background (for when user's mouse is hovering over it)
        if ctx.is_hot() {
            ctx.fill(self.msg_content_label.layout_rect(), &Color::rgba8(255, 255, 255, 20));
        }

        // Draw text
        self.msg_content_label.paint(ctx, data, env);
        self.sender_name_label.paint(ctx, data, env);
        self.datetime_label.paint(ctx, data, env);

        // Next, the profile pic
        let piet_image = {
            let image_data = data.profile_pic.clone();
            image_data.to_image(ctx.render_ctx)
        };
        if show_pic {
            ctx.with_save(|ctx| { // Makes it so the clip doesn't mess up the following draws
                let shape_as_int = env.get(crate::IMAGE_SHAPE_KEY);
                let pic_y_offset = if is_bubble_flipped && has_bubble {
                    0.0f64.max(bubble_y1 - profile_pic_width) - 0.3
                } else {
                    0.3 // For preventing some of the profile pic from showing over the tail
                };
                match num_traits::FromPrimitive::from_u64(shape_as_int) {
                    Some(PictureShape::Rectangle) => {},
                    Some(PictureShape::RoundedRectangle) => {
                        ctx.clip(
                            RoundedRect::new(profile_pic_x_offset, 0.0, 
                                profile_pic_x_offset + profile_pic_width, profile_pic_width, 4.0)
                        )
                    },
                    Some(PictureShape::Circle) => {
                        ctx.clip(Circle::new(
                            Point::new(profile_pic_x_offset + profile_pic_width / 2.0, profile_pic_width / 2.0 + pic_y_offset), profile_pic_width / 2.0)
                        )
                    },
                    Some(PictureShape::Hexagon) => {
                        ctx.clip(make_hexagon_path(profile_pic_x_offset, 0.08, 0.25, profile_pic_width))
                    },
                    Some(PictureShape::Octagon) => {
                        ctx.clip(make_octagon_path(profile_pic_x_offset, 0.25, profile_pic_width))
                    },
                    None => eprintln!("Shape int does not translate to known shape, or it is not implemented."),
                }
                ctx.draw_image(&piet_image,
                    druid::Rect::new(profile_pic_x_offset, pic_y_offset,
                        profile_pic_width + profile_pic_x_offset, profile_pic_width + pic_y_offset),
                        druid::piet::InterpolationMode::Bilinear
                );
            });
        }
        // Now the little arrow/tail that goes from the image to the bubble
        if has_bubble {
            let tail_shape_int = env.get(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY);
            let tail_shape: TailShape = num_traits::FromPrimitive::from_u64(tail_shape_int).expect("Invalid tail shape");
            let tail_y_position = if is_bubble_flipped {
                bubble_y1
            } else {
                0.0
            };
            
            if tail_shape != TailShape::Hidden {
                ctx.fill(make_tail_path(
                    tail_x_center,
                    tail_y_position,
                    tail_shape,
                    is_self_user,
                    is_bubble_flipped
                ), &bubble_color);
            }
        }
        // Now draw the line to left of content, if enabled
        if show_left_line {
            let line_x0 = content_label_rect.x0 - content_left_line_spacing;
            let line_rect = Rect::new(line_x0, content_label_rect.y0, line_x0 + 1.0, content_label_rect.y1);
            ctx.fill(line_rect, &Color::GRAY);
        }
    }


}