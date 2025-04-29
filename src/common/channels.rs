use bevy_quinnet::shared::channels::{ChannelKind, DEFAULT_MAX_RELIABLE_FRAME_LEN};

pub const ORDERED_RELIABLE: u8 = 0;
pub const UNORDERED_RELIABLE: u8 = 1;
pub const UNRELIABLE: u8 = 2;

pub const CHANNELS: [ChannelKind; 3] = [
    ChannelKind::OrderedReliable {
        max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
    },
    ChannelKind::UnorderedReliable {
        max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
    },
    ChannelKind::Unreliable,
];
