use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt};
use druid::widget;
use druid::im;
use druid;
use druid::piet::Color;
use std::sync;
use tracing::error;

#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
    timeline_data: im::Vector<String>,
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
        widget::List::new( || {
            widget::Label::new(|item: &String, _env: &_| item.clone())
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .padding(10.0)
                .background(Color::rgb8(75, 75, 76))
                .rounded(10.0)
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
        timeline_data: im::vector!["test".to_string()],
    };

    let mut msg_body = String::new();
    msg_body.push_str("Start of msg.");

    for _ in 1..40 {
        msg_body.push_str(" Appended!");
        initial_state.timeline_data.push_back(msg_body.clone());
    }
    initial_state.timeline_data.push_back("This\nis\na\nnarrow\nbut\nlong\nmessage.\nHopefully\nthe\nbubble\nstays\nnarrow.".to_string());

    AppLauncher::with_window(
        WindowDesc::new(
            build_ui()
        ).window_size((300.0, 450.0))
    ).launch(
        initial_state
    )?;
    Ok(())
}