use bevy::prelude::*;
use bevy_hanabi::*;

#[allow(clippy::module_name_repetitions)]
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(particle_setup);
    }
}
#[derive(Component)]
pub struct PunchEffect;

fn particle_setup(mut effects: ResMut<Assets<EffectAsset>>, mut commands: Commands) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(0.3, Vec4::new(0., 1., 0., 1.));
    gradient.add_key(0.6, Vec4::new(0., 0., 1., 1.));
    gradient.add_key(1.0, Vec4::ZERO);

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.2));
    size_gradient.add_key(1.0, Vec2::splat(0.0));

    let effect = effects.add(
        EffectAsset {
            name: "PunchEffect".to_string(),
            capacity: 100,
            spawner: Spawner::once(10.0.into(), false),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.0,
            dimension: ShapeDimension::Volume,
            speed: 10.0.into(),
        })
        .init(ParticleLifetimeModifier { lifetime: 0.2 })
        .update(AccelModifier {
            accel: Vec3::new(0., 3., 0.),
        })
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
        }),
    );

    commands
        .spawn_bundle(ParticleEffectBundle::new(effect))
        .insert(PunchEffect);
}
