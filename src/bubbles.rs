use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient, ParticleEffect,
    ParticleEffectBundle, SetAttributeModifier, SizeOverLifetimeModifier, Spawner, WriterExpr,
};

const BUBBLE_INTERVAL_SECONDS: f32 = 5.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, bubbles_setup_system).add_systems(
        Update,
        gravel_bubbles_mover.run_if(on_timer(Duration::from_secs_f32(
            BUBBLE_INTERVAL_SECONDS * 0.5,
        ))),
    );
}

#[derive(Resource, Deref)]
pub struct BubblesEffect(Handle<EffectAsset>);

#[derive(Component)]
pub struct GravelBubbler;

fn bubbles_setup_system(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let writer = ExprWriter::new();

    let init_size = SetAttributeModifier {
        attribute: Attribute::SIZE,
        value: writer.lit(0.01).expr(),
    };

    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(3.).expr());

    let init_pos = SetAttributeModifier {
        attribute: Attribute::POSITION,
        value: WriterExpr::vec3(writer.lit(0.0), writer.lit(0.0), writer.lit(0.0)).expr(),
    };

    let init_vel = SetAttributeModifier {
        attribute: Attribute::VELOCITY,
        value: WriterExpr::vec3(writer.lit(0.0), writer.lit(0.2), writer.lit(0.0)).expr(),
    };

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0.9, 0.9, 1.0, 0.8));
    color_gradient.add_key(0.7, Vec4::new(0.8, 0.8, 1.0, 0.7));
    color_gradient.add_key(1.0, Vec4::new(0.8, 0.8, 1.0, 0.0));

    let update_color = ColorOverLifetimeModifier {
        gradient: color_gradient,
    };

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.05));
    size_gradient.add_key(0.3, Vec2::splat(0.05));
    size_gradient.add_key(1.0, Vec2::splat(0.02));

    let update_size = SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false,
    };

    let bubbles_effect = effects.add(
        EffectAsset::new(
            vec![32768],
            Spawner::rate((1. / BUBBLE_INTERVAL_SECONDS).into()),
            writer.clone().finish(),
        )
        .init(init_size)
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(update_size.clone())
        .render(update_color.clone()),
    );

    commands.spawn((
        GravelBubbler,
        ParticleEffectBundle {
            effect: ParticleEffect::new(bubbles_effect.clone()),
            transform: Transform::from_xyz(0., -1.7, 0.),
            ..default()
        },
    ));

    commands.insert_resource(BubblesEffect(bubbles_effect));
}

fn gravel_bubbles_mover(
    time: Res<Time>,
    mut gravel_bubbler: Query<&mut Transform, With<GravelBubbler>>,
) {
    let mut transform = gravel_bubbler.single_mut();

    transform.translation.x = 1.7 * time.elapsed_seconds().sin();
}
