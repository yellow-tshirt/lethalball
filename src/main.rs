use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_circle)
        .run();
}

#[derive(Component)]
struct C;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let circle = meshes.add(Circle::new(30.0));
    let color = Color::srgb(0.39, 0.39, 0.82);
    commands.spawn((
        C{},
        //physics
        Collider::ball(30.0),
        Restitution::coefficient(0.1),
        RigidBody::KinematicPositionBased,
        //phsyics end
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    //make second ball
    let second_ball = meshes.add(Circle::new(30.0));
    let color = Color::srgb(0.39, 0.39, 0.82);
    commands.spawn((Mesh2d(second_ball), MeshMaterial2d(materials.add(color)), Collider::ball(30.0), Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Restitution::coefficient(0.1))
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic);
    //make other platforms
    let bottom_wall = meshes.add(Rectangle::new(1000.0, 10.0));
    commands.spawn((Mesh2d(bottom_wall), MeshMaterial2d(materials.add(color)), Collider::cuboid(1000.0, 10.0), Transform::from_xyz(0.0, -200.0, 0.0)))
        .insert(Restitution::coefficient(0.1))
        .insert(RigidBody::Fixed);
    //make top
    let top_wall = meshes.add(Rectangle::new(1000.0, 10.0));
    commands.spawn((Mesh2d(top_wall), MeshMaterial2d(materials.add(color)), Collider::cuboid(1000.0, 10.0), Transform::from_xyz(0.0, 200.0, 0.0)))
        .insert(Restitution::coefficient(0.1))
        .insert(RigidBody::Fixed);
    //make right
    let right_wall = meshes.add(Rectangle::new(10.0, 1000.0));
    commands.spawn((Mesh2d(right_wall), MeshMaterial2d(materials.add(color)), Collider::cuboid(10.0, 1000.0), Transform::from_xyz(300.0, 0.0, 0.0)))
        .insert(Restitution::coefficient(0.1))
        .insert(RigidBody::Fixed);
    //make left
    let left_wall = meshes.add(Rectangle::new(10.0, 1000.0));
    commands.spawn((Mesh2d(left_wall), MeshMaterial2d(materials.add(color)), Collider::cuboid(10.0, 1000.0), Transform::from_xyz(-300.0, 0.0, 0.0)))
        .insert(Restitution::coefficient(0.1))
        .insert(RigidBody::Fixed);
}

fn move_circle(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut circle_q: Query<&mut Transform, With<C>>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();

    if let Some(world_position) = window.cursor_position().and_then(|cursor| {
        let err = camera.viewport_to_world_2d(camera_transform, cursor);
        match err {
            Ok(pos) => Some(pos),
            Err(_) => None,
        }
    }) {
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        for mut c in &mut circle_q {
            //change position of the circle
            c.translation.x = world_position.x;
            c.translation.y = world_position.y;
        }
    }
}