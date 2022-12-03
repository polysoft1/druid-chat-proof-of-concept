use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, ImageBuf, AppDelegate, WindowId, WindowHandle};
use druid::widget;
use druid::im;
use druid;
use std::sync;
use tracing::{error, Instrument};
use rand::Rng;
use std::path::Path;
use core::time::Duration;

mod widgets;
use widgets::timeline_item;

use crate::widgets::timeline_item::TimelineItemWidget;

#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
    timeline_data: im::Vector<Message>,
}

#[derive(Clone, druid::Data)]
struct Message {
    user_id: u32,
    profile_pic: ImageBuf,
    message: String,
}

struct Delegate {
    main_window_id: WindowId,
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> Option<druid::Event> {
        Some(event)
    }

    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        _cmd: &druid::Command,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        id: druid::WindowId,
        handle: druid::WindowHandle,
        _data: &mut AppState,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        if id == self.main_window_id {
            println!("Main Window Added");
            println!("PX Required: {}", timeline_item::TimelineItemWidget::get_required_icon_resolution(&handle));
        } else {
            println!("Other Window Added");
        }
    }

    fn window_removed(&mut self, id: druid::WindowId, _data: &mut AppState, _env: &druid::Env, _ctx: &mut druid::DelegateCtx) {
        if id == self.main_window_id {
            println!("Main Window Removed");

        } else {
            println!("Other Window Removed");
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
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
            widget::Svg::new(settings_svg).fix_height(15.0).padding(7.0)
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
            widget::Svg::new(send_svg).fix_height(25.0).padding(5.0)
        );

    let timeline = widget::Scroll::new(
        widget::List::new( move || {
            timeline_item::TimelineItemWidget::new()
        })
        .with_spacing(10.0)
        .padding(5.0)
    )
    .vertical()
    .expand()
    .lens(AppState::timeline_data);

    widget::Flex::column()
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
        .must_fill_main_axis(true)
    }

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let mut initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![],
    };

    let mut rng = rand::thread_rng();
    let mut msg_body = String::new();
    msg_body.push_str("Start of msg.");

    // Find required image resolution to not cause blurry profile pics

    // Load profile pics
    let mut profile_pic_buffers: Vec<ImageBuf> = Vec::new();
    for i in 1..6 {
        let filename = format!("./images/user_{}_55px.png", i);
        let profile_pic_file = Path::new(filename.as_str());
        let img_data = ImageBuf::from_file(profile_pic_file);

        profile_pic_buffers.push(img_data.unwrap());
    }

    for _ in 1..20 {
        msg_body.push_str(" Appended!");
        let user_id = rng.gen_range(0..5);
        let msg = Message { message: msg_body.clone(), user_id: user_id, profile_pic: profile_pic_buffers[user_id as usize].clone()};
        initial_state.timeline_data.push_back(msg);
    }
    let user_id = rng.gen_range(0..5);
    let msg = Message { message: "This\nis\na\nnarrow\nbut\nlong\nmessage.\nHopefully\nthe\nbubble\nstays\nnarrow.".to_string(),
        user_id: user_id, profile_pic: profile_pic_buffers[user_id as usize].clone()};
    initial_state.timeline_data.push_back(msg);

    let window = WindowDesc::new(
        build_ui()
    ).window_size((300.0, 450.0));
    let window_id = window.id;

    AppLauncher::with_window(
        window
    ).delegate(
        Delegate {
            main_window_id: window_id,
        }
    )
    .launch(
        initial_state
    )?;
    Ok(())
}