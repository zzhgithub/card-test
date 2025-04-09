use crate::cards::Card;
use crate::color::{BaseColor, basic_color};
use crate::hands::HandCardChanged;
use crate::shark::{custom_interpolators_plugin, effect_intensity};
use bevy::app::App;
use bevy::color::palettes::css::{WHITE, YELLOW};
use bevy::picking::focus::update_interactions;
use bevy::prelude::*;
use bevy_tween::asset_tween_system;
use bevy_tween::bevy_time_runner::TimeRunnerEnded;
use bevy_tween::combinator::{backward, forward, sequence, tween};
use bevy_tween::interpolate::{scale, sprite_color, translation_to};
use bevy_tween::prelude::*;
use bevy_tween::tween::AnimationTarget;
use rand::prelude::*;
use std::f32::consts::PI;
use log::info;

pub mod camera_controller;
pub mod cards;
pub mod cases;
pub mod color;
pub mod hands;
pub mod shark;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (effect_system, despawn_effect_system));
        app.add_tween_systems(asset_tween_system::<BaseColor>())
            .register_type::<AssetTween<BaseColor>>();
        custom_interpolators_plugin::<MainCamera>(app);
    }
}

#[derive(Component)]
pub struct OnConfirm;

#[derive(Component)]
pub struct OnCancel;

fn spawn_ui_popup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    title: &'static str,
    mut on_confirm: impl FnMut(&mut Commands, &mut Query<&Children, With<Card>>) + Send + Sync + 'static,
    mut on_cancel: impl FnMut(&mut Commands) + Send + Sync + 'static,
) {
    let all = commands
        .spawn(
            (Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }),
        )
        .id();

    commands.entity(all).with_children(|plane| {
        plane
            .spawn((
                Node {
                    width: Val::Px(500.),
                    height: Val::Px(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.6)),
                BorderColor(Color::BLACK),
                BorderRadius::all(Val::Px(10.0)),
            ))
            .with_children(|parent| {
                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },))
                    .with_children(|title_bar| {
                        title_bar.spawn((
                            Text::new(title),
                            TextFont {
                                font: asset_server.load("fonts/wqy-microhei.ttc"),
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));
                    });

                // 按钮区域
                parent
                    .spawn(
                        (Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        }),
                    )
                    .with_children(|b_zone| {
                        b_zone
                            .spawn((
                                Button,
                                OnConfirm,
                                Node {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.0, 0.1, 0.1)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("确认"),
                                    TextFont {
                                        font: asset_server.load("fonts/wqy-microhei.ttc"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.0)),
                                ));
                            })
                            .observe(
                                move |click: Trigger<Pointer<Click>>, mut commands: Commands, mut children_query: Query<&Children,With<Card>>,
                                      mut changed_event: EventWriter<HandCardChanged>| {
                                    on_confirm(&mut commands, &mut children_query);
                                    commands.entity(all).despawn_recursive();
                                    // 发送手牌变化事件
                                    info!("手牌发生变化");
                                    changed_event.send(HandCardChanged);
                                },
                            );

                        b_zone
                            .spawn((
                                Button,
                                OnCancel,
                                Node {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("取消"),
                                    TextFont {
                                        font: asset_server.load("fonts/wqy-microhei.ttc"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                            })
                            .observe(
                                move |click: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    on_cancel(&mut commands);
                                    commands.entity(all).despawn_recursive();
                                },
                            );
                    });
            });
    });
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct Effect;

fn effect_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event: EventReader<TweenEvent<&'static str>>,
    query: Query<(Entity, &Transform), With<MainCamera>>,
) {
    event.read().for_each(|event| match event.data {
        "small_boom" => {
            let handle = materials.add(StandardMaterial {
                base_color: WHITE.into(),
                unlit: true,
                ..Default::default()
            });
            let mut target = handle.clone().into_target();

            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    // .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0)),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(0.2),
                    EaseKind::Linear,
                    (
                        entity.with(scale(Vec3::new(0.6, 0.6, 0.), Vec3::new(3., 3., 0.))),
                        target.with(basic_color(
                            into_color(WHITE.with_alpha(0.5)),
                            into_color(YELLOW.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "boom" => {
            info!("Boom!");
            let handle = materials.add(StandardMaterial {
                base_color: WHITE.into(),
                unlit: true,
                ..Default::default()
            });
            let mut target = handle.clone().into_target();

            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    // .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0)),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(1.0),
                    EaseKind::QuadraticOut,
                    (
                        entity.with(scale(Vec3::new(1., 1., 0.), Vec3::new(10., 10., 0.))),
                        target.with(basic_color(
                            into_color(WHITE.with_alpha(1.)),
                            into_color(YELLOW.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "shark" => {
            if let Ok((entity, trans)) = query.get_single() {
                commands
                    .entity(entity)
                    .insert(AnimationTarget)
                    .animation()
                    .insert(sequence((
                        tween(
                            Duration::from_secs_f32(0.2),
                            EaseKind::QuarticIn,
                            effect_intensity(0., 1.),
                        ),
                        tween(
                            Duration::from_secs_f32(1.),
                            EaseKind::QuarticIn,
                            effect_intensity(1., 0.0),
                        ),
                    )));
            }
        }
        _ => {}
    });
}

fn despawn_effect_system(
    mut commands: Commands,
    q_effect: Query<(), With<Effect>>,
    mut ended: EventReader<TimeRunnerEnded>,
) {
    ended.read().for_each(|ended| {
        if ended.is_completed() && q_effect.contains(ended.time_runner) {
            commands.entity(ended.time_runner).despawn_recursive();
        }
    });
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
