use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;

use bevy::math::{Quat, Vec2};
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card_test::camera_controller::{CameraController, CameraControllerPlugin};
use card_test::cards::gen_put_card;
use card_test::cases::{CaseImages, CasePlane, render_case};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            // CameraControllerPlugin,
            // 动画相关
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct CardPlane;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    // 自由相机来测试Ω
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 25.).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 场地相关的 素材
    let case_images: CaseImages = CaseImages {
        stone1: asset_server.load("stone_1.png"),
        stone2: asset_server.load("stone_2.png"),
        safe: asset_server.load("safe.png"),
        lx: asset_server.load("lx.png"),
        jq: asset_server.load("jq.png"),
    };

    render_case(&mut commands, &mut meshes, &mut materials, case_images);

    // 设置两个用来触发的 平面 用来计算当前鼠标的位置
    let card_plane =
        Transform::from_xyz(0.0, 0.0, 18.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));
    let case_plane =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));

    commands.spawn((CardPlane, card_plane));
    commands.spawn((CasePlane, case_plane));

    // 卡片放置器 放置在查看面上
    let mut card_fn = gen_put_card::<CardPlane>(
        &mut materials,
        &mut meshes,
        3. / 1.4,
        3.,
        0.05,
        0.01,
    );
    let yellow = asset_server.load("NAAI-A-001.png");
    card_fn(
        &mut commands,
        yellow.clone(),
        Transform::from_xyz(0., 0., card_plane.translation.z),
    );
}
