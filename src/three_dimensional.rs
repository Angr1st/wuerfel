use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::prelude::*;

use crate::core::Error;
use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub fn run_three_dimensional() -> Result<(), Error> {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin::default(),
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Wuerfel".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_jump)
        .run();
    Ok(())
}

/// A marker component for our wuerfel  so we can query them separately from the ground plane
#[derive(Component)]
struct Wuerfel;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("wuerfel.png");

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: texture_handle.into(),
        ..Default::default()
    });

    let shape = meshes.add(Cuboid::default());

    commands
        .spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(0.0, 7.0, 0.0),
                ..default()
            },
            Wuerfel,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.9))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(ColliderMassProperties::Mass(10.0));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(100.0, 100.0)
                    .subdivisions(10),
            ),
            material: materials.add(Color::from(SILVER)),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.0, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section("Press space to jump the wuerfel", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn toggle_jump(
    mut query: Query<&mut ExternalImpulse, With<Wuerfel>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut ext_impulse in &mut query {
            ext_impulse.impulse = Vec3::new(0.0, 90.0, 0.0);
            ext_impulse.torque_impulse = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}
