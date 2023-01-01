use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, ImageBuf, AppDelegate, EventCtx};
use druid::widget;
use druid::im;
use druid;
use rand::rngs::ThreadRng;
use std::sync;
use tracing::{error};
use rand::Rng;
use std::path::Path;
use std::env;

mod widgets;
use widgets::timeline_item_widget::{self, PictureShape, TailShape, ItemLayoutOption};

// Env keys to define layout in the environment
pub const ITEM_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.item_layout");
pub const PICTURE_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.picture_shape");
pub const PICTURE_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.picture_size");
pub const CHAT_BUBBLE_TAIL_SHAPE_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.tail_shape");
pub const CHAT_BUBBLE_TAIL_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.tail_size");
pub const CHAT_BUBBLE_RADIUS_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_radius");
pub const CHAT_BUBBLE_IMG_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_img_spacing");
pub const SELF_USER_ID_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.self_user");
pub const SHOW_SELF_PROFILE_PIC_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.show_self_pic");
pub const BUBBLE_PADDING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.bubble_padding");
pub const METADATA_CONTENT_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.metadata_content_padding");
pub const ALIGN_TO_PICTURE: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.align-to-picture");
pub const GROUP_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.group_spacing");
pub const SINGLE_MESSAGE_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.single_message_spacing");
pub const SHOW_LEFT_LINE_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.show_left_line");
pub const LEFT_SPACING_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.left_spacing");
pub const LEFT_BUBBLE_FLIPPED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.left_bubble_flipped");
pub const RIGHT_BUBBLE_FLIPPED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.right_bubble_flipped");
pub const SENDER_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.sender_font_size");
pub const CONTENT_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.content_font_size");
pub const DATETIME_FONT_SIZE_KEY: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.datetime_font_size");
pub const HEADER_FONT_BOLDED_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.metadata_font_bolded");
pub const COMPACT_DATETIME_KEY: druid::env::Key<bool> = druid::env::Key::new("polysoft.druid-demo.compact_datetime");
pub const SENDER_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.sender_color");
pub const DATETIME_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.datetime_color");
// Commands to communicate things that need to happen
const REFRESH_UI_SELECTOR: druid::Selector = druid::Selector::new("olysoft.druid-demo.refresh_ui");


#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
    timeline_data: im::Vector<MessageGroup>,
    profile_pics: im::Vector<ImageBuf>,
    layout_settings: LayoutSettings,
    settings_open: bool,
}

#[derive(Clone, druid::Data, druid::Lens)]
struct LayoutSettings {
    item_layout: ItemLayoutOption,
    picture_shape: PictureShape,
    picture_size: f64,
    chat_bubble_tail_shape: TailShape,
    chat_bubble_tail_size: f64,
    chat_bubble_radius: f64,
    chat_bubble_picture_spacing: f64,
    show_self_pic: bool,
    bubble_padding: f64,
    metadata_content_spacing: f64,
    align_to_picture: bool,
    group_spacing: f64,
    single_message_spacing: f64,
    show_left_line: bool,
    left_spacing: f64,
    left_bubble_flipped: bool,
    right_bubble_flipped: bool,
    content_font_size: f64,
    sender_font_size: f64,
    datetime_font_size: f64,
    metadata_font_bolded: bool,
    compact_datetime: bool,
    sender_color: SimpleColor,
    datetime_color: SimpleColor,
}

