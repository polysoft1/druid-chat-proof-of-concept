use druid::{WindowDesc, Widget, WidgetPod, WidgetExt, EventCtx, im};
use druid::widget;
use crate::{AppState, Message, MessageGroup};
use super::timeline_item_widget;
use tracing::error;
use crate::settings_ui::build_settings_ui;

pub struct ChatWindowWidget {
    header: WidgetPod<AppState, widget::Container<AppState>>,
    timeline: WidgetPod<AppState, Box<dyn druid::Widget<AppState>>>,
    footer: WidgetPod<AppState, widget::Flex<AppState>>,
}

impl ChatWindowWidget {
    pub fn new() -> ChatWindowWidget {
        ChatWindowWidget { header: Self::build_title(), timeline: Self::build_timeline(), footer: Self::build_footer() }
    }

    fn build_title() -> WidgetPod<AppState, widget::Container<AppState>> {
        let settings_svg = match include_str!("../assets/settings_gear.svg").parse::<widget::SvgData>() {
            Ok(svg) => svg,
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead.");
                widget::SvgData::default()
            }
        };

        WidgetPod::new(widget::Flex::row()
            .with_flex_child(
                widget::Label::new("Chat Title")
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .padding(7.0)
                .expand_width(),
            1.0)
            .with_child(
                widget::ControllerHost::new(
                    widget::Svg::new(settings_svg).fix_height(15.0).padding(7.0),
                    widget::Click::new(on_settings_icon_click)
                )
            )
            .background(druid::theme::BACKGROUND_LIGHT)
        )
    }

    fn build_timeline() -> WidgetPod<AppState, Box<dyn druid::Widget<AppState>>> {
        WidgetPod::new(
            widget::Scroll::new(
                widget::List::new( move || {
                    timeline_item_widget::TimelineItemWidget::new()
                })
                .with_spacing(crate::GROUP_SPACING_KEY)
                .padding(5.0)
            )
            .vertical()
            .expand()
            .lens(AppState::timeline_data)
            .boxed()
        )
    }

    fn build_footer() -> WidgetPod<AppState, widget::Flex<AppState>> {
        let send_svg = match include_str!("../assets/send.svg").parse::<widget::SvgData>() {
            Ok(svg) => svg,
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead.");
                widget::SvgData::default()
            }
        };

        WidgetPod::new(widget::Flex::row()
            .with_flex_child(
                widget::TextBox::multiline()
                    .with_placeholder("Message...")
                    .lens(AppState::text_edit)
                    .padding(1.0)
                    .expand_width(),
            1.0)
            .with_child(
                widget::ControllerHost::new(
                    widget::Svg::new(send_svg).fix_height(25.0).padding(5.0),
                    widget::Click::new(on_send_icon_click)
                )
                
            )
        )
    }


}

impl Widget<AppState> for ChatWindowWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, env: &druid::Env) {
        self.header.event(ctx, event, data, env);
        self.timeline.event(ctx, event, data, env);
        self.footer.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &AppState, env: &druid::Env) {
        self.header.lifecycle(ctx, event, data, env);
        self.timeline.lifecycle(ctx, event, data, env);
        self.footer.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &AppState, data: &AppState, env: &druid::Env) {
        self.header.update(ctx, data, env);
        self.timeline.update(ctx, data, env);
        self.footer.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, env: &druid::Env) -> druid::Size {
        let header_max_size = druid::BoxConstraints::new(
            druid::Size::new(0.0, 0.0), bc.max());
        let header_size = self.header.layout(ctx, &header_max_size, data, env);

        // Footer size limit is total height minus the height of the header, minus 100
        let footer_max_size = druid::BoxConstraints::new(
            druid::Size::new(0.0, 0.0), druid::Size::new(bc.max().width, 0.0f64.max(bc.max().height - header_size.height - 100.0)));
        let footer_size = self.footer.layout(ctx, &footer_max_size, data, env);

        let content_max_size = druid::BoxConstraints::new(
            druid::Size::new(0.0, 0.0), druid::Size::new(bc.max().width, 0.0f64.max(bc.max().height - header_size.height - footer_size.height)));

        let timeline_size = self.timeline.layout(ctx, &content_max_size, data, env);

        self.timeline.set_origin(ctx, druid::Point::new(0.0, header_size.height));
        self.footer.set_origin(ctx, druid::Point::new(0.0, header_size.height + timeline_size.height));

        druid::Size::new(bc.max().width, bc.max().height)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.header.paint(ctx, data, env);
        self.timeline.paint(ctx, data, env);
        self.footer.paint(ctx, data, env);
    }
}


fn on_send_icon_click(_ctx: &mut EventCtx, state: &mut AppState, env: &druid::Env) {
    println!("Send click");

    // Find which user is self
    let self_id = env.get(crate::SELF_USER_ID_KEY);

    // TODO: Check to see if last thing in the timeline is a message from
    // self user to append to existing group.
    state.timeline_data.push_back(
        MessageGroup {
            messages: im::vector![
                Message {
                    message: state.text_edit.to_string(),
                    position_in_group: 0,
                    timestamp_epoch_seconds: chrono::offset::Local::now().timestamp()
                }
            ],
            user_id: self_id as u32,
            profile_pic: state.profile_pics[self_id as usize].clone(),
        }
    );

    //state.text_edit
}

fn on_settings_icon_click(ctx: &mut EventCtx, state: &mut AppState, _env: &druid::Env) {
    println!("Settings click");

    if state.settings_open {
        println!("Settings already open. Ignoring.");
    } else {
        state.settings_open = true; // Prevent it from being opened a second time
        let settings_size = druid::Size::new(1400.0, 750.0);
        let mut new_win = WindowDesc::new(build_settings_ui()).resizable(false);
        new_win = new_win.window_size(settings_size);
        ctx.new_window(new_win);
    }
}
