use bevy::{
    input::{common_conditions::input_toggle_active, mouse::MouseMotion},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            rotate.run_if(input_toggle_active(true, KeyCode::KeyR)),
        )
        .add_systems(Update, orbit_camera)
        .run();
}

#[derive(Component, Clone)]
struct SphericalPosition {
    radius: f32,
    polar: f32,
    azimuthal: f32,
}

impl SphericalPosition {
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.radius * self.polar.sin() * self.azimuthal.cos(),
            y: self.radius * self.polar.cos(),
            z: self.radius * self.polar.sin() * self.azimuthal.sin(),
        }
    }
}

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct Id(i32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Shape,
        Id(0),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Shape,
        Id(1),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Camera
    let spherical_position = SphericalPosition {
        radius: 5.0,
        polar: 1.0,
        azimuthal: 0.0,
    };
    commands.spawn((
        Camera3d::default(),
        spherical_position.clone(),
        Transform::from_translation(spherical_position.clone().to_vec3())
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            ..default()
        },
        Transform::from_xyz(4.0, 4.0, 4.0),
    ));
}

fn rotate(mut query: Query<(&Id, &mut Transform), With<Shape>>, time: Res<Time>) {
    for (id, mut transform) in &mut query {
        if id.0 == 0 {
            transform.rotate_y(time.delta_secs() / 2.);
        } else {
            transform.rotate_y(-time.delta_secs() / 2.);
        }
    }
}

fn orbit_camera(
    mut mouse_motion: MessageReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut SphericalPosition, &mut Transform)>,
) {
    let sensitivity = 0.005;

    // Only orbit while holding the right mouse button.
    if !buttons.pressed(MouseButton::Right) {
        mouse_motion.clear();
        return;
    }

    let mut delta = Vec2::ZERO;

    for event in mouse_motion.read() {
        delta += event.delta;
    }

    for (mut orbit, mut transform) in &mut query {
        orbit.azimuthal += delta.x * sensitivity;
        orbit.polar -= delta.y * sensitivity;

        orbit.polar = orbit.polar.clamp(0.01, std::f32::consts::PI - 0.01);

        let target = Vec3::ZERO;

        transform.translation = orbit.to_vec3();
        transform.look_at(target, Vec3::Y);
    }
}
