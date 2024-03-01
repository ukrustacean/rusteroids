mod asset_loader;
mod asteroid;
mod camera;
mod despawn;
mod movement;
mod schedule;
mod spaceship;

use bevy::prelude::*;

use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use camera::CameraPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.1, 0.3)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}