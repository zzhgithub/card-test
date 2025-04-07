use crate::cases::CaseZone;
use crate::spawn_ui_popup;
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_tween::combinator::{
    event, event_for, parallel, sequence, tween, TransformTargetStateExt,
};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{AnimationBuilderExt, IntoTarget};
use bevy_tween::tween::AnimationTarget;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Default, Component, Clone, Debug)]
pub struct Card {
    pub trans: Transform,
}

#[derive(Component, Debug)]
pub struct CardInfo {}
// 生成闭包的模板

#[derive(Component, Debug)]
pub struct Setted;

pub fn gen_put_card<C>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
    radius: f32,
    thick: f32,
) -> impl FnMut(Handle<Image>, Transform) -> Entity
where
    C: Component,
{
    move |images: Handle<Image>, transform: Transform| {
        let mesh_list = gen_card_mesh_list(meshes, width, height, radius, thick);

        commands
            .spawn((
                Card {
                    trans: transform.clone(),
                },
                Visibility::Inherited,
                transform,
                AnimationTarget,
            ))
            .with_children(|parent| {
                // 加载黑色边框
                for (mesh_handle, trans) in mesh_list.0 {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(Color::BLACK)),
                    ));
                }
                // 加载内容
                for (mesh_handle, trans) in mesh_list.1 {
                    parent.spawn((
                        CardInfo {
                            // todo 这里是卡片的信息内容
                        },
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: Some(images.clone()),
                            alpha_mode: AlphaMode::Blend,
                            unlit: true,
                            ..Default::default()
                        })),
                    ));
                }
                // 背面
                for (mesh_handle, trans) in mesh_list.2 {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: Some(images.clone()),
                            alpha_mode: AlphaMode::Blend,
                            ..Default::default()
                        })),
                    ));
                }
            })
            .observe(move_on_drag::<C>())
            .observe(drag_start)
            .observe(drag_end)
            .observe(over_card)
            .observe(out_card)
            .id()
    }
}

// 处理拖拽到的代码
pub fn deal_on_drop(
    drag_drop: Trigger<Pointer<DragDrop>>,
    mut query: Query<&mut CaseZone>,
    mut card_info_query: Query<&mut CardInfo>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut card_q: Query<&mut Card, Without<Setted>>,
    mut p_q: Query<&Parent, With<CardInfo>>,
) {
    // 场地的值？ TODO 这处理
    // info!("{:?}", drag_drop);

    let case_zone = query.get_mut(drag_drop.target).unwrap();
    let end = case_zone.clone().transform.translation;
    // info!("{:?}", case_zone);
    if let Ok(y) = card_info_query.get(drag_drop.dropped) {
        // todo
    }
    //todo 处理内部的场地和卡片的关系
    // info!("{:?}", y);
    if let Ok(parent) = p_q.get(drag_drop.dropped) {
        if let Ok(mut card) = card_q.get_mut(parent.get()) {
            let p_clone = parent.get().clone();
            // 设置卡片被放置了
            let mut card_clone = card.clone();
            let card_end = Card {
                trans: Transform::from_translation(end.clone()),
            };
            spawn_ui_popup(
                &mut commands,
                &asset_server,
                "是否登场?",
                move |cmd, ch_q| {
                    let target = AnimationTarget.into_target();
                    let mut start = target.transform_state(card_clone.clone().trans);
                    // // todo 这里应该一系列动画
                    // card_clone.trans.translation = end.clone();
                    let mut mid = Vec3::ZERO;
                    mid.z = card_clone.trans.translation.z;

                    let mut mid2 = Vec3::ZERO;
                    mid2.z = card_clone.trans.translation.z + 3.0;
                    info!("{:?}", card_clone.clone().trans.translation);
                    info!("{:?}", mid);
                    info!("{:?}", end);
                    let mut mid_state = target.transform_state(Transform::from_translation(mid));
                    let mut mid_state2 = target.transform_state(Transform::from_translation(mid));
                    info!("add tween");
                    cmd.entity(p_clone)
                        .animation()
                        .clear_on_finish()
                        .insert(sequence((
                            tween(
                                Duration::from_secs_f32(1.0),
                                EaseKind::ExponentialOut,
                                start.translation_to(mid),
                            ),
                            tween(
                                Duration::from_secs_f32(1.0),
                                EaseKind::ExponentialOut,
                                mid_state.translation_to(mid2),
                            ),
                            tween(
                                Duration::from_secs_f32(0.6),
                                EaseKind::ExponentialOut,
                                mid_state2.translation_to(end),
                            ),
                            parallel((event("boom"), event("shark"))),
                        )))
                        .insert(card_end.clone())
                        .insert(Setted);

                    // 恢复自由身体
                    if let Ok(children) = ch_q.get(p_clone) {
                        for child in children.iter() {
                            cmd.entity(child.clone()).remove::<PickingBehavior>();
                        }
                    }
                    info!("确认");
                },
                move |cmd| {
                    info!("取消");
                },
            );
        }
    }
}

#[derive(Component, Debug)]
pub struct Dragging;

pub fn over_card(
    out: Trigger<Pointer<Over>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card), (Without<Dragging>, Without<Setted>)>,
) {
    if let Ok(parent) = query.get(out.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());
            let mut end = tr.clone().translation;
            end.y = -2.0;
            info!("over");
            info!("{:?}", tr);
            info!("{:?}", end.clone());
            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(end),
            );
        }
    }
}

