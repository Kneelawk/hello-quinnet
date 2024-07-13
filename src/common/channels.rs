use bevy_quinnet::shared::channels::ChannelType;

pub const ORDERED_RELIABLE: u8 = 0;
pub const UNORDERED_RELIABLE: u8 = 1;
pub const UNRELIABLE: u8 = 2;

pub const CHANNELS: [ChannelType; 3] = [
    ChannelType::OrderedReliable,
    ChannelType::UnorderedReliable,
    ChannelType::Unreliable,
];
