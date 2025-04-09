use bevy::math::ops::{cos, sin};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Component, Transform};


#[derive(Component,Debug,Default)]
pub struct HandCard;

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
