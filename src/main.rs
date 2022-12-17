use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, ImageBuf, AppDelegate, EventCtx};
use druid::widget;
use druid::im;
use druid;
use std::sync;
use tracing::{error};
use rand::Rng;
use std::path::Path;
use druid::piet::Color;
use std::env;

mod widgets;
use widgets::timeline_item::{self, PictureShape, TailShape, ItemLayoutOption};

pub const ITEM_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.item_layout");
pub const IMAGE_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.image_shape");
pub const IMAGE_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.image_size");
pub const CHAT_BUBBLE_TAIL_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.tail_shape");
pub const CHAT_BUBBLE_RADIUS_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_radius");
pub const CHAT_BUBBLE_IMG_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_img_spacing");
pub const SELF_USER_ID_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.self_user");
pub const SHOW_SELF_PROFILE_PIC: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.show_self_pic");
pub const MSG_PADDING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.msg_padding");

#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
    timeline_data: im::Vector<Message>,
    profile_pics: im::Vector<ImageBuf>,
    layout_settings: LayoutSettings,
}

#[derive(Clone, druid::Data, druid::Lens)]
struct LayoutSettings {
    item_layout: ItemLayoutOption,
    settings_open: bool,
    picture_shape: PictureShape,
    picture_size: f64,
    chat_bubble_tail_shape: TailShape,
    chat_bubble_radius: f64,
    chat_bubble_picture_spacing: f64,
    show_self_pic: bool,
    msg_padding: f64,
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

    fn window_removed(&mut self, _id: druid::WindowId, data: &mut AppState, _env: &druid::Env, _ctx: &mut druid::DelegateCtx) {
        self.window_count -= 1;
        data.layout_settings.settings_open = false;
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
        let settings_size = druid::Size::new(480.0, 760.0);
        let mut new_win = WindowDesc::new(build_settings_ui()).resizable(false);
        new_win = new_win.window_size(settings_size);
        ctx.new_window(new_win);
    }
}

fn on_send_icon_click(_ctx: &mut EventCtx, state: &mut AppState, env: &druid::Env) {
    println!("Send click");

    // Find which user is self
    let self_id = env.get(SELF_USER_ID_KEY);

    state.timeline_data.push_back(
        Message {
            message: state.text_edit.to_string(),
            timestamp_epoch_seconds: chrono::offset::Local::now().timestamp(),
            user_id: self_id as u32,
            profile_pic: state.profile_pics[self_id as usize].clone(),
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
            env.set(ITEM_LAYOUT_KEY, data.layout_settings.item_layout as u64);
            env.set(IMAGE_SHAPE_KEY, data.layout_settings.picture_shape as u64);
            env.set(IMAGE_SIZE_KEY, data.layout_settings.picture_size as f64);
            env.set(CHAT_BUBBLE_TAIL_SHAPE_KEY, data.layout_settings.chat_bubble_tail_shape as u64);
            env.set(CHAT_BUBBLE_RADIUS_KEY, data.layout_settings.chat_bubble_radius as f64);
            env.set(CHAT_BUBBLE_IMG_SPACING_KEY, data.layout_settings.chat_bubble_picture_spacing as f64);
            env.set(SHOW_SELF_PROFILE_PIC, data.layout_settings.show_self_pic);
            env.set(MSG_PADDING_KEY, data.layout_settings.msg_padding as f64);
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
const TAIL_SHAPE_OPTIONS: [(&str, TailShape); 3] =
[
    ("Concave Bottom", TailShape::ConcaveBottom),
    ("Straight", TailShape::Straight),
    ("Hidden", TailShape::Hidden),
];
const LAYOUT_OPTIONS: [(&str, ItemLayoutOption); 5] =
[
    ("Bubble w/ bottom external metadata", ItemLayoutOption::BubbleExternBottomMeta),
    ("Bubble w/ bottom internal metadata", ItemLayoutOption::BubbleInternalBottomMeta),
    ("Bubble w/ top metadata", ItemLayoutOption::BubbleInternalTopMeta),
    ("Bubbleless", ItemLayoutOption::Bubbleless),
    ("IRC Style", ItemLayoutOption::IRCStyle),
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
                    widget::Label::new("Item Layout:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(LAYOUT_OPTIONS)
                        //.on_click(on_pic_shape_change)
                        .lens(LayoutSettings::item_layout)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(15.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Profile Pic Shape:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(IMG_SHAPE_OPTIONS)
                        //.on_click(on_pic_shape_change)
                        .lens(LayoutSettings::picture_shape)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Size:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(10.0, 100.0).with_step(1.0)
                    .lens(LayoutSettings::picture_size)
                , 1.0)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:1}", data.picture_size)}),
                    0.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(20.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Show Self Profile Pic:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .lens(LayoutSettings::show_self_pic)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        .lens(AppState::layout_settings))
        .with_spacer(20.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Bubble Tail Shape:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(TAIL_SHAPE_OPTIONS)
                        //.on_click(on_pic_shape_change)
                        .lens(LayoutSettings::chat_bubble_tail_shape)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Bubble Radius:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 10.0).with_step(0.5)
                    .lens(LayoutSettings::chat_bubble_radius)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_radius)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(-8.0, 15.0).with_step(0.1)
                    .lens(LayoutSettings::chat_bubble_picture_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_picture_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Msg Padding:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 10.0).with_step(0.5)
                    .lens(LayoutSettings::msg_padding)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.msg_padding)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                .lens(AppState::layout_settings)
        )
        .with_spacer(30.0)
        .with_child(
            widget::Label::new("Shape changes require scroll to take effect\nSize changes require window resize to take effect")
                .with_text_size(20.0).with_text_color(Color::RED)
        )
}

fn get_self_user_from_args() -> u64 {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let parsed_input = args[1].parse::<u64>();
        match parsed_input {
            Ok(user_id) => {
                return user_id;
            },
            Err(_e) => {
                eprintln!("Could not parse first arg. Expected int as user ID.");
                return 0;
            }
        }
    } else {
        println!("Using default self user 0");
        return 0;
    }
}

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let mut initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![],
        profile_pics: im::vector![],
        layout_settings: LayoutSettings {
            item_layout: ItemLayoutOption::BubbleExternBottomMeta,
            settings_open: false,
            picture_shape: PictureShape::Circle,
            picture_size: 30.0,
            chat_bubble_tail_shape: TailShape::ConcaveBottom,
            chat_bubble_radius: 5.0,
            chat_bubble_picture_spacing: 3.5,
            show_self_pic: true,
            msg_padding: 5.0,
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
    // Set self user
    let mut self_id = get_self_user_from_args();
    if self_id > 4 {
        self_id = 0;
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
    .configure_env(move |env, _| {
        // Makes it so the entire UI knows which ID the user is.
        env.set(SELF_USER_ID_KEY, self_id);
    })
    .launch(
        initial_state
    )?;
    Ok(())
}