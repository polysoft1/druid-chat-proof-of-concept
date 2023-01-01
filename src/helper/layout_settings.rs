use druid;
use druid::{BoxConstraints, Size, Point};
use super::helper_functions;
use crate::widgets::timeline_item_widget::{PictureShape, TailShape, ItemLayoutOption, };

const IRC_STACK_WIDTH: f64 = 400.0; // How wide should be required for it to no longer be stacked.
const IRC_HEADER_WIDTH: f64 = 160.0; // How far should we push the text right to make it so they don't end up staggered.

#[derive(Clone, druid::Data, druid::Lens)]
pub struct SimpleColor {
    r: u8,
    g: u8,
    b: u8,
}

impl SimpleColor {
    pub fn to_druid_color(&self) -> druid::Color {
        druid::Color::rgb8(self.r, self.g, self.b)
    }
}

#[derive(Clone, druid::Data, druid::Lens)]
pub struct LayoutSettings {
    pub item_layout: ItemLayoutOption,
    pub picture_shape: PictureShape,
    pub picture_size: f64,
    pub chat_bubble_tail_shape: TailShape,
    pub chat_bubble_tail_size: f64,
    pub chat_bubble_radius: f64,
    pub chat_bubble_picture_spacing: f64,
    pub show_self_pic: bool,
    pub bubble_padding: f64,
    pub metadata_content_spacing: f64,
    pub align_to_picture: bool,
    pub group_spacing: f64,
    pub single_message_spacing: f64,
    pub show_left_line: bool,
    pub left_spacing: f64,
    pub left_bubble_flipped: bool,
    pub right_bubble_flipped: bool,
    pub content_font_size: f64,
    pub sender_font_size: f64,
    pub datetime_font_size: f64,
    pub metadata_font_bolded: bool,
    pub relative_datetime: bool,
    pub show_old_times_datetime: bool,
    pub sender_color: SimpleColor,
    pub datetime_color: SimpleColor,
    pub left_meta_offset: f64,
}


#[derive(Clone, Copy, PartialEq, druid::Data)]
pub enum PredefinedLayout {
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

impl LayoutSettings {
    pub fn default() -> LayoutSettings {
        LayoutSettings {
            item_layout: ItemLayoutOption::BubbleExternBottomMeta,
            picture_shape: PictureShape::Circle,
            picture_size: 32.0,
            chat_bubble_tail_shape: TailShape::ConcaveBottom,
            chat_bubble_tail_size: 6.0,
            chat_bubble_radius: 4.0,
            chat_bubble_picture_spacing: 3.5,
            show_self_pic: false,
            metadata_content_spacing: 1.0,
            align_to_picture: true,
            bubble_padding: 5.0,
            group_spacing: 6.0,
            single_message_spacing: 5.0,
            show_left_line: false,
            left_spacing: 0.0,
            left_bubble_flipped: false,
            right_bubble_flipped: true,
            metadata_font_bolded: false,
            content_font_size: 13.0,
            sender_font_size: 11.0,
            datetime_font_size: 11.0,
            relative_datetime: false,
            show_old_times_datetime: false,
            left_meta_offset: 2.0,
            sender_color: SimpleColor { r: 175, g: 175, b: 175 },
            datetime_color: SimpleColor { r: 175, g: 175, b: 175 },
        }
    }

