use bevy::{
    mesh::RectangleMeshBuilder,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .add_systems(Update,controls)
        //TODO: default()
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .run();
}
// fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
//     //if primary window is not activated then better to panic
//     let window = window_query.single().unwrap();
//     //Spawn camera little higher so you can see objects
//     commands.spawn((
//         Camera2d,
//         Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
//     ));
// }
//
//Lets spawn camera at center without changing anything for now
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    commands.spawn(Camera2d);
    let mesh = meshes.add(RectangleMeshBuilder::new(200.0, 200.0));
    let color_material = color_materials.add(MandelbrotMaterial {
        view: Vec4::new(-0.5, 0.0, 2.0, 100.0),
    });
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(color_material),
        //splat basically means take a thing and put it in multiple places
        Transform::from_scale(Vec3::splat(8.0)),
    ));
}
fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    let dt = time.delta_secs();
    for (_, m) in materials.iter_mut() {
        let zoom = m.view.z;
        if keys.pressed(KeyCode::KeyW) { m.view.y += zoom * dt; }
        if keys.pressed(KeyCode::KeyS) { m.view.y -= zoom * dt; }
        if keys.pressed(KeyCode::KeyA) { m.view.x -= zoom * dt; }
        if keys.pressed(KeyCode::KeyD) { m.view.x += zoom * dt; }
        if keys.pressed(KeyCode::KeyE) { m.view.z *= 1.0 - dt; }  // zoom in
        if keys.pressed(KeyCode::KeyQ) { m.view.z *= 1.0 + dt; }  // zoom out
    }
}
#[derive(Asset, Clone, Debug, TypePath, AsBindGroup)]
struct MandelbrotMaterial {
    #[uniform(0)]
    view: Vec4
}

impl Material2d for MandelbrotMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/mandel.wgsl".into()
    }
}
