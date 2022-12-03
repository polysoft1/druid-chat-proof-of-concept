use druid::kurbo::{Circle, RoundedRect};
use druid::widget::prelude::*;
use druid::{Widget, WidgetExt, widget, WindowHandle};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;
use druid::Screen;

use crate::Message;

pub struct TimelineItemWidget {
    label: WidgetPod<Message, widget::Container<Message>>,
}

const PROFILE_PIC_WIDTH: f64 = 35.0;
const PROFILE_PIC_SPACING: f64 = 3.5;
const PROFILE_PIC_AREA: f64 = PROFILE_PIC_WIDTH + PROFILE_PIC_SPACING;
const MSG_COLOR: Color = Color::rgb8(75, 75, 76);
const ARROW_SIZE: f64 = 7.0;
const ARROW_X: f64 = PROFILE_PIC_AREA;
const CURVE_PATH: bool = true;
const PICTURE_SHAPE: PictureShape = PictureShape::Circle;
pub enum PictureShape {
    Rectangle,
    RoundedRectangle,
    Circle,
    Hexagon,
    Octagon,
}

fn make_arrow_path() -> kurbo::BezPath {
    let mut path = kurbo::BezPath::new();
    path.move_to(Point::new(ARROW_X, 0.0)); // Start
    path.line_to(Point::new(ARROW_X - ARROW_SIZE, 0.0)); // towards picture
    // Now to low point. + is down
    if CURVE_PATH {
        path.quad_to(
            Point::new(ARROW_X - ARROW_SIZE/4.0, ARROW_SIZE/4.0),
            Point::new(ARROW_X, ARROW_SIZE * 1.3),
        );
    } else {
        path.line_to(Point::new(ARROW_X, ARROW_SIZE));            
    }
    // To right to cover the curve of the bubble. Double size to ensure coverage of bubble.
    path.line_to(Point::new(ARROW_X + ARROW_SIZE * 2.0, 0.0));
    path.line_to(Point::new(ARROW_X, 0.0));
    path.close_path();
    path
}
fn make_hexagon_path(vertical_trim: f64, inset: f64) -> kurbo::BezPath {
    let mut path = kurbo::BezPath::new();
    let second_x = PROFILE_PIC_WIDTH * inset;
    let third_x = PROFILE_PIC_WIDTH * (1.0 - inset);
    let top_y = PROFILE_PIC_WIDTH * vertical_trim;
    let middle_y = PROFILE_PIC_WIDTH / 2.0;
    let bottom_y = PROFILE_PIC_WIDTH * (1.0 - vertical_trim);
    path.move_to(Point::new(0.0, middle_y)); // Start
    path.line_to(Point::new( second_x, top_y));
    path.line_to(Point::new( third_x, top_y));
    path.line_to(Point::new( PROFILE_PIC_WIDTH, middle_y));
    path.line_to(Point::new( third_x, bottom_y));
    path.line_to(Point::new( second_x, bottom_y));
    path.line_to(Point::new(0.0, middle_y));
    path.close_path();
    path
}

fn make_octagon_path(fraction_from_corner: f64) -> kurbo::BezPath {
    let dist_from_corner = PROFILE_PIC_WIDTH * fraction_from_corner;
    let other_side_pos = PROFILE_PIC_WIDTH - dist_from_corner;

    let mut path = kurbo::BezPath::new();
    path.move_to(Point::new(0.0, dist_from_corner)); // Start
    path.line_to(Point::new( dist_from_corner, 0.0));
    path.line_to(Point::new( other_side_pos, 0.0));
    path.line_to(Point::new( PROFILE_PIC_WIDTH, dist_from_corner));
    path.line_to(Point::new( PROFILE_PIC_WIDTH, other_side_pos));
    path.line_to(Point::new( other_side_pos, PROFILE_PIC_WIDTH));
    path.line_to(Point::new( dist_from_corner, PROFILE_PIC_WIDTH));
    path.line_to(Point::new( 0.0, other_side_pos));
    path.close_path();
    path
}

impl TimelineItemWidget {
    pub fn new() -> Self {
        let label = WidgetPod::new(widget::Label::new(|item: &Message, _env: &_| item.message.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .padding(10.0)
                .background(MSG_COLOR)
                .rounded(7.0));
        Self {
            label: label,
        }
    }

    pub fn get_required_icon_resolution(window: &WindowHandle) -> f64 {
        let scale_request = window.get_scale();
        match scale_request {
            Ok(scale) => {
                println!("Scale_Y: {}", scale.y());
                scale.x() * PROFILE_PIC_WIDTH
            },
            Err(e) => {
                eprintln!("Error getting scale: {}", e);
                return PROFILE_PIC_WIDTH;
            }
        }
    }
}

impl Widget<Message> for TimelineItemWidget {

    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Message, _env: &Env) {
        self.label.event(_ctx, _event, _data, _env);
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Message,
        _env: &Env,
    ) {
        self.label.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Message, _data: &Message, _env: &Env) {
        self.label.update(_ctx, _data, _env);
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Message,
        _env: &Env,
    ) -> Size {
        // Label, which is offset to right to fit profile pic
        let label_origin: Point = Point::new(PROFILE_PIC_AREA, 0.0);
        let label_bounding_box = BoxConstraints::new(
            Size::new(0.0, 0.0),
            Size::new(bc.max().width - PROFILE_PIC_AREA, bc.max().height)
        );
        self.label.set_origin(_layout_ctx, _data, _env, label_origin);

        let label_size = self.label.layout(_layout_ctx, &label_bounding_box, _data, _env);        
        
        // The image is at the top left if other, or top right if self (if shown)
        // Potential future support for bottom images
        Size::new(bc.max().width, label_size.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Message, env: &Env) {
        self.label.paint(ctx, data, env);

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
        
        let piet_image = {
            let image_data = data.profile_pic.clone();
            image_data.to_image(ctx.render_ctx)
        };
        ctx.with_save(|ctx| { // Makes it so the clip doesn't mess up the following draws
            match PICTURE_SHAPE {
                PictureShape::Rectangle => {},
                PictureShape::RoundedRectangle => ctx.clip(RoundedRect::new(0.0, 0.0, PROFILE_PIC_WIDTH, PROFILE_PIC_WIDTH, 6.0)),
                PictureShape::Circle => ctx.clip(Circle::new(Point::new(PROFILE_PIC_WIDTH / 2.0, PROFILE_PIC_WIDTH / 2.0), PROFILE_PIC_WIDTH / 2.0)),
                PictureShape::Hexagon => ctx.clip(make_hexagon_path(0.08, 0.25)),
                PictureShape::Octagon => ctx.clip(make_octagon_path(0.25)),
            }
            ctx.draw_image(&piet_image, 
                druid::Rect::new(0.0, 0.0, PROFILE_PIC_WIDTH, PROFILE_PIC_WIDTH),
                    druid::piet::InterpolationMode::Bilinear
            );
        });
        // Now the little arrow that goes from the image to the bubble
        ctx.fill(make_arrow_path(), &MSG_COLOR);
    }


}