use druid::widget::prelude::*;
use druid::{Widget, WidgetExt, widget};
use druid::piet::{Color, kurbo};
use druid::WidgetPod;
use druid::Point;

use crate::Message;

pub struct TimelineItemWidget {
    label: WidgetPod<Message, widget::Container<Message>>,
}

const PROFILE_PIC_WIDTH: f64 = 40.0;
const PROFILE_PIC_SPACING: f64 = 2.0;
const PROFILE_PIC_AREA: f64 = PROFILE_PIC_WIDTH + PROFILE_PIC_SPACING;
const MSG_COLOR: Color = Color::rgb8(75, 75, 76);
const ARROW_SIZE: f64 = 8.0;
const ARROW_X: f64 = PROFILE_PIC_AREA;
const CURVE_PATH: bool = true;

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
    path.line_to(Point::new(ARROW_X + ARROW_SIZE, 0.0));            
    // To right to cover the curve of the bubble
    path.line_to(Point::new(ARROW_X, 0.0));
    path.close_path();
    path
}

impl TimelineItemWidget {
    pub fn new() -> Self {
        // Loads the image
        // TODO: Make it so that all pics for the same user use the same ImageBuf
        //let filename = format!("./user_{}.webp", |item: &Message, _env: &_| item.user_id);

        // This needs image and png features enabled
        let label = WidgetPod::new(widget::Label::new(|item: &Message, _env: &_| item.message.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .padding(10.0)
                .background(MSG_COLOR)
                .rounded(10.0));
        Self {
            label: label,
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
        ctx.draw_image(&piet_image, 
            druid::Rect::new(0.0, 0.0, PROFILE_PIC_WIDTH, PROFILE_PIC_WIDTH),
             druid::piet::InterpolationMode::Bilinear
        );
        // Now the little arrow that goes from the image to the bubble
        ctx.fill(make_arrow_path(), &MSG_COLOR);
    }


}