pub fn out_card(
    out: Trigger<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card), (Without<Dragging>, Without<Setted>)>,
) {
    if let Ok(parent) = query.get(out.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());
            info!("back");
            info!("{:?}", tr.clone());
            info!("{:?}", card);
            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(card.trans.translation),
            );
        }
    }
}

pub fn drag_start(
    drag_start: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    query: Query<(), With<CardInfo>>,
    query_parent: Query<&Parent>,
    card_query: Query<&Card, Without<Setted>>,
) {
    if query.get(drag_start.target).is_ok() {
        commands
            .entity(drag_start.target)
            .insert(PickingBehavior::IGNORE);
    }
    // 添加拖拽中的组件
    if let Ok(parent) = query_parent.get(drag_start.target) {
        if let Ok(_card) = card_query.get(parent.get()) {
            commands.entity(parent.get()).insert(Dragging);
        }
    }
}

pub fn drag_end(
    drag_start: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card), Without<Setted>>,
) {
    info!("Drag END {:?}", drag_start.target);
    // 发送回到原来位置的命令
    if let Ok(parent) = query.get(drag_start.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());

            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(card.trans.translation),
            );
            // 删除拖拽中的组件
            commands.entity(parent.get()).remove::<Dragging>();
        }
    }
    commands
        .entity(drag_start.target)
        .remove::<PickingBehavior>();
}

// 在这里个方法里 还可以做其他的事情 比如通知全局现在要选择
pub fn move_on_drag<C>() -> impl Fn(
    Trigger<Pointer<Drag>>,
    Query<&mut Transform, Without<Setted>>,
    Single<(&Camera, &GlobalTransform)>,
    Single<&Window>,
    Single<&GlobalTransform, With<C>>,
)
where
    C: Component,
{
    move |drag, mut transforms, camera_query, windows, ground| {
        // 这个是需要修改的值
        if let Ok(mut transform) = transforms.get_mut(drag.entity()) {
            let (camera, camera_transform) = *camera_query;

            let Some(cursor_position) = windows.cursor_position() else {
                info!("a");
                return;
            };

            // Calculate a ray pointing from the camera into the world based on the cursor's position.
            let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                info!("b");
                return;
            };

            // Calculate if and where the ray is hitting the ground plane.
            let Some(distance) =
                ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
            else {
                info!("c");
                return;
            };
            let point = ray.get_point(distance);
            // info!("{:?}", point);
            transform.translation.x = point.x;
            transform.translation.y = point.y;
        }
    }
}

fn gen_card_mesh_list(
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
    radius: f32,
    thick: f32,
) -> (
    [(Handle<Mesh>, Transform); 8],
    [(Handle<Mesh>, Transform); 1],
    [(Handle<Mesh>, Transform); 1],
) {
    // 四个 扇形 四个长方形  一个中央的部分
    let a: f32 = width - 2.0 * radius;
    let b: f32 = height - 2.0 * radius;

    // 四个角的坐标
    let right_top = Transform::from_xyz(a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4.));
    let right_bottom = Transform::from_xyz(a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4. - PI / 2.));

    let left_top = Transform::from_xyz(-a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4.));

    let left_bottom = Transform::from_xyz(-a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4. + PI / 2.));

    // 四个边框的坐标
    let right = Transform::from_xyz((a + radius) / 2.0, 0.0, 0.0);
    let left = Transform::from_xyz(-(a + radius) / 2.0, 0.0, 0.0);
    let top = Transform::from_xyz(0.0, (b + radius) / 2.0, 0.0);
    let bottom = Transform::from_xyz(0.0, -(b + radius) / 2.0, 0.0);
    // 中心的坐标
    let center = Transform::from_xyz(0.0, 0.0, thick / 2.);
    let back = Transform::from_xyz(0.0, 0.0, -thick / 2.)
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI));
    // 加载一组的shape

    let frames = [
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_bottom,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            top,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            left,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            right,
        ),
    ];

    // 正面主要
    let content = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), center)];
    let back_side = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), back)];

    (frames, content, back_side)
}

#[derive(Component)]
pub struct ClearOnFinish;

trait ClearOnFinishExt {
    fn clear_on_finish(self) -> Self;
}

impl ClearOnFinishExt for bevy_tween::combinator::AnimationBuilder<'_> {
    fn clear_on_finish(mut self) -> Self {
        self.entity_commands().insert(ClearOnFinish);
        self
    }
}

pub fn clear_on_finish_system(
    mut commands: Commands,
    mut time_runner_finished: EventReader<bevy_tween::bevy_time_runner::TimeRunnerEnded>,
    has_clear_on_finish: Query<Has<ClearOnFinish>>,
    q_children: Query<&Children>,
    q_tween: Query<(Entity, Has<bevy_tween::bevy_time_runner::TimeSpan>)>,
) {
    for time_runner in time_runner_finished.read() {
        if has_clear_on_finish
            .get(time_runner.time_runner)
            .unwrap_or(false)
        {
            let Ok(children) = q_children.get(time_runner.time_runner) else {
                continue;
            };
            for (entity, is_tween) in q_tween.iter_many(children) {
                if is_tween {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
