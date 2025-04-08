use bevy::prelude::*;
use bevy_tween::prelude::Interpolator;
use bevy_tween::tween::ResourceTween;
use bevy_tween::{BevyTweenRegisterSystems, resource_tween_system};
use rand::Rng;

#[derive(Default, Resource)]
pub struct EffectIntensitiy(f32);

pub struct EffectIntensity {
    pub start: f32,
    pub end: f32,
}

impl Interpolator for EffectIntensity {
    type Item = EffectIntensitiy;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.0 = self.start.lerp(self.end, value)
    }
}

pub fn effect_intensity(start: f32, end: f32) -> ResourceTween<EffectIntensity> {
    ResourceTween::new(EffectIntensity { start, end })
}

pub fn custom_interpolators_plugin<C>(app: &mut App)
where
    C: Component,
{
    app.add_tween_systems(resource_tween_system::<EffectIntensity>());
    app.init_resource::<EffectIntensitiy>();
    app.add_systems(Update, big_x_do_effect::<C>);
}

fn big_x_do_effect<C: Component>(
    effect_intensity: Res<EffectIntensitiy>,
    mut q_big_x: Query<&mut Transform, With<C>>,
) {
    let mut rng = rand::thread_rng();
    let dx: f32 = rng.random();
    let dy: f32 = rng.random();
    q_big_x.single_mut().translation.x = dx * effect_intensity.0;
    q_big_x.single_mut().translation.y = dy * effect_intensity.0;
}