impl LayoutSettings {
    fn from_env(env: &druid::Env) -> LayoutSettings{
        let sender_color = env.get(SENDER_COLOR_KEY).as_rgba8();
        let datetime_color = env.get(DATETIME_COLOR_KEY).as_rgba8();
        LayoutSettings {
            item_layout: num_traits::FromPrimitive::from_u64(env.get(crate::ITEM_LAYOUT_KEY)).expect("Invalid layout index"),
            picture_shape: num_traits::FromPrimitive::from_u64(env.get(crate::PICTURE_SHAPE_KEY)).expect("Invalid layout index"),
            picture_size: env.get(crate::PICTURE_SIZE_KEY),
            chat_bubble_tail_shape: num_traits::FromPrimitive::from_u64(env.get(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY)).expect("Invalid layout index"),
            chat_bubble_tail_size: env.get(CHAT_BUBBLE_TAIL_SIZE_KEY),
            chat_bubble_radius: env.get(CHAT_BUBBLE_RADIUS_KEY),
            chat_bubble_picture_spacing: env.get(CHAT_BUBBLE_IMG_SPACING_KEY),
            show_self_pic: env.get(SHOW_SELF_PROFILE_PIC_KEY),
            bubble_padding: env.get(BUBBLE_PADDING_KEY),
            metadata_content_spacing: env.get(METADATA_CONTENT_SPACING_KEY),
            align_to_picture: env.get(ALIGN_TO_PICTURE),
            group_spacing: env.get(GROUP_SPACING_KEY),
            single_message_spacing: env.get(SINGLE_MESSAGE_SPACING_KEY),
            show_left_line: env.get(SHOW_LEFT_LINE_KEY),
            left_spacing: env.get(LEFT_SPACING_KEY),
            left_bubble_flipped: env.get(LEFT_BUBBLE_FLIPPED_KEY),
            right_bubble_flipped: env.get(RIGHT_BUBBLE_FLIPPED_KEY),
            content_font_size: env.get(CONTENT_FONT_SIZE_KEY),
            sender_font_size: env.get(SENDER_FONT_SIZE_KEY),
            datetime_font_size: env.get(DATETIME_FONT_SIZE_KEY),
            metadata_font_bolded: env.get(HEADER_FONT_BOLDED_KEY),
            compact_datetime: env.get(COMPACT_DATETIME_KEY),
            sender_color: SimpleColor { r: sender_color.0, g: sender_color.1, b: sender_color.2 },
            datetime_color: SimpleColor { r: datetime_color.0, g: datetime_color.1, b: datetime_color.2 },
        }
    }

    fn set_env(&self, env: &mut druid::Env) {
        env.set(ITEM_LAYOUT_KEY, self.item_layout as u64);
        env.set(PICTURE_SHAPE_KEY, self.picture_shape as u64);
        env.set(PICTURE_SIZE_KEY, self.picture_size as f64);
        env.set(CHAT_BUBBLE_TAIL_SHAPE_KEY, self.chat_bubble_tail_shape as u64);
        env.set(CHAT_BUBBLE_TAIL_SIZE_KEY, self.chat_bubble_tail_size as f64);
        env.set(CHAT_BUBBLE_RADIUS_KEY, self.chat_bubble_radius as f64);
        env.set(CHAT_BUBBLE_IMG_SPACING_KEY, self.chat_bubble_picture_spacing as f64);
        env.set(SHOW_SELF_PROFILE_PIC_KEY, self.show_self_pic);
        env.set(BUBBLE_PADDING_KEY, self.bubble_padding as f64);
        env.set(METADATA_CONTENT_SPACING_KEY, self.metadata_content_spacing as f64);
        env.set(ALIGN_TO_PICTURE, self.align_to_picture as bool);
        env.set(GROUP_SPACING_KEY, self.group_spacing as f64);
        env.set(SINGLE_MESSAGE_SPACING_KEY, self.single_message_spacing as f64);
        env.set(SHOW_LEFT_LINE_KEY, self.show_left_line as bool);
        env.set(LEFT_SPACING_KEY, self.left_spacing as f64);
        env.set(LEFT_BUBBLE_FLIPPED_KEY, self.left_bubble_flipped as bool);
        env.set(RIGHT_BUBBLE_FLIPPED_KEY, self.right_bubble_flipped as bool);
        env.set(CONTENT_FONT_SIZE_KEY, self.content_font_size as f64);
        env.set(SENDER_FONT_SIZE_KEY, self.sender_font_size as f64);
        env.set(DATETIME_FONT_SIZE_KEY, self.datetime_font_size as f64);
        env.set(HEADER_FONT_BOLDED_KEY, self.metadata_font_bolded as bool);
        env.set(COMPACT_DATETIME_KEY, self.compact_datetime as bool);
        env.set(SENDER_COLOR_KEY, self.sender_color.to_druid_color());
        env.set(DATETIME_COLOR_KEY, self.datetime_color.to_druid_color());
    }
}

