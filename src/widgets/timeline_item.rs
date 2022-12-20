use druid::kurbo::{Circle, RoundedRect, BezPath};
use druid::widget::prelude::*;
use druid::{Widget, widget};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid;
use crate::Message;
use num_traits;
use num_derive;

extern crate chrono;
use chrono::{ Datelike, TimeZone, Timelike};

pub struct TimelineItemWidget {
    msg_content_label: WidgetPod<Message, widget::Label<Message>>,
    sender_name_label: WidgetPod<Message, widget::Label<Message>>,
}

const OTHER_MSG_COLOR: Color = Color::rgb8(74, 74, 76);
const SELF_MSG_COLOR: Color = Color::rgb8(12, 131, 242);
const SUB_TEXT_COLOR: Color = Color::rgb8(175, 175, 175);
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

fn make_tail_path(center_x: f64, shape: TailShape, flip_x: bool) -> kurbo::BezPath {
    let x_translation = if flip_x { -1.0 } else { 1.0 };
    let mut path = kurbo::BezPath::new();
    path.move_to(Point::new(center_x, -0.1)); // Start
    path.line_to(Point::new(center_x - ARROW_SIZE * x_translation, -0.2)); // towards picture
    // Now to low point. + is down
    match shape {
        TailShape::ConcaveBottom => {
            path.quad_to(
                Point::new(center_x - ARROW_SIZE/4.0 * x_translation, ARROW_SIZE/4.0),
                Point::new(center_x, ARROW_SIZE * 1.3),
            );
        }
        TailShape::Straight => {
            path.line_to(Point::new(center_x, ARROW_SIZE));
        },
        TailShape::Hidden => {
            return BezPath::default();
        }
    }

    // To right to cover the curve of the bubble. Double size to ensure coverage of bubble.
    path.line_to(Point::new(center_x + ARROW_SIZE * 2.0 * x_translation, 0.2));
    path.line_to(Point::new(center_x, -0.1));
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
            widget::Label::new(|item: &Message, _env: &_| item.message.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .with_text_size(13.0));
        let sender_name_label = WidgetPod::new(
            widget::Label::new(|item: &Message, _env: &_| {
                let mut username = "User".to_string();
                username.push_str(item.user_id.to_string().as_str());
                username.push_str(" •");
                username.push_str(timestamp_to_display_msg(item.timestamp_epoch_seconds, true).as_str());
                username
        })
            .with_line_break_mode(widget::LineBreaking::WordWrap)
        );
        Self {
            msg_content_label: msg_content_label,
            sender_name_label: sender_name_label,
        }
    }

}

