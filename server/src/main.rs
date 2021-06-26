use bevy::{
    prelude::*,
    core::Time,
    app::{App, EventReader, ScheduleRunnerSettings},
    MinimalPlugins,
};

use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use std::{net::{SocketAddr, IpAddr, Ipv4Addr}, time::Duration};
use share::{ClientMessage, ServerMessage, UserId};
use serde::{Deserialize, Serialize};

const SERVER_PORT: u16 = 9001;

#[derive(Serialize, Deserialize)]
struct NetworkHandle(u32);

fn main() {
    App::build()
        // minimal plugins necessary for timers + headless loop
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(
            1000 / 30,
        )))
        .add_plugins(MinimalPlugins)
        // The NetworkingPlugin
        .add_plugin(NetworkingPlugin::default())
        // Our networking
        .add_startup_system(share::network_channels_setup.system())
        .add_startup_system(server_setup.system())
        .add_system(read_network_channels.system())
        .add_system(handle_network_events.system())
        .add_system_to_stage(CoreStage::PreUpdate, read_network_channels.system())
        .run();
}

fn server_setup(mut net: ResMut<NetworkResource>) {
    // let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    // TODO: Set port from cli.arg
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
    net.listen(socket_address, None, None);
    log::info!("Starting server");
    println!("start server")
}

fn read_network_channels(
    mut net: ResMut<NetworkResource>
) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(message) = channels.recv::<ClientMessage>() {
            println!("Received message: {:?}", message);
        }
    }
}

fn handle_network_events(
    mut cmd: Commands,
    mut net: ResMut<NetworkResource>,
    mut net_event_reader: EventReader<NetworkEvent>,
    time: Res<Time>,
    unowned_users: Query<(Entity, &UserId), Without<NetworkHandle>>,
) {
    for event in net_event_reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(&handle) {
                Some(_connection) => {
                    println!("New connection handle: {:?}", handle);

                    let (entity, user) = unowned_users.iter().next().expect("No unowned user");
                    cmd.entity(entity).insert(NetworkHandle(*handle));
                    net.send_message(*handle, ServerMessage::Welcome(*user))
                        .expect("Could not send welcome");
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                println!("Got packet on [{}]: {}", handle, message);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            log::info!("Sent PONG");
                        }
                        Err(error) => {
                            log::info!("PONG send error: {}", error);
                        }
                    }
                }
            },
            _ => {}
        }
    }
}