    pub fn from_env(env: &druid::Env) -> LayoutSettings{
        let sender_color = env.get(crate::SENDER_COLOR_KEY).as_rgba8();
        let datetime_color = env.get(crate::DATETIME_COLOR_KEY).as_rgba8();
        LayoutSettings {
            item_layout: num_traits::FromPrimitive::from_u64(env.get(crate::ITEM_LAYOUT_KEY)).expect("Invalid layout index"),
            picture_shape: num_traits::FromPrimitive::from_u64(env.get(crate::PICTURE_SHAPE_KEY)).expect("Invalid layout index"),
            picture_size: env.get(crate::PICTURE_SIZE_KEY),
            chat_bubble_tail_shape: num_traits::FromPrimitive::from_u64(env.get(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY)).expect("Invalid layout index"),
            chat_bubble_tail_size: env.get(crate::CHAT_BUBBLE_TAIL_SIZE_KEY),
            chat_bubble_radius: env.get(crate::CHAT_BUBBLE_RADIUS_KEY),
            chat_bubble_picture_spacing: env.get(crate::CHAT_BUBBLE_IMG_SPACING_KEY),
            show_self_pic: env.get(crate::SHOW_SELF_PROFILE_PIC_KEY),
            bubble_padding: env.get(crate::BUBBLE_PADDING_KEY),
            metadata_content_spacing: env.get(crate::METADATA_CONTENT_SPACING_KEY),
            align_to_picture: env.get(crate::ALIGN_TO_PICTURE),
            group_spacing: env.get(crate::GROUP_SPACING_KEY),
            single_message_spacing: env.get(crate::SINGLE_MESSAGE_SPACING_KEY),
            show_left_line: env.get(crate::SHOW_LEFT_LINE_KEY),
            left_spacing: env.get(crate::LEFT_SPACING_KEY),
            left_bubble_flipped: env.get(crate::LEFT_BUBBLE_FLIPPED_KEY),
            right_bubble_flipped: env.get(crate::RIGHT_BUBBLE_FLIPPED_KEY),
            content_font_size: env.get(crate::CONTENT_FONT_SIZE_KEY),
            sender_font_size: env.get(crate::SENDER_FONT_SIZE_KEY),
            datetime_font_size: env.get(crate::DATETIME_FONT_SIZE_KEY),
            metadata_font_bolded: env.get(crate::HEADER_FONT_BOLDED_KEY),
            relative_datetime: env.get(crate::RELATIVE_DATETIME_KEY),
            show_old_times_datetime: env.get(crate::SHOW_OLD_DATETIME_KEY),
            left_meta_offset: env.get(crate::LEFT_META_OFFSET),
            sender_color: SimpleColor { r: sender_color.0, g: sender_color.1, b: sender_color.2 },
            datetime_color: SimpleColor { r: datetime_color.0, g: datetime_color.1, b: datetime_color.2 },
        }
    }

    pub fn set_env(&self, env: &mut druid::Env) {
        env.set(crate::ITEM_LAYOUT_KEY, self.item_layout as u64);
        env.set(crate::PICTURE_SHAPE_KEY, self.picture_shape as u64);
        env.set(crate::PICTURE_SIZE_KEY, self.picture_size as f64);
        env.set(crate::CHAT_BUBBLE_TAIL_SHAPE_KEY, self.chat_bubble_tail_shape as u64);
        env.set(crate::CHAT_BUBBLE_TAIL_SIZE_KEY, self.chat_bubble_tail_size as f64);
        env.set(crate::CHAT_BUBBLE_RADIUS_KEY, self.chat_bubble_radius as f64);
        env.set(crate::CHAT_BUBBLE_IMG_SPACING_KEY, self.chat_bubble_picture_spacing as f64);
        env.set(crate::SHOW_SELF_PROFILE_PIC_KEY, self.show_self_pic);
        env.set(crate::BUBBLE_PADDING_KEY, self.bubble_padding as f64);
        env.set(crate::METADATA_CONTENT_SPACING_KEY, self.metadata_content_spacing as f64);
        env.set(crate::ALIGN_TO_PICTURE, self.align_to_picture as bool);
        env.set(crate::GROUP_SPACING_KEY, self.group_spacing as f64);
        env.set(crate::SINGLE_MESSAGE_SPACING_KEY, self.single_message_spacing as f64);
        env.set(crate::SHOW_LEFT_LINE_KEY, self.show_left_line as bool);
        env.set(crate::LEFT_SPACING_KEY, self.left_spacing as f64);
        env.set(crate::LEFT_BUBBLE_FLIPPED_KEY, self.left_bubble_flipped as bool);
        env.set(crate::RIGHT_BUBBLE_FLIPPED_KEY, self.right_bubble_flipped as bool);
        env.set(crate::CONTENT_FONT_SIZE_KEY, self.content_font_size as f64);
        env.set(crate::SENDER_FONT_SIZE_KEY, self.sender_font_size as f64);
        env.set(crate::DATETIME_FONT_SIZE_KEY, self.datetime_font_size as f64);
        env.set(crate::HEADER_FONT_BOLDED_KEY, self.metadata_font_bolded as bool);
        env.set(crate::RELATIVE_DATETIME_KEY, self.relative_datetime as bool);
        env.set(crate::SHOW_OLD_DATETIME_KEY, self.show_old_times_datetime as bool);
        env.set(crate::LEFT_META_OFFSET, self.left_meta_offset);
        env.set(crate::SENDER_COLOR_KEY, self.sender_color.to_druid_color());
        env.set(crate::DATETIME_COLOR_KEY, self.datetime_color.to_druid_color());
    }