#[derive(Clone, druid::Data, druid::Lens)]
struct SimpleColor {
    r: u8,
    g: u8,
    b: u8,
}

impl SimpleColor {
    fn to_druid_color(&self) -> druid::Color {
        druid::Color::rgb8(self.r, self.g, self.b)
    }
}

#[derive(Clone, druid::Data, druid::Lens)]
struct MessageGroup {
    user_id: u32,
    profile_pic: ImageBuf,
    messages: im::Vector<Message>,
}

#[derive(Clone, druid::Data)]
struct Message {
    message: String,
    position_in_group: u32,
    timestamp_epoch_seconds: i64,
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
        data.settings_open = false;
        if self.window_count <= 0 {
            println!("All windows closed. Quitting...");
            druid::Application::global().quit();
        }
    }
}

fn on_settings_icon_click(ctx: &mut EventCtx, state: &mut AppState, _env: &druid::Env) {
    println!("Settings click");

    if state.settings_open {
        println!("Settings already open. Ignoring.");
    } else {
        state.settings_open = true; // Prevent it from being opened a second time
        let settings_size = druid::Size::new(1400.0, 530.0);
        let mut new_win = WindowDesc::new(build_settings_ui()).resizable(false);
        new_win = new_win.window_size(settings_size);
        ctx.new_window(new_win);
    }
}

