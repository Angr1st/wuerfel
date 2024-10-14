//use bevy_rapier3d::prelude::*;

use std::f32::consts::PI;

use crate::core::Error;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub fn run_three_dimensional() -> Result<(), Error> {
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    //     .add_plugins(RapierDebugRenderPlugin::default())
    //     .add_systems(Startup, setup_graphics)
    //     .add_systems(Startup, setup_physics)
    //     .add_systems(Update, print_ball_altitude)
    //     .run();
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, (setup_env, add_assets, spawn_tasks))
    //     .add_systems(Update, handle_tasks)
    //     .run();

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WireframePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, toggle_wireframe))
        .run();
    Ok(())
}

// fn setup_graphics(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Add a camera so we can see the debug-render.
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
//     let box_mesh_handle = meshes.add(Cuboid::new(0.25, 0.25, 0.25));
//     commands.insert_resource(BoxMeshHandle(box_mesh_handle));

//     let box_material_handle = materials.add(Color::srgb(1.0, 0.2, 0.3));
//     commands.insert_resource(BoxMaterialHandle(box_material_handle));
// }

// fn setup_physics(mut commands: Commands) {
//     /* Create the ground. */
//     commands
//         .spawn(Collider::cuboid(100.0, 0.1, 100.0))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

//     /* Create the bouncing ball. */
//     commands
//         .spawn(RigidBody::Dynamic)
//         .insert(Collider::cuboid(0.5, 0.5, 0.5))
//         .insert(Restitution::coefficient(0.7))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
// }

// fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
//     for mut transform in positions.iter_mut() {
//         dbg!(transform.rotation.to_axis_angle());
//         transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
//         //println!("Ball altitude: {}", transform.translation.y);
//     }
// }

// Number of cubes to spawn across the x, y, and z axis
// const NUM_CUBES: u32 = 6;

// #[derive(Resource, Deref)]
// struct BoxMeshHandle(Handle<Mesh>);

// #[derive(Resource, Deref)]
// struct BoxMaterialHandle(Handle<StandardMaterial>);

// /// Startup system which runs only once and generates our Box Mesh
// /// and Box Material assets, adds them to their respective Asset
// /// Resources, and stores their handles as resources so we can access
// /// them later when we're ready to render our Boxes
// fn add_assets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let box_mesh_handle = meshes.add(Cuboid::new(0.25, 0.25, 0.25));
//     commands.insert_resource(BoxMeshHandle(box_mesh_handle));

//     let box_material_handle = materials.add(Color::srgb(1.0, 0.2, 0.3));
//     commands.insert_resource(BoxMaterialHandle(box_material_handle));
// }

// #[derive(Component)]
// struct ComputeTransform(Task<CommandQueue>);

// /// This system generates tasks simulating computationally intensive
// /// work that potentially spans multiple frames/ticks. A separate
// /// system, [`handle_tasks`], will poll the spawned tasks on subsequent
// /// frames/ticks, and use the results to spawn cubes
// fn spawn_tasks(mut commands: Commands) {
//     let thread_pool = AsyncComputeTaskPool::get();
//     for x in 0..NUM_CUBES {
//         for y in 0..NUM_CUBES {
//             for z in 0..NUM_CUBES {
//                 // Spawn new task on the AsyncComputeTaskPool; the task will be
//                 // executed in the background, and the Task future returned by
//                 // spawn() can be used to poll for the result
//                 let entity = commands.spawn_empty().id();
//                 let task = thread_pool.spawn(async move {
//                     let duration = Duration::from_secs_f32(oorandom::Rand32::new(32).rand_float());

//                     // Pretend this is a time-intensive function. :)

//                     // Such hard work, all done!
//                     let transform = Transform::from_xyz(x as f32, y as f32, z as f32);
//                     let mut command_queue = CommandQueue::default();

//                     // we use a raw command queue to pass a FnOne(&mut World) back to be
//                     // applied in a deferred manner.
//                     command_queue.push(move |world: &mut World| {
//                         let (box_mesh_handle, box_material_handle) = {
//                             let mut system_state = SystemState::<(
//                                 Res<BoxMeshHandle>,
//                                 Res<BoxMaterialHandle>,
//                             )>::new(world);
//                             let (box_mesh_handle, box_material_handle) =
//                                 system_state.get_mut(world);

//                             (box_mesh_handle.clone(), box_material_handle.clone())
//                         };

//                         world
//                             .entity_mut(entity)
//                             // Add our new PbrBundle of components to our tagged entity
//                             .insert(PbrBundle {
//                                 mesh: box_mesh_handle,
//                                 material: box_material_handle,
//                                 transform,
//                                 ..default()
//                             })
//                             // Task is complete, so remove task component from entity
//                             .remove::<ComputeTransform>();
//                     });

//                     command_queue
//                 });

//                 // Spawn new entity and add our new task as a component
//                 commands.entity(entity).insert(ComputeTransform(task));
//             }
//         }
//     }
// }

// /// This system queries for entities that have our Task<Transform> component. It polls the
// /// tasks to see if they're complete. If the task is complete it takes the result, adds a
// /// new [`PbrBundle`] of components to the entity using the result from the task's work, and
// /// removes the task component from the entity.
// fn handle_tasks(mut commands: Commands, mut transform_tasks: Query<&mut ComputeTransform>) {
//     for mut task in &mut transform_tasks {
//         if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
//             // append the returned command queue to have it execute later
//             commands.append(&mut commands_queue);
//         }
//     }
// }

// /// This system is only used to setup light and camera for the environment
// fn setup_env(mut commands: Commands) {
//     // Used to center camera on spawned cubes
//     let offset = if NUM_CUBES % 2 == 0 {
//         (NUM_CUBES / 2) as f32 - 0.5
//     } else {
//         (NUM_CUBES / 2) as f32
//     };

//     // lights
//     commands.spawn(PointLightBundle {
//         transform: Transform::from_xyz(4.0, 12.0, 15.0),
//         ..default()
//     });

//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(offset, offset, 15.0)
//             .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
//         ..default()
//     });
// }

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let extrusions = [
        meshes.add(Extrusion::new(Rectangle::default(), 1.)),
        meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
        meshes.add(Extrusion::new(Annulus::default(), 1.)),
        meshes.add(Extrusion::new(Circle::default(), 1.)),
        meshes.add(Extrusion::new(Ellipse::default(), 1.)),
        meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                    2.0,
                    Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    let num_extrusions = extrusions.len();

    for (i, shape) in extrusions.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -EXTRUSION_X_EXTENT / 2.
                        + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                    2.0,
                    -Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
