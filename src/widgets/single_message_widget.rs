use druid::kurbo::Rect;
use druid::{Widget, widget, WidgetPod};
use druid::widget::prelude::*;
use druid::Point;
use crate::{Message};
use druid::piet::Color;
use crate::LayoutSettings;
use crate::helper::helper_functions::{self, TimestampFormat};

/// A widget that shows a single message
/// 
/// It also handles timestamps, the settings menu, reactions, and more.
pub struct SingleMessageWidget {
    msg_content_label: WidgetPod<Message, widget::Label<Message>>,
    timestamp_label: WidgetPod<Message, widget::Label<Message>>,
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
        let timestamp_label = WidgetPod::new(
            widget::Label::new(|item: &Message, env: &Env| {
                let time_format: TimestampFormat = num_traits::FromPrimitive::from_u64(
                    env.get::<u64>(crate::SIDE_TIME_FORMAT_KEY)
                ).expect("Invalid side time format index");

                helper_functions::timestamp_to_display_msg(
                    item.timestamp_epoch_seconds,
                    time_format,
                )
            })
            .with_line_break_mode(widget::LineBreaking::Overflow)
            .with_text_size(crate::DATETIME_FONT_SIZE_KEY)
            .with_text_color(crate::DATETIME_COLOR_KEY)
        );
        
        SingleMessageWidget {
            msg_content_label: msg_content_label,
            timestamp_label: timestamp_label
        }
    }
}

impl Widget<Message> for SingleMessageWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Message, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(crate::REFRESH_UI_SELECTOR) => {
                ctx.request_layout();
                ctx.request_paint();
            }
            _ => {}
        }
        self.msg_content_label.event(ctx, event, data, env);
        self.timestamp_label.event(ctx, event, data, env);
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
        self.timestamp_label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Message, data: &Message, env: &Env) {
        self.msg_content_label.update(ctx, data, env);
        self.timestamp_label.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Message,
        env: &Env,
    ) -> Size {
        let settings = LayoutSettings::from_env(env);
        // Now position the content label
        let msg_content_bc = helper_functions::to_full_height_area(
            bc.max().width - settings.left_spacing
        );
        let msg_content_origin = Point::new(settings.left_spacing, 0.0);
        let msg_size = self.msg_content_label.layout(layout_ctx, &msg_content_bc, data, env);
        self.msg_content_label.set_origin(layout_ctx, msg_content_origin);
        // Now position the timestamp label
        let timestamp_size = self.timestamp_label.layout(layout_ctx, &bc, data, env);
        let timestamp_y = msg_size.height - timestamp_size.height;
        let mut timestamp_x = 0.0 - timestamp_size.width - settings.left_meta_offset - settings.left_spacing;
        timestamp_x -= settings.bubble_padding;
        let timestamp_origin = Point::new(timestamp_x, timestamp_y);
        // Just using the given bc because we don't want it to wrap.
        self.timestamp_label.set_origin(layout_ctx, timestamp_origin);
        msg_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Message, env: &Env) {
        let settings = LayoutSettings::from_env(env);
        // Draw hot background (for when user's mouse is hovering over it)
        if ctx.is_hot() {
            ctx.fill(
                self.msg_content_label.layout_rect().inflate(1.5, 1.5),
                &Color::rgba8(255, 255, 255, 20)
            );
        }

        self.draw_left_line(ctx, &settings);
        self.msg_content_label.paint(ctx, data, env);
        // Always paint because it's only when hot,
        // and because it's out of bounds.
        let is_below_profile_pic = self.msg_content_label.layout_rect().height()
            - self.timestamp_label.layout_rect().height() - settings.picture_size > -10.0;
        if (data.position_in_group > 0 || settings.left_bubble_flipped || is_below_profile_pic)
            && ctx.is_hot()
        {
            self.timestamp_label.paint_always(ctx, data, env);
        }
    }
}

impl SingleMessageWidget {
    
    fn draw_left_line(&self, ctx: &mut PaintCtx, settings: &LayoutSettings) {
        if settings.show_left_line {
            let content_label_rect = self.msg_content_label.layout_rect();
            let line_x0 = content_label_rect.x0 - settings.left_spacing;
            let line_rect = Rect::new(line_x0, content_label_rect.y0, line_x0 + 1.0, content_label_rect.y1);
            ctx.fill(line_rect, &Color::GRAY);
        }
    }
}