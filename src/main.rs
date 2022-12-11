use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, ImageBuf, AppDelegate, EventCtx};
use druid::widget;
use druid::im;
use druid;
use std::sync;
use tracing::{error};
use rand::Rng;
use std::path::Path;
use druid::piet::Color;

mod widgets;
use widgets::timeline_item::{self, PictureShape, TailShape};

pub const IMAGE_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.image_shape");
pub const IMAGE_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.image_size");
pub const CHAT_BUBBLE_TAIL_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.tail_shape");
pub const CHAT_BUBBLE_IMG_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_img_spacing");

#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
    timeline_data: im::Vector<Message>,
    profile_pics: im::Vector<ImageBuf>,
    layout_settings: LayoutSettings,
}

#[derive(Clone, druid::Data, druid::Lens)]
struct LayoutSettings {
    settings_open: bool,
    picture_shape: PictureShape,
    picture_size: f64,
    chat_bubble_tail_shape: TailShape,
    chat_bubble_picture_spacing: f64,
}

#[derive(Clone, druid::Data)]
struct Message {
    user_id: u32,
    message: String,
    timestamp_epoch_seconds: i64,
    profile_pic: ImageBuf,
}

struct Delegate {
    window_count: i32,
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
        _id: druid::WindowId,
        _handle: druid::WindowHandle,
        _data: &mut AppState,
        _env: &druid::Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        self.window_count += 1;
    }

    fn window_removed(&mut self, _id: druid::WindowId, _data: &mut AppState, _env: &druid::Env, _ctx: &mut druid::DelegateCtx) {
        self.window_count -= 1;
        if self.window_count <= 0 {
            println!("All windows closed. Quitting...");
            druid::Application::global().quit();
        }
    }
}

fn on_settings_icon_click(ctx: &mut EventCtx, state: &mut AppState, _env: &druid::Env) {
    println!("Settings click");

    if state.layout_settings.settings_open {
        println!("Settings already open. Ignoring.");
    } else {
        state.layout_settings.settings_open = true; // Prevent it from being opened a second time
        let new_win = WindowDesc::new(build_settings_ui());
        ctx.new_window(new_win);
    }
}

fn on_send_icon_click(ctx: &mut EventCtx, state: &mut AppState, _env: &druid::Env) {
    println!("Send click");

    state.timeline_data.push_back(
        Message {
            message: state.text_edit.to_string(),
            timestamp_epoch_seconds: chrono::offset::Local::now().timestamp(),
            user_id: 0,
            profile_pic: state.profile_pics[0].clone(),
        }
    );

    //state.text_edit
}

//fn on_pic_shape_change(ctx: &mut EventCtx, state: &mut PictureShape, env: &druid::Env) {
//}

fn get_chat_window_desc() -> WindowDesc<AppState> {
    let main_window = WindowDesc::new(
        build_chat_ui()
    ).window_size((300.0, 450.0));
    return main_window;
}

fn build_chat_ui() -> impl Widget<AppState> {
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
            timeline_item::TimelineItemWidget::new()
        })
        .with_spacing(6.0)
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
            env.set(IMAGE_SHAPE_KEY, data.layout_settings.picture_shape as u64);
            env.set(IMAGE_SIZE_KEY, data.layout_settings.picture_size as f64);
            env.set(CHAT_BUBBLE_TAIL_SHAPE_KEY, data.layout_settings.chat_bubble_tail_shape as u64);
            env.set(CHAT_BUBBLE_IMG_SPACING_KEY, data.layout_settings.chat_bubble_picture_spacing as f64);
        },
        layout
    )
}

const IMG_SHAPE_OPTIONS: [(&str, PictureShape); 5] =
[
    ("Circle", PictureShape::Circle),
    ("Rectangle", PictureShape::Rectangle),
    ("Rounded Rectangle", PictureShape::RoundedRectangle),
    ("Hexagon", PictureShape::Hexagon),
    ("Octagon", PictureShape::Octagon),
];
const TAIL_SHAPE_OPTIONS: [(&str, TailShape); 2] =
[
    ("Concave Bottom", TailShape::ConcaveBottom),
    ("Straight", TailShape::Straight),
];

fn build_settings_ui() -> impl Widget<AppState> {
    widget::Flex::column()
        .with_child(
            widget::Label::new("Layout Settings")
                .with_text_size(20.0).padding(8.0).align_left()
        )
        .with_default_spacer()
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Profile Pic Shape:").align_right()
                , 1.0)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(IMG_SHAPE_OPTIONS)
                        //.on_click(on_pic_shape_change)
                        .lens(LayoutSettings::picture_shape)
                , 1.0)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Size:").align_right()
                , 1.0)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(10.0, 100.0).with_step(1.0)
                    .lens(LayoutSettings::picture_size)
                , 0.7)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:1}", data.picture_size)}),
                    0.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(20.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Bubble Tail Shape:").align_right()
                , 1.0)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(TAIL_SHAPE_OPTIONS)
                        //.on_click(on_pic_shape_change)
                        .lens(LayoutSettings::chat_bubble_tail_shape)
                , 1.0)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Bubble Spacing:").align_right()
                , 1.0)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 15.0).with_step(0.1)
                    .lens(LayoutSettings::chat_bubble_picture_spacing)
                , 0.7)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_picture_spacing)}),
                    0.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(30.0)
        .with_child(
            widget::Label::new("Shape changes require scroll to take effect\nSize changes require window resize to take effect")
                .with_text_size(20.0).with_text_color(Color::RED)
        )
}

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let mut initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![],
        profile_pics: im::vector![],
        layout_settings: LayoutSettings {
            settings_open: false,
            picture_shape: PictureShape::Circle,
            picture_size: 35.0,
            chat_bubble_tail_shape: TailShape::ConcaveBottom,
            chat_bubble_picture_spacing: 3.5,
        }
    };

    let mut rng = rand::thread_rng();
    let mut msg_body = String::new();
    msg_body.push_str("Start of msg.");

    // Find required image resolution to not cause blurry profile pics

    // Load profile pics
    for i in 1..6 {
        let filename = format!("./images/user_{}_55px.png", i);
        let profile_pic_file = Path::new(filename.as_str());
        let img_data = ImageBuf::from_file(profile_pic_file);

        initial_state.profile_pics.push_back(img_data.unwrap());
    }

    let mut time = chrono::offset::Local::now().timestamp();
    time -= 10; // 10 seconds ago
    let mut offset_amount = 60;

    for _ in 1..20 {
        msg_body.push_str(" Appended!");
        let user_id = rng.gen_range(0..5);
        let msg = Message {
            timestamp_epoch_seconds: time,
            message: msg_body.clone(),
            user_id: user_id,
            profile_pic: initial_state.profile_pics[user_id as usize].clone(),
        };
        initial_state.timeline_data.push_front(msg);
        time -= offset_amount;
        offset_amount *= 2;
    }
    let user_id = rng.gen_range(0..5);
    let msg = Message {
        timestamp_epoch_seconds: time,
        message: "This\nis\na\nnarrow\nbut\nlong\nmessage.\nHopefully\nthe\nbubble\nstays\nnarrow.".to_string(),
        user_id: user_id,
        profile_pic: initial_state.profile_pics[user_id as usize].clone(),
    };
    initial_state.timeline_data.push_front(msg);

    AppLauncher::with_window(
        get_chat_window_desc()
    ).delegate(
        Delegate {
            window_count: 0,
        }
    )
    .launch(
        initial_state
    )?;
    Ok(())
}