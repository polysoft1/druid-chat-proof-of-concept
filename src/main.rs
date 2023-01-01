use druid::{AppLauncher, WindowDesc, PlatformError, ImageBuf, AppDelegate};
use druid::im;
use druid;
use rand::rngs::ThreadRng;
use std::sync;
use rand::Rng;
use std::path::Path;
use std::env;

use helper::layout_settings::LayoutSettings;

mod widgets;
mod helper;
mod settings_ui;
mod chat_ui;

// Env keys to define layout in the environment
pub const ITEM_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.item_layout");
pub const METADATA_LAYOUT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.metadata_layout");
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
pub const DATETIME_FORMAT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.datetime_format");
pub const SIDE_TIME_FORMAT_KEY: druid::env::Key<u64> = druid::env::Key::new("polysoft.druid-demo.side_time_format");
pub const SENDER_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.sender_color");
pub const DATETIME_COLOR_KEY: druid::env::Key<druid::Color> = druid::env::Key::new("polysoft.druid-demo.datetime_color");
pub const LEFT_META_OFFSET: druid::env::Key<f64> = druid::env::Key::new("polysoft.druid-demo.left_meta_offset");
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

fn get_chat_window_desc() -> WindowDesc<AppState> {
    let main_window = WindowDesc::new(
        chat_ui::build_chat_ui()
    ).window_size((300.0, 450.0));
    return main_window;
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
        layout_settings: LayoutSettings::default(),
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