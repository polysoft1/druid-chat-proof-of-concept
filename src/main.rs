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
    timeline_data: im::Vector<u32>,
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
            widget::Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
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
    let initial_state = AppState {
        text_edit: "".to_string().into(),
        timeline_data: im::vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    };
    AppLauncher::with_window(
        WindowDesc::new(
            build_ui()
        ).window_size((300.0, 450.0))
    ).launch(
        initial_state
    )?;
    Ok(())
}