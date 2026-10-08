use bevy::{
    color::palettes::css::PURPLE,
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
    let color_material = color_materials.add(MandelbrotMaterial {});
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(color_material),
        //splat basically means take a thing and put it in multiple places
        Transform::from_scale(Vec3::splat(8.0)),
    ));
}

#[derive(Asset, Clone, Debug, TypePath, AsBindGroup)]
struct MandelbrotMaterial {}

impl Material2d for MandelbrotMaterial {}
