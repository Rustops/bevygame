use serde::{Deserialize, Serialize};
use bevy_networking_turbulence::{NetworkResource, ConnectionChannelsBuilder, MessageChannelSettings, MessageChannelMode, ReliableChannelSettings};
use bevy::{
    ecs::bundle::Bundle,
    prelude::ResMut
};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Hello,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Welcome(UserId),
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct UserId(pub u32);

#[derive(Serialize, Deserialize)]
struct NetworkHandle(u32);

#[derive(Debug)]
pub struct Name(pub String);

#[derive(Bundle)]
pub struct UserBundle {
    user_id: UserId,
    user_name: Name,
}

impl UserBundle {
    pub fn new(user_id: UserId) -> UserBundle {
        UserBundle {
            user_id,
            user_name: Name(String::from("new_user")),
        }
    }
}

pub fn network_channels_setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ServerMessage>(SERVER_MESSAGE_SETTINGS)
            .unwrap();
    });
}

pub const CLIENT_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

pub const SERVER_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};