fn on_send_icon_click(_ctx: &mut EventCtx, state: &mut AppState, env: &druid::Env) {
    println!("Send click");

    // Find which user is self
    let self_id = env.get(SELF_USER_ID_KEY);

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

fn ui_changed_callback(ctx: &mut EventCtx) {
    // Signal to all timeline widgets to refresh
    ctx.submit_command(REFRESH_UI_SELECTOR.to(druid::Target::Global));
}

fn predefined_layout_selected(ctx: &mut EventCtx, layout: PredefiendLayout, settings: &mut LayoutSettings) {
    match layout {
        PredefiendLayout::ModernHangouts => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 32.0;
            settings.chat_bubble_tail_shape = TailShape::ConcaveBottom;
            settings.chat_bubble_tail_size = 6.0;
            settings.chat_bubble_radius = 4.0;
            settings.chat_bubble_picture_spacing = 3.5;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 1.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 6.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::ModernBubble => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 32.0;
            settings.chat_bubble_tail_shape = TailShape::ConcaveBottom;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 10.0;
            settings.chat_bubble_picture_spacing = 6.5;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 7.0;
            settings.group_spacing = 10.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::LargeBubble => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 40.0;
            settings.chat_bubble_tail_shape = TailShape::ConcaveBottom;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 8.0;
            settings.chat_bubble_picture_spacing = 6.5;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 7.0;
            settings.group_spacing = 10.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 14.0;
            settings.sender_font_size = 12.0;
            settings.datetime_font_size = 12.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::OldHangouts => {
            settings.item_layout = ItemLayoutOption::BubbleInternalBottomMeta;
            settings.picture_shape = PictureShape::Rectangle;
            settings.picture_size = 35.0;
            settings.chat_bubble_tail_shape = TailShape::Straight;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 0.5;
            settings.chat_bubble_picture_spacing = 0.5;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 3.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 9.5;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 10.0;
            settings.datetime_font_size = 10.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 200, g: 200, b: 200 };
            settings.datetime_color = SimpleColor { r: 200, g: 200, b: 200 };
        },
        PredefiendLayout::IMessage => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 32.0;
            settings.chat_bubble_tail_shape = TailShape::Fancy;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 10.0;
            settings.chat_bubble_picture_spacing = 6.5;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 7.0;
            settings.group_spacing = 10.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = true;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::Telegram => {
            settings.item_layout = ItemLayoutOption::BubbleInternalTopMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 32.0;
            settings.chat_bubble_tail_shape = TailShape::ConcaveBottom;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 4.0;
            settings.chat_bubble_picture_spacing = 8.0;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 5.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 9.5;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = true;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 12.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::OldKik => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 30.0;
            settings.chat_bubble_tail_shape = TailShape::Symmetric;
            settings.chat_bubble_tail_size = 5.5;
            settings.chat_bubble_radius = 4.0;
            settings.chat_bubble_picture_spacing = 7.0;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 1.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 6.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::TearDrop => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 25.0;
            settings.chat_bubble_tail_shape = TailShape::Square;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 12.0;
            settings.chat_bubble_picture_spacing = 3.5;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 7.5;
            settings.group_spacing = 10.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::Tailless => {
            settings.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 25.0;
            settings.chat_bubble_tail_shape = TailShape::Hidden;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 8.0;
            settings.chat_bubble_picture_spacing = 3.5;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 10.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 11.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::OtherBubble => {
            settings.item_layout = ItemLayoutOption::BubbleInternalTopMeta;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 28.0;
            settings.chat_bubble_tail_shape = TailShape::ConcaveBottom;
            settings.chat_bubble_tail_size = 7.0;
            settings.chat_bubble_radius = 3.0;
            settings.chat_bubble_picture_spacing = 6.0;
            settings.show_self_pic = false;
            settings.metadata_content_spacing = 5.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 5.0;
            settings.group_spacing = 9.5;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.left_bubble_flipped = false;
            settings.right_bubble_flipped = true;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 14.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::Discord => {
            settings.item_layout = ItemLayoutOption::Bubbleless;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 40.0;
            settings.chat_bubble_picture_spacing = 13.0;
            settings.metadata_content_spacing = 7.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 0.0;
            settings.group_spacing = 23.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 14.0;
            settings.sender_font_size = 14.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = false;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::CompactDiscord => {
            settings.item_layout = ItemLayoutOption::Bubbleless;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 36.0;
            settings.chat_bubble_picture_spacing = 8.0;
            settings.metadata_content_spacing = 7.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 0.0;
            settings.group_spacing = 13.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = false;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::Slack => {
            settings.item_layout = ItemLayoutOption::Bubbleless;
            settings.picture_shape = PictureShape::RoundedRectangle;
            settings.picture_size = 36.0;
            settings.chat_bubble_picture_spacing = 5.5;
            settings.metadata_content_spacing = 5.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 0.0;
            settings.group_spacing = 14.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = false;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::Compact => {
            settings.item_layout = ItemLayoutOption::Bubbleless;
            settings.picture_shape = PictureShape::Circle;
            settings.picture_size = 25.0;
            settings.chat_bubble_picture_spacing = 2.5;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 2.0;
            settings.align_to_picture = true;
            settings.bubble_padding = 0.0;
            settings.group_spacing = 7.0;
            settings.single_message_spacing = 5.0;
            settings.show_left_line = false;
            settings.left_spacing = 0.0;
            settings.metadata_font_bolded = true;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::IRC => {
            settings.item_layout = ItemLayoutOption::IRCStyle;
            settings.picture_shape = PictureShape::Rectangle;
            settings.picture_size = 16.0;
            settings.chat_bubble_picture_spacing = 3.5;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 3.0;
            settings.align_to_picture = false;
            settings.group_spacing = 6.0;
            settings.single_message_spacing = 5.0;
            settings.bubble_padding = 6.0;
            settings.show_left_line = true;
            settings.left_spacing = 4.0;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::LargeIRC => {
            settings.item_layout = ItemLayoutOption::IRCStyle;
            settings.picture_shape = PictureShape::Rectangle;
            settings.picture_size = 18.0;
            settings.chat_bubble_picture_spacing = 4.0;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 3.0;
            settings.align_to_picture = false;
            settings.group_spacing = 7.0;
            settings.single_message_spacing = 6.0;
            settings.bubble_padding = 6.0;
            settings.show_left_line = true;
            settings.left_spacing = 5.0;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 14.0;
            settings.sender_font_size = 14.0;
            settings.datetime_font_size = 12.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
        PredefiendLayout::SpacedIRC => {
            settings.item_layout = ItemLayoutOption::IRCStyle;
            settings.picture_shape = PictureShape::Rectangle;
            settings.picture_size = 16.0;
            settings.chat_bubble_picture_spacing = 3.5;
            settings.show_self_pic = true;
            settings.metadata_content_spacing = 6.0;
            settings.align_to_picture = false;
            settings.group_spacing = 12.0;
            settings.single_message_spacing = 5.0;
            settings.bubble_padding = 6.0;
            settings.show_left_line = true;
            settings.left_spacing = 4.5;
            settings.metadata_font_bolded = false;
            settings.content_font_size = 13.0;
            settings.sender_font_size = 13.0;
            settings.datetime_font_size = 11.0;
            settings.compact_datetime = true;
            settings.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
            settings.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
        },
    }
    ui_changed_callback(ctx);
}

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
            timeline_item_widget::TimelineItemWidget::new()
        })
        .with_spacing(GROUP_SPACING_KEY)
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

#[derive(Clone, Copy, PartialEq, druid::Data)]
pub enum PredefiendLayout {
    ModernHangouts,
    ModernBubble,
    LargeBubble,
    OldHangouts,
    Telegram,
    IMessage,
    OldKik,
    TearDrop,
    Tailless,
    OtherBubble,
    Slack,
    Discord,
    CompactDiscord,
    Compact,
    IRC,
    LargeIRC,
    SpacedIRC,
}


const IMG_SHAPE_OPTIONS: [(&str, PictureShape); 5] =
[
    ("Circle", PictureShape::Circle),
    ("Rectangle", PictureShape::Rectangle),
    ("Rounded Rectangle", PictureShape::RoundedRectangle),
    ("Hexagon", PictureShape::Hexagon),
    ("Octagon", PictureShape::Octagon),
];
const TAIL_SHAPE_OPTIONS: [(&str, TailShape); 6] =
[
    ("Concave Bottom", TailShape::ConcaveBottom),
    ("Straight", TailShape::Straight),
    ("Fancy", TailShape::Fancy),
    ("Square", TailShape::Square),
    ("Symmetric", TailShape::Symmetric),
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
    widget::Tabs::new()
        .with_tab("Layouts", build_predefined_styles_settings().lens(AppState::layout_settings))
        .with_tab("Advanced", build_advanced_settings().lens(AppState::layout_settings))
}

fn build_predefined_styles_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Label::new("Predefined Layouts")
                .with_text_size(20.0).padding(8.0).align_left()
        )
        .with_default_spacer()
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Layout").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Flex::column()
                        .with_child(
                            widget::Button::new("Modern Hangouts")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::ModernHangouts, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Modern Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::ModernBubble, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Large Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::LargeBubble, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Old Fashioned Hangouts")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::OldHangouts, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Telegram")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::Telegram, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("iMessage")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::IMessage, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Old Kik")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::OldKik, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Tear Drop")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::TearDrop, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("No Tail")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::Tailless, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Other Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::OtherBubble, data);
                                })
                        )
                        .cross_axis_alignment(widget::CrossAxisAlignment::Fill)
                    , 1.3)
                .with_flex_child(
                    widget::Flex::column()
                        .with_child(
                            widget::Button::new("Discord")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::Discord, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Compact Discord")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::CompactDiscord, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Slack")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::Slack, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Compact")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::Compact, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Modern IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::IRC, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Large IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::LargeIRC, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Spaced Modern IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefiendLayout::SpacedIRC, data);
                                })
                        )
                        .cross_axis_alignment(widget::CrossAxisAlignment::Fill)
                    , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_flex_child(widget::Label::new("The standard IRC layout changes when width > 400"), 1.0)

}

