
use druid::{Widget, widget, WidgetPod};
use druid::widget::prelude::*;
use crate::{Message};
use druid::piet::{Color};

/// A widget that shows a single message
/// 
/// It also handles timestamps, the settings menu, reactions, and more.
pub struct SingleMessageWidget {
    msg_content_label: WidgetPod<Message, widget::Label<Message>>,
}

impl SingleMessageWidget {
    pub fn new() -> Self {
        let msg_content_label = WidgetPod::new(
            widget::Label::new(|item: &Message, _env: &_| {
                item.message.to_string()
            })
            .with_line_break_mode(widget::LineBreaking::WordWrap)
            .with_text_size(crate::CONTENT_FONT_SIZE_KEY)
        );
        
        SingleMessageWidget { msg_content_label: msg_content_label }
    }
}

impl Widget<Message> for SingleMessageWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Message, env: &Env) {
        self.msg_content_label.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Message,
        env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(_) => {
                ctx.request_layout();
                ctx.request_paint();
            },
            _ => {}
        }
        self.msg_content_label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Message, data: &Message, env: &Env) {
        self.msg_content_label.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Message,
        env: &Env,
    ) -> Size {
        self.msg_content_label.layout(layout_ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Message, env: &Env) {
        // Draw hot background (for when user's mouse is hovering over it)
        if ctx.is_hot() {
            ctx.fill(
                self.msg_content_label.layout_rect().inflate(3.0, 3.0),
                &Color::rgba8(255, 255, 255, 20)
            );
        }

        self.msg_content_label.paint(ctx, data, env);
    }
}