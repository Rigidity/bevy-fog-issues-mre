use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

#[allow(unused)]
enum Example {
    WorkingTonemappingBlack,
    NotWorkingTonemappingWhite,
    WorkingWhite,
    NotWorkingOtherColor,
}

const EXAMPLE: Example = Example::NotWorkingOtherColor;

const COLOR: Color = match EXAMPLE {
    Example::WorkingTonemappingBlack => Color::BLACK,
    Example::NotWorkingTonemappingWhite => Color::WHITE,
    Example::WorkingWhite => Color::WHITE,
    Example::NotWorkingOtherColor => Color::rgb(0.1, 0.0, 1.0),
};

const TONEMAPPING: Tonemapping = match EXAMPLE {
    Example::WorkingTonemappingBlack => Tonemapping::TonyMcMapface,
    Example::NotWorkingTonemappingWhite => Tonemapping::TonyMcMapface,
    Example::WorkingWhite => Tonemapping::None,
    Example::NotWorkingOtherColor => Tonemapping::None,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(ClearColor(COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(FlyCam)
        .insert(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 0.0, -2.0))
                .looking_at(Vec3::NEG_Z, Vec3::Y),
            tonemapping: TONEMAPPING,
            ..default()
        })
        .insert(FogSettings {
            falloff: FogFalloff::Linear {
                start: 6.0,
                end: 10.0,
            },
            color: COLOR,
            directional_light_color: COLOR,
            ..default()
        });

    for i in 0..10 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::GOLD.into()),
            transform: Transform::from_translation(Vec3::new(i as f32 * 1.2, -1.0, i as f32 * 1.2)),
            ..default()
        });
    }
}
