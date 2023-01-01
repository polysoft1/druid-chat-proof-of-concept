use druid::{WindowDesc, Widget, WidgetExt, EventCtx, im};
use druid::widget;
use crate::{AppState, Message, MessageGroup};
use tracing::error;
use crate::settings_ui::build_settings_ui;
use crate::widgets::timeline_item_widget;

pub(crate) fn build_chat_ui() -> impl Widget<AppState> {
    let send_svg = match include_str!("./assets/send.svg").parse::<widget::SvgData>() {
        Ok(svg) => svg,
        Err(err) => {
            error!("{}", err);
            error!("Using an empty SVG instead.");
            widget::SvgData::default()
        }
    };
    let settings_svg = match include_str!("./assets/settings_gear.svg").parse::<widget::SvgData>() {
        Ok(svg) => svg,
        Err(err) => {
            error!("{}", err);
            error!("Using an empty SVG instead.");
            widget::SvgData::default()
        }
    };

    let title = widget::Flex::row()
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
        .background(druid::theme::BACKGROUND_LIGHT);

    let editor = widget::Flex::row()
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
            
        );

    let timeline = widget::Scroll::new(
        widget::List::new( move || {
            timeline_item_widget::TimelineItemWidget::new()
        })
        .with_spacing(crate::GROUP_SPACING_KEY)
        .padding(5.0)
    )
    .vertical()
    .expand()
    .lens(AppState::timeline_data);

    let layout = widget::Flex::column()
        // Title
        .with_child(
            title
        )
        // The timeline itself
        .with_flex_child(timeline, 1.0)
        // The bottom editor
        .with_child(
            editor
        )
        .must_fill_main_axis(true);
    widget::EnvScope::new(
        |env: &mut druid::env::Env, data: &AppState| {
            data.layout_settings.set_env(env);
        },
        layout
    )
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
        let settings_size = druid::Size::new(1400.0, 670.0);
        let mut new_win = WindowDesc::new(build_settings_ui()).resizable(false);
        new_win = new_win.window_size(settings_size);
        ctx.new_window(new_win);
    }
}