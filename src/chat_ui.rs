use druid::Widget;
use druid::widget;
use crate::AppState;
use crate::widgets::chat_window_widget;

pub(crate) fn build_chat_ui() -> impl Widget<AppState> {
    let chat_widget = chat_window_widget::ChatWindowWidget::new();
    widget::EnvScope::new(
        |env: &mut druid::env::Env, data: &AppState| {
            data.layout_settings.set_env(env);
        },
        chat_widget
    )
}
