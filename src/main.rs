use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt};
use druid::widget;
use druid;
use std::sync;
use tracing::error;

#[derive(Clone, druid::Data, druid::Lens)]
struct AppState {
    text_edit: sync::Arc<String>,
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

    widget::Flex::column()
        // Title
        .with_child(
            widget::Label::new("Chat Title")
                .with_line_break_mode(widget::LineBreaking::WordWrap)
                .padding(5.0)
                .background(druid::theme::BACKGROUND_LIGHT)
                .expand_width()
        )
        // The timeline itself
        .with_flex_spacer(1.0)
        // The bottom editor
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::TextBox::multiline()
                        .with_placeholder("Message...")
                        .lens(AppState::text_edit)
                        .padding(1.0)
                        .expand_width(),
                1.0)
                .with_child(
                    widget::Svg::new(send_svg).fix_height(25.0).padding(5.0)
                )
        )
    }

fn main() -> Result<(), PlatformError> {
    // create the initial app state
    let initial_state = AppState {
        text_edit: "".to_string().into(),
    };
    AppLauncher::with_window(
        WindowDesc::new(
            build_ui()
        )
    ).launch(
        initial_state
    )?;
    Ok(())
}