    /// Gets the font for the title
    /// 
    /// It is semi-bolded when the settings specify that it should be.
    pub fn get_metadata_font_descriptor(&self) -> druid::FontDescriptor {
        druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI)
            .with_weight(
                if self.metadata_font_bolded {
                    druid::FontWeight::SEMI_BOLD
                } else {
                    druid::FontWeight::REGULAR
                }
            )
    }

    /// It returns true when the layout specifies that it is a bubble
    /// A bubble has padding on all sides of the content, and a background is drawn behind it.
    pub fn is_bubble(&self) -> bool {
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta | ItemLayoutOption::BubbleInternalBottomMeta
                | ItemLayoutOption::BubbleInternalTopMeta => {
                true
            },
            _ => false
        }
    }

    /// A bubble is flipped when the tail and profile picture is on the
    /// bottom instead of the top.
    pub fn is_bubble_flipped(&self, is_self_user: bool) -> bool {
        if is_self_user {
            self.right_bubble_flipped
        } else {
            self.left_bubble_flipped
        }
    }

    /// Gets the proper width of a profile picture
    /// If there is no profile picture given the current situation or setting, it returns 0.0
    pub fn actual_profile_pic_width(&self, is_self_user: bool) -> f64 {
        if self.show_picture(is_self_user) {
            // profile pic is shown always when not self, and when configured when self.
            self.picture_size
        } else {
            // Not shown, so zero.
            0.0
        }
    }

    /// Offset in the case of tiny flipped bubbles with tails, since tiny
    /// messages cause the tail to not align with the picture properly
    pub fn get_top_y_offset(&self, is_self_user: bool, sender_label_size: &Size, msg_label_size: &Size) -> f64 {
        if self.is_bubble() && self.is_bubble_flipped(is_self_user) {
            let mut space_taken = 2.0 * self.bubble_padding + msg_label_size.height;
            if self.item_layout != ItemLayoutOption::BubbleExternBottomMeta {
                space_taken += sender_label_size.height + self.metadata_content_spacing
            };
            let profile_pic_width = self.actual_profile_pic_width(is_self_user);
            if space_taken >= profile_pic_width {
                0.0
            } else {
                profile_pic_width - space_taken
            }
        } else {
            0.0
        }
    }

    /// Gets the width of the profile pic and the space between it and the message or bubble.
    pub fn get_profile_pic_area_width(&self, is_self_user: bool) -> f64 {
        self.actual_profile_pic_width(is_self_user) + self.chat_bubble_picture_spacing
    }

    /// Returns true when the content will not be vertically stacked, but instead horizontally aligned.
    pub fn is_side_by_side(&self, space_available: &BoxConstraints) -> bool {
        self.item_layout == ItemLayoutOption::IRCStyle && space_available.max().width > IRC_STACK_WIDTH
    }

    /// The area that the content can take up.
    /// 
    /// Under most layouts, that's the total width minus the space taken up by
    /// the profile pic and the space between it and the content.
    /// 
    /// In side by side, it's the total width minus the width of the IRC header.
    pub fn get_available_content_width(&self, space_available: &BoxConstraints, is_self_user: bool) -> f64 {
        let mut width: f64 = space_available.max().width;
        width -= if self.is_side_by_side(space_available) {
            IRC_HEADER_WIDTH
        } else {
            self.get_profile_pic_area_width(is_self_user)
        };
        if self.is_bubble() {
            width -= self.bubble_padding * 2.0;
        }
        if self.is_bubble() && is_self_user {
            // Leave room for left labels
            width -= 25.0;
        }
        width
    }

    /// Returns the available bounding area for the content.
    /// 
    /// The min is set to zero space, and the max is the max height and
    /// the width is the width provided by [get_available_content_width()]
    pub fn get_available_content_area(&self, space_available: &BoxConstraints, is_self_user: bool) -> BoxConstraints {
        helper_functions::to_full_height_area(self.get_available_content_width(space_available, is_self_user), space_available)
    }

    /// Returns the available bounding area for the content.
    /// 
    /// The min is set to zero space, and the max is the max height and
    /// the width is either the total width minus the left spacing, or the IRC width minus the picture size
    pub fn get_sender_label_area(&self, space_available: &BoxConstraints) -> BoxConstraints {
        let width = if self.is_side_by_side(space_available) {
            IRC_HEADER_WIDTH - self.picture_size
        } else {
            space_available.max().width
        };
        helper_functions::to_full_height_area(width, space_available)
    }

    /// Gets the unpadded content x left position
    /// This is how far right the content needs to move right
    /// Depending on the layout and content, it can be left or right aligned.
    /// 
    /// If left aligned, it is just pushed to the right of the profile pic and its padding.
    /// If right aligned, it subtracts the content size from the available space.
    pub fn get_unpadded_content_x_left_position(&self, is_self_user: bool, space_available: &BoxConstraints,
        actual_max_content_width: f64, total_metadata_width: f64) -> f64
    {
        if is_self_user && self.is_bubble() { // Only shift if using a bubble layout
            let required_width = if self.item_layout != ItemLayoutOption::BubbleExternBottomMeta
                && total_metadata_width > actual_max_content_width
            {
                // For ExternBottomMeta, the content is on the outside, so the bubble doesn't affect the size
                total_metadata_width
            } else {
                actual_max_content_width
            };
            // Offset so that the profile pic is pushed all the way to the right
            space_available.max().width - required_width
                - self.bubble_padding * 2.0 - self.get_profile_pic_area_width(is_self_user)
        } else {
            // Push to right of profile pic
            self.get_profile_pic_area_width(is_self_user)
        }
    }

    /// Gets the origin position for the content, taking into account things including padding,
    /// layout, and the size of other items.
    pub fn get_content_origin(&self, is_self_user: bool, space_available: &BoxConstraints, y_top_offset: f64,
        widest_msg_content: f64, total_metadata_width: f64, metadata_height: f64) -> Point
    {
        let content_x_start = self.get_unpadded_content_x_left_position(
            is_self_user, space_available, widest_msg_content, total_metadata_width
        ) + self.left_spacing;
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta | ItemLayoutOption::BubbleInternalBottomMeta => {
                Point::new(
                    content_x_start + self.bubble_padding,
                    y_top_offset + self.bubble_padding
                )
            },
            ItemLayoutOption::BubbleInternalTopMeta => {
                Point::new(
                    // Align to left inside of bubble
                    content_x_start + self.bubble_padding,
                    // Near the top, below metadata and padding
                    y_top_offset + self.bubble_padding * 2.0 + metadata_height
                )
            },
            ItemLayoutOption::IRCStyle => {
                // Allow having msg and name on same axis if wide enough
                // else stack them
                if self.is_side_by_side(space_available) {
                    // The msg content is to the right of the metadata
                    Point::new(IRC_HEADER_WIDTH, y_top_offset)
                } else {
                    // Stacked, with picture above instead of to side, since this is the most compact layout
                    Point::new(0.0, metadata_height + self.metadata_content_spacing + y_top_offset)
                }
            },
            _ => {
                // Allow text to move all the way to left if the picture's size
                // is less than the height of the meta label
                Point::new(
                    if self.align_to_picture {
                        content_x_start // Nothing special. Just aligned to context x start.
                    } else {
                        0.0 // All the way to the left
                    },
                    // Just below the metadata
                    metadata_height + self.metadata_content_spacing + y_top_offset
                )
            }
        }
    }

    /// Gets the origin position for the sender, taking into account things including padding,
    /// layout, and the size of other items.
    pub fn get_sender_origin(&self, is_self_user: bool, space_available: &BoxConstraints,
        total_metadata_width: f64, total_msg_height: f64, widest_msg_width: f64, y_top_offset: f64) -> Point
    {
        let msg_x_start = self.get_unpadded_content_x_left_position(is_self_user, space_available, 
            widest_msg_width, total_metadata_width);
        match self.item_layout {
            ItemLayoutOption::BubbleExternBottomMeta => {
                // Outside the bubble, under it.
                // Do not let it cut off the screen to right if it's self user
                let metadata_x_start = if is_self_user && total_metadata_width > widest_msg_width {
                    msg_x_start + widest_msg_width - total_metadata_width + self.bubble_padding
                } else {
                    msg_x_start
                };
                Point::new(metadata_x_start, total_msg_height
                    + self.bubble_padding * 2.0 + self.metadata_content_spacing + y_top_offset)
            },
            ItemLayoutOption::BubbleInternalTopMeta => {
                // Inside the bubble, near the top, offset by just the padding
                Point::new(msg_x_start + self.bubble_padding, self.bubble_padding + y_top_offset)
            },
            ItemLayoutOption::BubbleInternalBottomMeta => {
                // Near the bottom of the bubble, but inside it. Offset by padding.
                Point::new(msg_x_start + self.bubble_padding,
                    total_msg_height + self.bubble_padding + self.metadata_content_spacing + y_top_offset)
            },
            _ => {
                // Non-bubble
                Point::new(msg_x_start, 0.0)
            }
        }
    }

    /// Gets the total height of a timeline item widget.
    /// Accounts for everything, including layout, content sizes, other label sizes, and padding.
    pub fn get_total_height(&self, space_available: &BoxConstraints, sender_label_size: &Size,
        msg_label_size: &Size, y_top_offset: f64) -> f64
    {
        if self.is_side_by_side(space_available) {
            sender_label_size.height.max(msg_label_size.height) + y_top_offset
        } else {
            y_top_offset + msg_label_size.height + sender_label_size.height + self.metadata_content_spacing + if self.is_bubble() {
                2.0 * self.bubble_padding // total height of padding (both top and bottom).
            } else {
                0.0 // Not a bubble, so no padding.
            }
        }
    }

    pub fn show_picture(&self, is_self_user: bool) -> bool {
        !is_self_user || self.show_self_pic || !self.is_bubble()
    }

    /// Used to position the profile pic
    pub fn profile_pic_x_offset(&self, is_self_user: bool, width_available: f64) -> f64 {
        if is_self_user && self.is_bubble() {
            width_available - self.picture_size
        } else {
            0.0
        }
    }

    /// Used to account for cases when the tail will not
    /// just be pinned to the top or bottom
    pub fn get_tail_y_offset(&self, is_self_user: bool) -> f64 {
        if self.chat_bubble_tail_shape == TailShape::Symmetric {
            // It's symmetric and centered at the profile picture
            let mut offset = self.picture_size / 2.0;
            // If the bubble is flipped, the reference point changes to the
            // bottom, so it needs to be negative.
            if self.is_bubble_flipped(is_self_user) {
                offset *= -1.0;
            }
            offset
        } else {
            0.0
        }
    }

    pub fn set_from_predefined_layout(&mut self, layout: PredefinedLayout) {
        match layout {
            PredefinedLayout::ModernHangouts => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 32.0;
                self.chat_bubble_tail_shape = TailShape::ConcaveBottom;
                self.chat_bubble_tail_size = 6.0;
                self.chat_bubble_radius = 4.0;
                self.chat_bubble_picture_spacing = 3.5;
                self.show_self_pic = false;
                self.metadata_content_spacing = 1.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 6.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::ModernBubble => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 32.0;
                self.chat_bubble_tail_shape = TailShape::ConcaveBottom;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 10.0;
                self.chat_bubble_picture_spacing = 6.5;
                self.show_self_pic = false;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 7.0;
                self.group_spacing = 10.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 3.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::LargeBubble => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 40.0;
                self.chat_bubble_tail_shape = TailShape::ConcaveBottom;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 8.0;
                self.chat_bubble_picture_spacing = 6.5;
                self.show_self_pic = true;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 7.0;
                self.group_spacing = 10.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 14.0;
                self.sender_font_size = 12.0;
                self.datetime_font_size = 12.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 3.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::OldHangouts => {
                self.item_layout = ItemLayoutOption::BubbleInternalBottomMeta;
                self.picture_shape = PictureShape::Rectangle;
                self.picture_size = 35.0;
                self.chat_bubble_tail_shape = TailShape::Straight;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 0.5;
                self.chat_bubble_picture_spacing = 0.5;
                self.show_self_pic = true;
                self.metadata_content_spacing = 3.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 9.5;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 10.0;
                self.datetime_font_size = 10.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 200, g: 200, b: 200 };
                self.datetime_color = SimpleColor { r: 200, g: 200, b: 200 };
            },
            PredefinedLayout::IMessage => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 32.0;
                self.chat_bubble_tail_shape = TailShape::Fancy;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 10.0;
                self.chat_bubble_picture_spacing = 6.5;
                self.show_self_pic = false;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 7.0;
                self.group_spacing = 10.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = true;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::Telegram => {
                self.item_layout = ItemLayoutOption::BubbleInternalTopMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 32.0;
                self.chat_bubble_tail_shape = TailShape::ConcaveBottom;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 4.0;
                self.chat_bubble_picture_spacing = 8.0;
                self.show_self_pic = false;
                self.metadata_content_spacing = 5.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 9.5;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = true;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = true;
                self.content_font_size = 13.0;
                self.sender_font_size = 12.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::OldKik => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 30.0;
                self.chat_bubble_tail_shape = TailShape::Symmetric;
                self.chat_bubble_tail_size = 5.5;
                self.chat_bubble_radius = 4.0;
                self.chat_bubble_picture_spacing = 7.0;
                self.show_self_pic = false;
                self.metadata_content_spacing = 1.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 6.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::TearDrop => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 25.0;
                self.chat_bubble_tail_shape = TailShape::Square;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 12.0;
                self.chat_bubble_picture_spacing = 3.5;
                self.show_self_pic = false;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 7.5;
                self.group_spacing = 10.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::Tailless => {
                self.item_layout = ItemLayoutOption::BubbleExternBottomMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 25.0;
                self.chat_bubble_tail_shape = TailShape::Hidden;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 8.0;
                self.chat_bubble_picture_spacing = 3.5;
                self.show_self_pic = false;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 10.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 11.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 175, g: 175, b: 175 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::OtherBubble => {
                self.item_layout = ItemLayoutOption::BubbleInternalTopMeta;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 28.0;
                self.chat_bubble_tail_shape = TailShape::ConcaveBottom;
                self.chat_bubble_tail_size = 7.0;
                self.chat_bubble_radius = 3.0;
                self.chat_bubble_picture_spacing = 6.0;
                self.show_self_pic = false;
                self.metadata_content_spacing = 5.0;
                self.align_to_picture = true;
                self.bubble_padding = 5.0;
                self.group_spacing = 9.5;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.left_bubble_flipped = false;
                self.right_bubble_flipped = true;
                self.metadata_font_bolded = true;
                self.content_font_size = 14.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::Discord => {
                self.item_layout = ItemLayoutOption::Bubbleless;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 40.0;
                self.chat_bubble_picture_spacing = 13.0;
                self.metadata_content_spacing = 7.0;
                self.align_to_picture = true;
                self.bubble_padding = 0.0;
                self.group_spacing = 23.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.metadata_font_bolded = true;
                self.content_font_size = 14.0;
                self.sender_font_size = 14.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = true;
                self.show_old_times_datetime = true;
                self.left_meta_offset = 15.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::CompactDiscord => {
                self.item_layout = ItemLayoutOption::Bubbleless;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 36.0;
                self.chat_bubble_picture_spacing = 8.0;
                self.metadata_content_spacing = 7.0;
                self.align_to_picture = true;
                self.bubble_padding = 0.0;
                self.group_spacing = 13.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.metadata_font_bolded = true;
                self.content_font_size = 13.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = true;
                self.show_old_times_datetime = true;
                self.left_meta_offset = 10.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::Slack => {
                self.item_layout = ItemLayoutOption::Bubbleless;
                self.picture_shape = PictureShape::RoundedRectangle;
                self.picture_size = 36.0;
                self.chat_bubble_picture_spacing = 5.5;
                self.metadata_content_spacing = 5.0;
                self.align_to_picture = true;
                self.bubble_padding = 0.0;
                self.group_spacing = 14.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.metadata_font_bolded = true;
                self.content_font_size = 13.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = true;
                self.show_old_times_datetime = true;
                self.left_meta_offset = 5.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::Compact => {
                self.item_layout = ItemLayoutOption::Bubbleless;
                self.picture_shape = PictureShape::Circle;
                self.picture_size = 25.0;
                self.chat_bubble_picture_spacing = 2.5;
                self.show_self_pic = true;
                self.metadata_content_spacing = 2.0;
                self.align_to_picture = true;
                self.bubble_padding = 0.0;
                self.group_spacing = 8.0;
                self.single_message_spacing = 5.0;
                self.show_left_line = false;
                self.left_spacing = 0.0;
                self.metadata_font_bolded = true;
                self.content_font_size = 13.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = true;
                self.show_old_times_datetime = true;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::IRC => {
                self.item_layout = ItemLayoutOption::IRCStyle;
                self.picture_shape = PictureShape::Rectangle;
                self.picture_size = 16.0;
                self.chat_bubble_picture_spacing = 3.5;
                self.show_self_pic = true;
                self.metadata_content_spacing = 3.0;
                self.align_to_picture = false;
                self.group_spacing = 6.0;
                self.single_message_spacing = 5.0;
                self.bubble_padding = 6.0;
                self.show_left_line = true;
                self.left_spacing = 4.0;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::LargeIRC => {
                self.item_layout = ItemLayoutOption::IRCStyle;
                self.picture_shape = PictureShape::Rectangle;
                self.picture_size = 18.0;
                self.chat_bubble_picture_spacing = 4.0;
                self.show_self_pic = true;
                self.metadata_content_spacing = 3.0;
                self.align_to_picture = false;
                self.group_spacing = 7.0;
                self.single_message_spacing = 6.0;
                self.bubble_padding = 6.0;
                self.show_left_line = true;
                self.left_spacing = 5.0;
                self.metadata_font_bolded = false;
                self.content_font_size = 14.0;
                self.sender_font_size = 14.0;
                self.datetime_font_size = 12.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
            PredefinedLayout::SpacedIRC => {
                self.item_layout = ItemLayoutOption::IRCStyle;
                self.picture_shape = PictureShape::Rectangle;
                self.picture_size = 16.0;
                self.chat_bubble_picture_spacing = 3.5;
                self.show_self_pic = true;
                self.metadata_content_spacing = 6.0;
                self.align_to_picture = false;
                self.group_spacing = 12.0;
                self.single_message_spacing = 5.0;
                self.bubble_padding = 6.0;
                self.show_left_line = true;
                self.left_spacing = 4.5;
                self.metadata_font_bolded = false;
                self.content_font_size = 13.0;
                self.sender_font_size = 13.0;
                self.datetime_font_size = 11.0;
                self.relative_datetime = false;
                self.show_old_times_datetime = false;
                self.left_meta_offset = 2.0;
                self.sender_color = SimpleColor { r: 255, g: 255, b: 255 };
                self.datetime_color = SimpleColor { r: 175, g: 175, b: 175 };
            },
        }
    }
}