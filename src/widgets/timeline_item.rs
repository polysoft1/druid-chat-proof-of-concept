use druid::kurbo::{Circle, RoundedRect};
use druid::widget::prelude::*;
use druid::{Widget, WidgetExt, widget};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid;
use crate::Message;
use num_traits;
use num_derive;

pub struct TimelineItemWidget {
    msg_content_label: WidgetPod<Message, widget::Container<Message>>,
    sender_name_label: WidgetPod<Message, widget::Padding<Message, widget::Label<Message>>>,
}

const MSG_COLOR: Color = Color::rgb8(75, 75, 76);
const SUB_TEXT_COLOR: Color = Color::rgb8(175, 175, 175);
const ARROW_SIZE: f64 = 7.0;

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
}

fn make_tail_path(tail_area: f64, shape: TailShape) -> kurbo::BezPath {
    let mut path = kurbo::BezPath::new();
    path.move_to(Point::new(tail_area, 0.0)); // Start
    path.line_to(Point::new(tail_area - ARROW_SIZE, 0.0)); // towards picture
    // Now to low point. + is down
    match shape {
        TailShape::ConcaveBottom => {
            path.quad_to(
                Point::new(tail_area - ARROW_SIZE/4.0, ARROW_SIZE/4.0),
                Point::new(tail_area, ARROW_SIZE * 1.3),
            );
        }
        TailShape::Straight => {
            path.line_to(Point::new(tail_area, ARROW_SIZE));
        }
    }

    // To right to cover the curve of the bubble. Double size to ensure coverage of bubble.
    path.line_to(Point::new(tail_area + ARROW_SIZE * 2.0, 0.0));
    path.line_to(Point::new(tail_area, 0.0));
    path.close_path();
    path
}
fn make_hexagon_path(vertical_trim: f64, inset: f64, pic_width: f64) -> kurbo::BezPath {
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
    path
}

fn make_octagon_path(fraction_from_corner: f64, pic_width: f64) -> kurbo::BezPath {
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
    path
}

impl TimelineItemWidget {
    pub fn new() -> Self {
        let msg_content_label = WidgetPod::new(
            widget::Label::new(|item: &Message, _env: &_| item.message.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .with_text_size(14.0)
                .padding(7.0)
                .background(MSG_COLOR)
                .rounded(7.0));
        let sender_name_label = WidgetPod::new(
            widget::Label::new(|item: &Message, _env: &_| {
                let mut username = "User".to_string();
                username.push_str(item.user_id.to_string().as_str());
                username
        })
            .with_line_break_mode(widget::LineBreaking::WordWrap)
            .with_text_size(12.0)
            .with_text_color(SUB_TEXT_COLOR)
            .padding(3.0)
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
        let profile_pic_width = env.get(crate::IMAGE_SIZE_KEY);
        let profile_pic_bubble_spacing = env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY);
        let profile_pic_area = profile_pic_width + profile_pic_bubble_spacing;

        // Label, which is offset to right to fit profile pic
        let msg_label_origin: Point = Point::new(profile_pic_area, 0.0);
        let label_bounding_box = BoxConstraints::new(
            Size::new(0.0, 0.0),
            Size::new(bc.max().width - profile_pic_area, bc.max().height)
        );
        self.msg_content_label.set_origin(layout_ctx, data, env, msg_label_origin);

        let msg_label_size = self.msg_content_label.layout(layout_ctx, &label_bounding_box, data, env);

        let sender_label_origin: Point = Point::new(profile_pic_area, msg_label_size.height);
        self.sender_name_label.set_origin(layout_ctx, data, env, sender_label_origin);
        let sender_label_size = self.sender_name_label.layout(layout_ctx, &label_bounding_box, data, env);

        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        Size::new(bc.max().width, msg_label_size.height + sender_label_size.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Message, env: &Env) {
        self.msg_content_label.paint(ctx, data, env);
        self.sender_name_label.paint(ctx, data, env);

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
        let profile_pic_width = env.get(crate::IMAGE_SIZE_KEY);
        let profile_pic_area = env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY) + profile_pic_width;

        let piet_image = {
            let image_data = data.profile_pic.clone();
            image_data.to_image(ctx.render_ctx)
        };
        ctx.with_save(|ctx| { // Makes it so the clip doesn't mess up the following draws
            let shape_as_int = env.get(crate::IMAGE_SHAPE_KEY);
            match num_traits::FromPrimitive::from_u64(shape_as_int) {
                Some(PictureShape::Rectangle) => {},
                Some(PictureShape::RoundedRectangle) => ctx.clip(RoundedRect::new(0.0, 0.0, profile_pic_width, profile_pic_width, 6.0)),
                Some(PictureShape::Circle) => ctx.clip(Circle::new(Point::new(profile_pic_width / 2.0, profile_pic_width / 2.0), profile_pic_width / 2.0)),
                Some(PictureShape::Hexagon) => ctx.clip(make_hexagon_path(0.08, 0.25, profile_pic_width)),
                Some(PictureShape::Octagon) => ctx.clip(make_octagon_path(0.25, profile_pic_width)),
                None => eprintln!("unknown number"),
            }
            ctx.draw_image(&piet_image,
                druid::Rect::new(0.0, 0.0, profile_pic_width, profile_pic_width),
                    druid::piet::InterpolationMode::Bilinear
            );
        });
        let tail_shape_int = env.get(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY);
        // Now the little arrow that goes from the image to the bubble
        ctx.fill(make_tail_path(
            profile_pic_area,
            num_traits::FromPrimitive::from_u64(tail_shape_int).expect("Invalid tail shape")
        ), &MSG_COLOR);
    }


}