fn build_advanced_layout_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Item Layout:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(LAYOUT_OPTIONS)
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::item_layout)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
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
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::picture_shape)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Size:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(10.0, 100.0).with_step(1.0)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::picture_size)
                , 1.0)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:1}", data.picture_size)}),
                    0.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Compact Datetime:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::compact_datetime)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
}

fn build_advanced_bubble_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Bubble Tail Shape:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(TAIL_SHAPE_OPTIONS)
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::chat_bubble_tail_shape)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Bubble Radius:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 12.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::chat_bubble_radius)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_radius)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Tail Size:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(2.0, 12.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::chat_bubble_tail_size)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_tail_size)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Flip left bubble:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::left_bubble_flipped)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Flip right bubble:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::right_bubble_flipped)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Bubble Padding:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 15.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::bubble_padding)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.bubble_padding)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(15.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Show Self Profile Pic:")
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::show_self_pic)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .disabled_if(|data, _| data.item_layout != ItemLayoutOption::BubbleExternBottomMeta
            && data.item_layout != ItemLayoutOption::BubbleInternalBottomMeta
            && data.item_layout != ItemLayoutOption::BubbleInternalTopMeta
        )

}

fn build_advanced_sizing_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Profile Pic Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(-15.0, 20.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::chat_bubble_picture_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_bubble_picture_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Content Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 10.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::metadata_content_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.metadata_content_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Msg Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 28.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::single_message_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.single_message_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Group Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 28.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::group_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.group_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Left Spacing:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 10.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::left_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.left_spacing)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Show Left Line:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::show_left_line)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Align to Pic:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Switch::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::align_to_picture)
            , 1.3)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
            .disabled_if(|data, _| data.item_layout == ItemLayoutOption::BubbleExternBottomMeta
                || data.item_layout == ItemLayoutOption::BubbleInternalBottomMeta
                || data.item_layout == ItemLayoutOption::BubbleInternalTopMeta
            )
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Content Font Size:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Stepper::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::content_font_size)
            , 0.9)
            .with_flex_child(
                widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.content_font_size)})
            , 0.4)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Sender Font Size:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Stepper::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::sender_font_size)
            , 0.9)
            .with_flex_child(
                widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.sender_font_size)})
            , 0.4)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(widget::Flex::row()
            .with_flex_child(widget::Label::new("Datetime Font Size:").align_right()
            , 0.7)
            .with_default_spacer()
            .with_flex_child(
                widget::Stepper::new()
                .on_click( |ctx: &mut EventCtx, _, _ | {
                    ui_changed_callback(ctx);
                })
                .lens(LayoutSettings::datetime_font_size)
            , 0.9)
            .with_flex_child(
                widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.datetime_font_size)})
            , 0.4)
            .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
}


