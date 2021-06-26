use bevy::{
    prelude::*,
    MinimalPlugins,
};

use bevy_networking_turbulence::{NetworkResource, NetworkingPlugin, Packet, NetworkEvent};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use share::ClientMessage;
// use bevy_web_fullscreen::FullViewportPlugin;

const SERVER_PORT: u16 = 9001;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("cannot initialize console_log");

    App::build()
        // .add_plugins(bevy_webgl2::DefaultPlugins)
        // .add_plugin(FullViewportPlugin)
        // The NetworkingPlugin
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(share::network_channels_setup.system())
        .add_startup_system(setup_world.system())
        // Our networking
        .add_startup_system(client_setup.system())
        .add_system(send_packets.system())
        .add_system(handle_network_events.system())
        .add_system(keyboard_input.system())
        .run();
}

fn setup_world(
) {
    // TODO: How it works?
}

fn client_setup(mut net: ResMut<NetworkResource>) {
    // let ip_address =
    //     bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    // TODO: Read ip_address from cli.arg
    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    log::info!("Starting client");
    info!("Connecting to {}...", socket_address);
    net.connect(socket_address);
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        log::info!("PING");
        net.broadcast(Packet::from("PING"));
    }
}

fn handle_network_events(
    mut net: ResMut<NetworkResource>,
    mut net_event_reader: EventReader<NetworkEvent>,
) {
    for event in net_event_reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(&handle) {
                Some(_connection) => {
                    info!("Connection successful");

                    net.send_message(*handle, ClientMessage::Hello)
                        .expect("Could not send hello");
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}

fn keyboard_input(keyboard_input: Res<Input<KeyCode>>) {
    let pressed = keyboard_input.get_just_pressed();
    for key in pressed {
        info!("Keyboard input: {:?}", key);
    }
    // TODO: Read msg and pass the msg
}