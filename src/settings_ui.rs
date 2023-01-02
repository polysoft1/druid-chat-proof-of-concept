use crate::helper::helper_functions::TimestampFormat;
use crate::widgets::timeline_item_widget::{PictureShape, TailShape, ItemLayoutOption, MetadataLayout};
use druid::{EventCtx, Widget, WidgetExt};
use druid::widget;
use crate::helper::layout_settings::{LayoutSettings, PredefinedLayout};
use crate::AppState;


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
const METADATA_LAYOUT_OPTIONS: [(&str, MetadataLayout); 3] =
[
    ("Side By Side", MetadataLayout::LeftSideBySide),
    ("With Dot", MetadataLayout::LeftSideBySideWithDot),
    ("Spaced", MetadataLayout::LeftRightSpaced),
];
const DATETIME_OPTIONS: [(&str, TimestampFormat); 4] =
[
    ("Full 12hr", TimestampFormat::Full12),
    ("Full 24hr", TimestampFormat::Full24),
    ("Compact 12hr", TimestampFormat::Compact12),
    ("Compact 24hr", TimestampFormat::Compact24),
];

const TIME_OPTIONS: [(&str, TimestampFormat); 3] =
[
    ("12hr", TimestampFormat::TimeOnly12),
    ("24hr", TimestampFormat::TimeOnly24),
    ("12hr am/pm", TimestampFormat::TimeOnlyAmPm),
];

pub(crate) fn build_settings_ui() -> impl Widget<AppState> {
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
                                    predefined_layout_selected(ctx, PredefinedLayout::ModernHangouts, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Modern Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::ModernBubble, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Large Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::LargeBubble, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Old Fashioned Hangouts")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::OldHangouts, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Telegram")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Telegram, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("iMessage")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::IMessage, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Old Kik")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::OldKik, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Tear Drop")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::TearDrop, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("No Tail")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Tailless, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Relaxed")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Relaxed, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Other Bubble")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::OtherBubble, data);
                                })
                        )
                        .cross_axis_alignment(widget::CrossAxisAlignment::Fill)
                    , 1.3)
                .with_flex_child(
                    widget::Flex::column()
                        .with_child(
                            widget::Button::new("Discord")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Discord, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Compact Discord")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::CompactDiscord, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Slack")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Slack, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Compact")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::Compact, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Modern IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::IRC, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Large IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::LargeIRC, data);
                                })
                        )
                        .with_child(
                            widget::Button::new("Spaced Modern IRC")
                                .on_click( |ctx: &mut EventCtx, data: &mut LayoutSettings, _ | {
                                    predefined_layout_selected(ctx, PredefinedLayout::SpacedIRC, data);
                                })
                        )
                        .cross_axis_alignment(widget::CrossAxisAlignment::Fill)
                    , 1.3)
                .with_flex_child(
                    widget::Flex::column()
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
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Metadata Layout:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(METADATA_LAYOUT_OPTIONS)
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::metadata_layout)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
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
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Time Format:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(DATETIME_OPTIONS)
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::datetime_format)
                , 1.3)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(
                    widget::Label::new("Side Time Format:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::RadioGroup::column(TIME_OPTIONS)
                        .on_click( |ctx: &mut EventCtx, _, _ | {
                            ui_changed_callback(ctx);
                        })
                        .lens(LayoutSettings::side_time_format)
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

fn build_advanced_irc_settings() -> impl Widget<LayoutSettings> {
    widget::Flex::column()
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Stack width:").align_right()
                    , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(100.0, 1000.0).with_step(5.0)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::irc_stack_width)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.irc_stack_width)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .with_spacer(10.0)
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Header Width:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(20.0, 250.0).with_step(1.0)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::irc_header_width)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.irc_header_width)}),
                    0.4)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        .disabled_if(|data, _| data.item_layout != ItemLayoutOption::IRCStyle)
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
                    .lens(LayoutSettings::chat_picture_spacing)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.chat_picture_spacing)}),
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
        .with_child(
            widget::Flex::row()
                .with_flex_child(widget::Label::new("Left Meta Offset:").align_right()
                , 0.7)
                .with_default_spacer()
                .with_flex_child(
                    widget::Slider::new().with_range(0.0, 30.0).with_step(0.5)
                    .on_click( |ctx: &mut EventCtx, _, _ | {
                        ui_changed_callback(ctx);
                    })
                    .lens(LayoutSettings::left_meta_offset)
                , 0.9)
                .with_flex_child(widget::Label::new(
                    |data: &LayoutSettings, _: &_| {format!("{:.1}", data.left_meta_offset)}),
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
                .with_flex_child(
                    widget::Flex::column()
                        .with_child(widget::Label::new("Bubble-specific"))
                        .with_spacer(20.0)
                        .with_child(build_advanced_bubble_settings())
                        .with_spacer(30.0)
                        .with_child(widget::Label::new("IRC-specific"))
                        .with_spacer(20.0)
                        .with_child(build_advanced_irc_settings())
                        .cross_axis_alignment(widget::CrossAxisAlignment::Start)
                , 1.0)
                .with_flex_child(build_advanced_sizing_settings(), 1.0)
                .cross_axis_alignment(widget::CrossAxisAlignment::Start)
        )
        
}

fn ui_changed_callback(ctx: &mut EventCtx) {
    // Signal to all timeline widgets to refresh
    ctx.submit_command(crate::REFRESH_UI_SELECTOR.to(druid::Target::Global));
}

fn predefined_layout_selected(ctx: &mut EventCtx, layout: PredefinedLayout, settings: &mut LayoutSettings) {
    settings.set_from_predefined_layout(layout);
    ui_changed_callback(ctx);
}