fn build_advanced_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Flex::row()
                .with_child(
                    widget::Label::new("Layout Settings")
                        .with_text_size(20.0).padding(8.0).align_left()
                )
                .with_child(
                    widget::Button::new("Refresh Window")
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                )
        )
        .with_default_spacer()
        .with_child(
            widget::Flex::row()
                .with_flex_child(build_advanced_layout_settings(), 1.5)
                .with_flex_child(build_advanced_bubble_settings(), 1.0)
                .with_flex_child(build_advanced_sizing_settings(), 1.0)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
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

const NOUNS: &'static [&str] = &[
    "time", "person", "year", "way", "day", "thing", "man", "world", "life",
    "hand", "part", "child", "eye", "woman", "place", "work", "week", "case",
    "point",  "government", "company", "number", "group", "problem", "fact"
];
const VERBS: &'static [&str] = &[
    "be", "have", "do", "say", "get", "make", "go", "know", "take", "see",
    "come", "think", "look", "want", "give", "use", "find", "tell", "ask",
    "work", "seem", "feel", "try", "leave", "call"
];
const ADJECTIVES: &'static [&str] = &[
    "good", "new", "first", "long", "great", "little", "own", "other", "old",
    "right", "big", "high", "different", "small", "large", "next", "early",
    "important", "few", "public", "bad", "same", "able",
];
const PREPOSITIONS: &'static [&str] = &[
    "to", "of", "in", "for", "on", "with", "at", "by", "from", "up", "about",
    "info", "over", "after",
];
const OTHERS: &'static [&str] = &[
    "the", "and", "a", "that", "I", "it", "not", "he", "as", "you", "this",
    "but", "his", "they", "her", "she", "or", "an", "will", "my", "one",
    "all", "would", "there", "their",
];
const ALL_WORD_LISTS: &'static [&'static [&str]] = &[
    NOUNS, VERBS, ADJECTIVES, PREPOSITIONS, OTHERS,
];
const SINGLE_WORDS: &'static [&str] = &[
    "Nice", "Thanks", "Okay", "ok", "Hey", "lol",
];

