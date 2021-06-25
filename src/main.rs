use bevy::prelude::*;

fn hello_bevy() {
    println!("hello bevy!");
}

fn main() {
    App::build()
        .add_system(hello_bevy.system())
        .run();
}
