use crate::cards::Card;
use bevy::app::App;
use bevy::math::ops::{cos, sin};
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use bevy_tween::prelude::*;
use bevy_tween::tween::AnimationTarget;
use std::f32::consts::PI;

#[derive(Component, Debug, Default)]
pub struct HandCard;

#[derive(Component)]
pub struct CardPlane;

#[derive(Event, Debug)]
pub struct HandCardChanged;

pub struct HandCardPlugin;

impl Plugin for HandCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HandCardChanged>();
        app.add_systems(Update, change_hand_cards);
    }
}

pub fn change_hand_cards(
    mut commands: Commands,
    mut hand_card_changed: EventReader<HandCardChanged>,
    mut cards: Query<(Entity, &mut Transform, &mut Card), With<HandCard>>,
    mut card_plane: Query<&Transform, (With<CardPlane>, Without<Card>)>,
) {
    for _ in hand_card_changed.read() {
        let num = cards.iter().len();
        if num > 0 {
            if let tr = card_plane.single() {
                let hand_positions =
                    calculate_hand_positions(num, 0.0, 200., PI / 4., tr.translation.z, -6.7);
                let mut list: Vec<_> = cards.iter_mut().collect();
                list.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());
                list.iter_mut()
                    .enumerate()
                    .for_each(|(index, &mut (ref mut entity, ref mut transform, ref mut card))| {
                        let target = AnimationTarget.into_target();
                        let mut start = target.transform_state(transform.clone());
                        if let Some(tr_end) = hand_positions.get(index) {
                            commands.entity(*entity).animation().insert_tween_here(
                                Duration::from_secs_f32(0.2),
                                EaseKind::ExponentialOut,
                                start.translation_to(tr_end.clone().translation),
                            );
                            card.trans = tr_end.clone();
                        }
                    })
            }
        }
    }
}

/// 计算手牌位置
pub fn calculate_hand_positions(
    card_count: usize,
    center_x: f32,
    base_radius: f32,
    max_angle: f32,
    z: f32,
    base_y: f32,
) -> Vec<Transform> {
    // 不处理card_count == 0 的情况
    let mut vec = Vec::new();

    let mut radius = base_radius + (card_count - 1) as f32 * 10.0;
    let total_angle = max_angle.min(10.0 * card_count as f32);
    let angle_step = total_angle / 1.0_f32.max((card_count - 1) as f32);
    let start_angle = -total_angle / 2.;

    for i in 0..card_count {
        let angle = start_angle + i as f32 * angle_step;
        let radian = angle.to_radians();
        let x = center_x + radius * sin(radian);
        let mut y = base_y + radius * (1.0_f32 - cos(radian));
        y += radius * 0.1 * (1. - cos(2. * radian));
        let rotation = -angle;
        vec.push(
            Transform::from_xyz(x, y, z - 0.001 * i as f32), // .with_rotation(Quat::from_axis_angle(Vec3::Z, rotation)),
        );
    }
    vec
}
