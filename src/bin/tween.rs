use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::asset::{Assets, Handle};

use bevy::color::palettes;
use bevy::math::{Quat, Vec2};
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::BillboardText;
use bevy_mod_billboard::prelude::BillboardPlugin;
use bevy_tween::DefaultTweenPlugins;
use bevy_tween::prelude::*;
use card_test::camera_controller::{CameraController, CameraControllerPlugin};
use card_test::card_info::get_card_info;
use card_test::cards::{Card, CardInfo, Dragging, Setted, gen_put_card};
use card_test::cases::{CaseImages, CasePlane, render_case};
use card_test::hands::{CardPlane, HandCard, HandCardPlugin, calculate_hand_positions};
use card_test::{CommonPlugin, MainCamera};
use std::f32::consts::PI;
use std::num::NonZero;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            // CameraControllerPlugin,
            // 动画相关
            DefaultTweenPlugins,
            CommonPlugin,
            HandCardPlugin,
            BillboardPlugin,
            // Text3dPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        // .add_systems(Update, show_text)
        .add_systems(Update, card_test::cards::clear_on_finish_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    // 自由相机来测试Ω
    commands.spawn((
        MainCamera,
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
        Transform::from_xyz(0.0, 0.0, 10.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));
    let case_plane =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));

    commands.spawn((CardPlane, card_plane));
    commands.spawn((CasePlane, case_plane));

    // 卡片放置器 放置在查看面上
    let mut card_fn =
        gen_put_card::<CardPlane>(&mut materials, &mut meshes, 3. / 1.4, 3., 0.05, 0.01);

    let card_list = [
        "NAAI-A-001",
        "NAAI-A-001",
        "NAAI-A-001",
        "S001-A-001",
        "S001-A-001",
        "S001-A-001",
    ];

    let hand_positions = calculate_hand_positions(
        card_list.len(),
        0.0,
        200.,
        PI / 4.,
        card_plane.translation.z,
        -6.7,
    );

    hand_positions
        .iter()
        .enumerate()
        .for_each(|(index, hand_position)| {
            let entity = card_fn(
                &mut commands,
                asset_server.load(format!("{}.png", card_list.get(index).unwrap())),
                hand_position.clone(),
                get_card_info(card_list.get(index).unwrap()),
            );
            commands.entity(entity).insert(HandCard);
        })
}

// 设置后显示信息
fn show_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &Card), Added<Setted>>,
) {
    let fira_sans_regular_handle = asset_server.load("fonts/wqy-microhei.ttc");

    query.iter_mut().for_each(|(entity, card)| {
        info!("添加文字{}", card.clone().info.name);

        commands.entity(entity).with_children(|parent| {
            parent
                .spawn((
                    BillboardText::default(),
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(0.0, 1.8, 1.0).with_scale(Vec3::splat(0.01)),
                ))
                .with_children(|info_plane| {
                    info_plane.spawn((
                        TextSpan::new(card.clone().info.name),
                        TextFont::from_font(fira_sans_regular_handle.clone()).with_font_size(60.0),
                        TextColor::from(Color::Srgba(palettes::css::WHITE)),
                    ));
                });
            parent
                .spawn((
                    BillboardText::default(),
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(0.0, -1.4, 1.0).with_scale(Vec3::splat(0.01)),
                ))
                .with_children(|info_plane| {
                    info_plane.spawn((
                        TextSpan::new(format!("ACK:{}", card.clone().info.ack)),
                        TextFont::from_font(fira_sans_regular_handle.clone()).with_font_size(60.0),
                        TextColor::from(Color::Srgba(palettes::css::WHITE)),
                    ));
                });
        });
    })
}
