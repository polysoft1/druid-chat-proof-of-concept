use druid::kurbo::{Circle, RoundedRect, BezPath};
use druid::widget::prelude::*;
use druid::{Widget, widget, WidgetExt};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid;
use crate::{MessageGroup, widgets::single_message_widget::SingleMessageWidget};
use crate::LayoutSettings;
use crate::helper::helper_functions;
use num_derive;

extern crate chrono;

pub struct TimelineItemWidget {
    msg_content_labels: WidgetPod<MessageGroup, Box<dyn Widget<MessageGroup>>>,
    sender_name_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
    datetime_label: WidgetPod<MessageGroup, widget::Label<MessageGroup>>,
}

const OTHER_MSG_COLOR: Color = Color::rgb8(74, 74, 76);
const SELF_MSG_COLOR: Color = Color::rgb8(12, 131, 242);
const DOT_SIZE: f64 = 1.5;
const DOT_X_OFFSET: f64 = -0.9;
const DOT_Y_OFFSET: f64 = 1.0;

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

#[derive(Clone, Copy, PartialEq, Data, num_derive::FromPrimitive)]
pub enum MetadataLayout {
    LeftSideBySide,
    LeftSideBySideWithDot,
    LeftRightSpaced,
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
                    helper_functions::timestamp_to_display_msg(
                        item.messages[0].timestamp_epoch_seconds,
                        num_traits::FromPrimitive::from_u64(env.get(crate::DATETIME_FORMAT_KEY)).expect("Invalid datetime format index"),
                    ).to_string()
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

impl Widget<MessageGroup> for TimelineItemWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MessageGroup, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(crate::REFRESH_UI_SELECTOR) => {
                self.msg_content_labels.event(ctx, event, data, env);
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
        self.sender_name_label.widget_mut().set_text_color(settings.get_sender_color(is_self_user));
        self.datetime_label.widget_mut().set_text_color(settings.get_datetime_color(is_self_user));

        let width_available = bc.max().width;


        // Do the label layouts first since we need to know their sizes
        let sender_label_size = self.sender_name_label.layout(
            layout_ctx,
            &settings.get_sender_label_area(width_available),
            data, env
        );
        let datetime_label_size = self.datetime_label.layout(
            layout_ctx, &settings.get_sender_label_area(width_available),
            data, env
        );

        let msg_label_list_size = self.msg_content_labels.layout(
            layout_ctx, &settings.get_available_content_area(width_available, is_self_user),
            data, env);
        let total_metadata_width = sender_label_size.width + datetime_label_size.width;

        // Offset in the case of tiny flipped bubbles with tails, since tiny
        // messages cause the tail to not align with the picture properly
        let y_top_offset = settings.get_top_y_offset(is_self_user, &sender_label_size, &msg_label_list_size);

        self.msg_content_labels.set_origin(layout_ctx,
            settings.get_content_origin(
                is_self_user,
                width_available,
                y_top_offset,
                msg_label_list_size.width,
                total_metadata_width,
                sender_label_size.height
            )
        );

        let sender_label_origin = settings.get_sender_origin(
            is_self_user,
            width_available,
            sender_label_size.width,
            datetime_label_size.width,
            msg_label_list_size.height,
            msg_label_list_size.width,
            y_top_offset
        );

        // Position to right of sender. Also account for differences in height.
        let datetime_label_origin = settings.get_datetime_origin(width_available, msg_label_list_size.width,
            &sender_label_origin, &sender_label_size, &datetime_label_size);
        
        self.sender_name_label.set_origin(layout_ctx, sender_label_origin);
        self.datetime_label.set_origin(layout_ctx, datetime_label_origin);

        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        Size::new(bc.max().width, settings.get_total_height(width_available, &sender_label_size, &msg_label_list_size, y_top_offset))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MessageGroup, env: &Env) {
        let settings = LayoutSettings::from_env(env);
        let is_self_user = env.get(crate::SELF_USER_ID_KEY) as u32 == data.user_id;
    
        // First, do the calculations and variables
        self.draw_bubble_background(ctx, &settings, is_self_user);

        // Next, the profile pic
        self.draw_profile_pic(ctx, data, &settings, is_self_user);
        // Now the little arrow/tail that goes from the image to the bubble
        self.draw_bubble_tail(ctx, &settings, is_self_user);

        // Draw text
        self.msg_content_labels.paint(ctx, data, env);
        self.sender_name_label.paint(ctx, data, env);
        self.datetime_label.paint(ctx, data, env);
        if settings.metadata_layout == MetadataLayout::LeftSideBySideWithDot {
            let datetime_position = self.datetime_label.layout_rect();
            let dot_location = druid::Point::new(datetime_position.x0 + DOT_X_OFFSET, (datetime_position.y0 + datetime_position.y1) / 2.0 + DOT_Y_OFFSET);
            let dot_color = settings.get_datetime_color(is_self_user);
            ctx.fill(druid::kurbo::Circle::new(dot_location, DOT_SIZE), &dot_color);
        }
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
        let profile_pic_x_offset = settings.profile_pic_x_origin(
            is_self_user,
            ctx.region().bounding_box().width(),
            self.sender_name_label.layout_rect().size()
        );
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
}