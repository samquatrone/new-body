use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "N-Body Simulation".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(control)
        .run();
}

#[derive(Component)]
struct Camera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Shapes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("#ffd891").unwrap(),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("#000080").unwrap(),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.0,
            range: 100.0,
            ..default()
        },
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 0.0, 8.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        Camera,
    ));
}

fn control(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera.single_mut();

    let rotation = if input.pressed(KeyCode::Left) {
        time.delta_seconds()
    } else if input.pressed(KeyCode::Right) {
        -time.delta_seconds()
    } else {
        0.0
    };

    camera_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
    );
}