impl Widget<Message> for TimelineItemWidget {

    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Message, _env: &Env) {
        self.msg_content_label.event(_ctx, _event, _data, _env);
        self.sender_name_label.event(_ctx, _event, _data, _env);
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Message,
        _env: &Env,
    ) {
        self.msg_content_label.lifecycle(_ctx, _event, _data, _env);
        self.sender_name_label.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Message, _data: &Message, _env: &Env) {
        self.msg_content_label.update(_ctx, _data, _env);
        self.sender_name_label.update(_ctx, _data, _env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Message,
        env: &Env,
    ) -> Size {
        let is_self_user: bool = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
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
        let msg_padding: f64 = env.get(crate::MSG_PADDING_KEY);
        let profile_pic_bubble_spacing: f64 = env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY);
        let profile_pic_area: f64 = profile_pic_width + profile_pic_bubble_spacing;
        let is_side_by_side = item_layout == ItemLayoutOption::IRCStyle && bc.max().width > IRC_STACK_WIDTH;

        // Ensure proper font size is used
        let has_bottom_metadata = item_layout == ItemLayoutOption::BubbleExternBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalBottomMeta;

        let mut font_descriptor = druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
        if has_bottom_metadata {
            font_descriptor = font_descriptor.with_weight(druid::FontWeight::REGULAR);
            font_descriptor = font_descriptor.with_size(11.0);
            self.sender_name_label.widget_mut().set_text_color(SUB_TEXT_COLOR);
        } else {
            font_descriptor = font_descriptor.with_weight(druid::FontWeight::SEMI_BOLD);
            font_descriptor = font_descriptor.with_size(13.0);
            self.sender_name_label.widget_mut().set_text_color(Color::WHITE);
        }
        self.sender_name_label.widget_mut().set_font(font_descriptor);

        // Do the label first since we need to know its size
        let full_width_bounding_box = BoxConstraints::new(
            Size::new(0.0, 0.0),
            Size::new(bc.max().width - profile_pic_area - 2.0 * msg_padding, bc.max().height)
        );
        let msg_content_bounding_box = if is_side_by_side {
            BoxConstraints::new(
                Size::new(0.0, 0.0),
                Size::new(0.0f64.max(bc.max().width - IRC_HEADER_WIDTH), bc.max().height)
            )
        } else {
            full_width_bounding_box.clone()
        };
        let sender_bounding_box = if is_side_by_side {
            BoxConstraints::new(
                Size::new(0.0, 0.0),
                Size::new(IRC_HEADER_WIDTH - profile_pic_width, bc.max().height)
            )
        } else {
            full_width_bounding_box.clone()
        };
        // Call layout higher up so we have its size.
        let sender_label_size = self.sender_name_label.layout(layout_ctx, &sender_bounding_box, data, env);

        let msg_label_size = self.msg_content_label.layout(layout_ctx, &msg_content_bounding_box, data, env);
        let msg_x_start: f64 = if is_self_user && has_bubble { // Only shift if using a bubble layout
            // Offset so that the profile pic is pushed all the way to the right
            bc.max().width - msg_label_size.width - msg_padding * 2.0 - profile_pic_area
        } else {
            // Push to right of profile pic
            profile_pic_area
        };
        let msg_content_origin = if has_bottom_metadata {
            Point::new(msg_x_start + msg_padding, msg_padding)
        } else if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            Point::new(msg_x_start + msg_padding, msg_padding + sender_label_size.height)
        } else if item_layout == ItemLayoutOption::IRCStyle{
            // Allow having msg and name on same axis if wide enough
            // else stack them
            if is_side_by_side {
                // The msg content is to the right of the metadata
                Point::new(IRC_HEADER_WIDTH + msg_padding, 0.0)
            } else {
                // Stacked, with no room for picture, since this is the most compact layout
                Point::new(0.0, msg_padding + sender_label_size.height)
            }
        } else {
            // Allow text to move all the way to left if the picture's size
            // is less than the height of the meta label
            if profile_pic_width - 5.0 < sender_label_size.height {
                Point::new(0.0, msg_padding + sender_label_size.height)
            } else {
                Point::new(msg_x_start, msg_padding + sender_label_size.height)
            }
        };

        self.msg_content_label.set_origin(layout_ctx, data, env, msg_content_origin);

        let sender_label_origin: Point = if item_layout == ItemLayoutOption::BubbleExternBottomMeta {
            // Outside the bubble, under it.
            Point::new(msg_x_start, msg_label_size.height + msg_padding * 2.0)
        } else if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
            // Inside the bubble, near the top, offset by just the padding
            Point::new(msg_x_start + msg_padding, msg_padding )
        } else if item_layout == ItemLayoutOption::BubbleInternalBottomMeta {
            // Near the bottom of the bubble, but inside it. Offset by padding.
            Point::new(msg_x_start + msg_padding, msg_label_size.height + msg_padding * 1.3)
        } else {
            // Non-bubble
            Point::new(msg_x_start, 0.0)
        };
        
        self.sender_name_label.set_origin(layout_ctx, data, env, sender_label_origin);

        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        if is_side_by_side {
            Size::new(bc.max().width, sender_label_size.height.max(msg_label_size.height))
        } else {
            Size::new(bc.max().width, msg_label_size.height + sender_label_size.height + msg_padding * 2.0)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Message, env: &Env) {
        let is_self_user = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
        let item_layout: ItemLayoutOption = num_traits::FromPrimitive::from_u64(env.get(crate::ITEM_LAYOUT_KEY)).expect("Invalid layout index");
        let has_bubble = item_layout == ItemLayoutOption::BubbleExternBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalBottomMeta
            || item_layout == ItemLayoutOption::BubbleInternalTopMeta;
        let show_self_pic = env.get(crate::SHOW_SELF_PROFILE_PIC);
        let bubble_radius = env.get(crate::CHAT_BUBBLE_RADIUS_KEY);
        let msg_padding: f64 = env.get(crate::MSG_PADDING_KEY);
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

        // Draw background
        if has_bubble {
            let content_label_rect = self.msg_content_label.layout_rect();
            let bubble_y_origin = if item_layout == ItemLayoutOption::BubbleInternalTopMeta {
                self.sender_name_label.layout_rect().y0
            } else {
                self.msg_content_label.layout_rect().y0
            };
            let mut bubble_x1 = content_label_rect.x1;

            let mut bubble_height = content_label_rect.y1 + msg_padding - content_label_rect.y0;
            if item_layout == ItemLayoutOption::BubbleInternalBottomMeta || item_layout == ItemLayoutOption::BubbleInternalTopMeta {
                bubble_height += self.sender_name_label.layout_rect().height();
                bubble_x1 = bubble_x1.max(self.sender_name_label.layout_rect().x1);
            }
            let background_rect = RoundedRect::new(content_label_rect.x0 - msg_padding, bubble_y_origin - msg_padding,
                bubble_x1 + msg_padding, bubble_y_origin + bubble_height, bubble_radius);
            ctx.fill(background_rect, &(bubble_color));
        }

        // Draw text
        self.msg_content_label.paint(ctx, data, env);
        self.sender_name_label.paint(ctx, data, env);

        // Next, the profile pic
        let piet_image = {
            let image_data = data.profile_pic.clone();
            image_data.to_image(ctx.render_ctx)
        };
        if show_pic {
            ctx.with_save(|ctx| { // Makes it so the clip doesn't mess up the following draws
                let shape_as_int = env.get(crate::IMAGE_SHAPE_KEY);
                let pic_y_offset = 0.3; // For preventing some of the profile pic from showing over the tail
                match num_traits::FromPrimitive::from_u64(shape_as_int) {
                    Some(PictureShape::Rectangle) => {},
                    Some(PictureShape::RoundedRectangle) => {
                        ctx.clip(
                            RoundedRect::new(profile_pic_x_offset, 0.0, 
                                profile_pic_x_offset + profile_pic_width, profile_pic_width, 6.0)
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
        if has_bubble {
            let tail_shape_int = env.get(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY);
            let tail_shape = num_traits::FromPrimitive::from_u64(tail_shape_int).expect("Invalid tail shape");
            // Now the little arrow that goes from the image to the bubble
            if tail_shape != TailShape::Hidden {
                ctx.fill(make_tail_path(
                    tail_x_center,
                    tail_shape,
                    is_self_user
                ), &bubble_color);
            }
        }
    }


}