fn uppercase_first_letter(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn generate_random_message(rng: &mut ThreadRng, capital_probability: f64) -> String {
    let mut msg_len = rng.gen_range(1..13);
    if rng.gen_bool(0.1) {
        msg_len *= 2;
    } else if rng.gen_bool(0.1) {
        msg_len *= 4;
    }

    let mut message = String::new();

    if msg_len <= 3 {
        if rng.gen_bool(0.5) {
            if rng.gen_bool(0.5) {
                message.push_str("Hi ");
            } else {
                message.push_str("Hello ");
            }
            if msg_len == 3 {
                message.push_str(ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())]);
                message.push(' ');
            }
            if msg_len > 1 {
                message.push_str(NOUNS[rng.gen_range(0..NOUNS.len())]);
                message.push(' ');
            }
            return message;
        } else if msg_len == 1 {
            message.push_str(SINGLE_WORDS[rng.gen_range(0..SINGLE_WORDS.len())]);
            return message;
        }
    }

    for _i in 0..msg_len {
        let word_list = ALL_WORD_LISTS[rng.gen_range(0..ALL_WORD_LISTS.len())];
        message += word_list[rng.gen_range(0..word_list.len())];
        message.push(' ');
    }

    if rng.gen_bool(0.4) {
        let str = message[0..message.len() - 1].to_string();
        message = str;
        message.push('.');
    }

    if rng.gen_bool(capital_probability) {
        uppercase_first_letter(message)
    } else {
        message
    }
}

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let mut initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![],
        profile_pics: im::vector![],
        settings_open: false,
        layout_settings: LayoutSettings {
            item_layout: ItemLayoutOption::BubbleExternBottomMeta,
            picture_shape: PictureShape::Circle,
            picture_size: 32.0,
            chat_bubble_tail_shape: TailShape::ConcaveBottom,
            chat_bubble_tail_size: 7.0,
            chat_bubble_radius: 4.0,
            chat_bubble_picture_spacing: 3.5,
            align_to_picture: true,
            show_self_pic: true,
            bubble_padding: 5.0,
            metadata_content_spacing: 1.0,
            group_spacing: 6.0,
            single_message_spacing: 5.0,
            show_left_line: false,
            left_spacing: 0.0,
            left_bubble_flipped: false,
            right_bubble_flipped: true,
            content_font_size: 13.0,
            sender_font_size: 11.0,
            datetime_font_size: 11.0,
            metadata_font_bolded: false,
            compact_datetime: true,
            sender_color: SimpleColor { r: 175, g: 175, b: 175 },
            datetime_color: SimpleColor { r: 175, g: 175, b: 175 },
        }
    };

    let mut rng = rand::thread_rng();

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
        let user_id = rng.gen_range(0..5);
        let group_size = rng.gen_range(1..5);
        let mut messages = im::vector![];
        for i in 0..group_size {
            messages.push_back(Message {
                message: generate_random_message(&mut rng, if i == 0 {0.95} else {0.7} ),
                position_in_group: i,
                timestamp_epoch_seconds: time + i as i64,
            })
        }
        let msg_group = MessageGroup {
            messages: messages,
            user_id: user_id,
            profile_pic: initial_state.profile_pics[user_id as usize].clone(),
        };
        initial_state.timeline_data.push_front(msg_group);
        time -= offset_amount;
        offset_amount *= 2;
    }
    let user_id = rng.gen_range(0..5);
    let msg = MessageGroup {
        messages: im::vector![
            Message {
                timestamp_epoch_seconds: time,
                position_in_group: 0,
                message: "This\nis\na\nnarrow\nbut\nlong\nmessage.\nHopefully\nthe\nbubble\nstays\nnarrow.".to_string(),
            },
        ],
        user_id: user_id,
        profile_pic: initial_state.profile_pics[user_id as usize].clone(),
    };
    initial_state.timeline_data.push_front(msg);
    let user_id = rng.gen_range(0..5);
    let msg = MessageGroup {
        messages: im::vector![
                Message {
                    timestamp_epoch_seconds: time,
                    position_in_group: 0,
                    message: "Hi".to_string(),
                },
        ],
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