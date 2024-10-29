use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::prelude::*;

use crate::core::Error;
use bevy::{color::palettes::basic::SILVER, prelude::*};

pub fn run_three_dimensional(random: oorandom::Rand32) -> Result<(), Error> {
    let mut app = App::new();
    app.add_plugins((
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
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    #[cfg(debug_assertions)]
    {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
    app.insert_resource(Random(random))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_jump,
                camera_follow_cube,
                ground_follow_cube,
                light_follow_cube,
            ),
        );
    app.run();
    Ok(())
}

#[derive(Resource)]
struct Random(oorandom::Rand32);

/// A marker component for our wuerfel  so we can query them separately from the ground plane
#[derive(Component)]
struct Wuerfel;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Light;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let scene_handle = asset_server.load("wuerfel.glb#Scene0");
    // let mesh_handle = asset_server.load("wuerfel.glb#Mesh0/Primitive0");
    // let texture_handle = asset_server.load("wuerfel.glb#Material0");

    // let debug_material = materials.add(texture_handle);
    //     StandardMaterial {
    //     base_color_texture: texture_handle.into(),

    //     ..Default::default()
    // });

    // let shape = meshes.add(mesh_handle);

    commands
        .spawn((
            SceneBundle {
                scene: scene_handle,
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..Default::default()
            },
            // PbrBundle {
            //     mesh: shape,
            //     material: debug_material.clone(),
            //     transform: Transform::from_xyz(0.0, 7.0, 0.0),
            //     ..default()
            // },
            Wuerfel,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Restitution::coefficient(0.9))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(ColliderMassProperties::Mass(10.0));

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                intensity: 10_000_000.,
                range: 100.0,
                shadow_depth_bias: 0.2,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 16.0, 0.0),
            ..default()
        },
        Light,
    ));

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
        .insert(Ground)
        .insert(Collider::cuboid(100.0, 0.0, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 0.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        Camera,
    ));

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
    mut random: ResMut<Random>,
    mut query: Query<&mut ExternalImpulse, With<Wuerfel>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut ext_impulse in &mut query {
            let x = random.0.rand_float() * 10.0 * random.0.rand_float();
            let z = random.0.rand_float() * 10.0 * random.0.rand_float();
            ext_impulse.impulse = Vec3::new(x, 90.0, z);
            ext_impulse.torque_impulse = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}

fn camera_follow_cube(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    wuerfel_query: Query<&Transform, (With<Wuerfel>, Without<Camera>)>,
) {
    for mut camera in &mut camera_query {
        for wuerfel in &wuerfel_query {
            camera.translation.x = wuerfel.translation.x;
            camera.translation.z = wuerfel.translation.z;

            //The camera should alway be at least 15.0 y from the ground.
            //If the wuerfel is approaching the camera it should maintain at least 10.0 y distance from it.
            let wuerfel_y = wuerfel.translation.y;
            if wuerfel_y < 5.0 {
                camera.translation.y = 15.0;
            } else {
                camera.translation.y = wuerfel_y + 10.0;
            }
        }
    }
}

fn ground_follow_cube(
    mut ground_query: Query<&mut Transform, With<Ground>>,
    wuerfel_query: Query<&Transform, (With<Wuerfel>, Without<Ground>)>,
) {
    for mut ground in &mut ground_query {
        for wuerfel in &wuerfel_query {
            ground.translation.x = wuerfel.translation.x;
            ground.translation.z = wuerfel.translation.z;
        }
    }
}

fn light_follow_cube(
    mut light_query: Query<&mut Transform, With<Light>>,
    wuerfel_query: Query<&Transform, (With<Wuerfel>, Without<Light>)>,
) {
    for mut light in &mut light_query {
        for wuerfel in &wuerfel_query {
            light.translation.x = wuerfel.translation.x + 8.0;
            light.translation.z = wuerfel.translation.z + 8.0;
        }
    }
}
