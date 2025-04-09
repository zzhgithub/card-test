use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card_test::cards::gen_put_card;
use std::f32::consts::PI;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource, Default)]
struct MyAssets {
    pub vertin: Handle<Image>,
    pub yellow: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .init_resource::<MyAssets>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(
            Startup,
            |asset_server: Res<AssetServer>, mut assets: ResMut<MyAssets>| {
                let vertin = asset_server.load("default.png");
                let yellow = asset_server.load("NAAI-A-001.png");
                assets.vertin = vertin;
                assets.yellow = yellow;
            },
        )
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .init_state::<GameState>()
        .run();
}

#[derive(Component)]
struct Plane;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<MyAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // if !asset_server
    //     .get_load_state(assets.vertin.id())
    //     .is_some_and(|s| s.is_loaded())
    //     || !asset_server
    //         .get_load_state(assets.yellow.id())
    //         .is_some_and(|s| s.is_loaded())
    // {
    //     return;
    // }
    next_state.set(GameState::Ready);

    //相机
    commands
        .spawn(Camera3d::default())
        .insert(Transform::from_xyz(0., 0., 5.));
    let white_matl = materials.add(Color::WHITE);
    let handle = meshes.add(Extrusion::new(CircularSector::new(1.0, PI / 4.0), 1.));
    // commands.spawn((
    //     Mesh3d(handle),
    //     MeshMaterial3d(white_matl.clone()),
    //     Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4.)),
    // ));
    // Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    // ));

    // 测试一个四分之一圆

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    let card_plane =
        Transform::from_xyz(0.0, 0.0, 15.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));
    let case_plane =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));

    commands.spawn((
        Plane,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        case_plane,
    ));

    let mut card_fn = gen_put_card::<Plane>(&mut materials, &mut meshes, 3. / 1.4, 3., 0.05, 0.01);
    card_fn(&mut commands, assets.vertin.clone(), Transform::default());
    // card_fn(
    //     assets.vertin.clone(),
    //     Transform::from_xyz(-0.5, -0.5, 0.0001),
    // );
}
