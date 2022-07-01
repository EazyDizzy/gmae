use crate::creature::component::attack::Attack;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::{MovementStrategy, CREATURE_MOVED_LABEL};
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::dummy::Dummy;
use crate::creature::pizza::Pizza;
use crate::player::entity::Player;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use lib::entity::level::creature::CreatureName;
use lib::entity::level::Level;
use lib::entity::point::Point;
use std::f32::consts::PI;
use crate::creature::component::attack::util::raytracer::get_last_seen_point;

pub mod component;
pub mod dummy;
pub mod pizza;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(creatures_execute_move_strategies)
                    .label(CREATURE_MOVED_LABEL),
            )
            .add_system(
                creatures_apply_gravity
                    .after(CREATURE_MOVED_LABEL)
                    .before(creature_move_model),
            )
            .add_system(creature_move_model.after(CREATURE_MOVED_LABEL))
            .add_system(creatures_show_direction_of_sight);
    }
}

fn spawn_creatures(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    let dummy_mesh = asset_server.load("mesh/dummy.glb#Scene0");
    let pizza_mesh = asset_server.load("mesh/pizza.glb#Scene0");

    for creature in level.creatures() {
        let mut ec = commands.spawn_bundle((
            Transform::from_xyz(
                creature.position.x,
                creature.position.y,
                creature.position.z,
            )
            //     TODO remove default rotation after debug
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
            //     TODO make sth to avoid this
            .with_scale(vec3(0.5, 0.5, 0.5)),
            GlobalTransform::identity(),
        ));
        ec.with_children(|parent| {
            let mesh = match creature.name {
                CreatureName::Dummy => dummy_mesh.clone(),
                CreatureName::Pizza => pizza_mesh.clone(),
            };
            parent.spawn_scene(mesh);
        })
        .insert(CreatureMarker {});

        if creature.is_enemy() {
            ec.insert(EnemyCreatureMarker {});
        }

        match creature.name {
            CreatureName::Dummy => {
                ec.insert(Dummy::new());
                dummy::insert(&mut ec, creature);
            }
            CreatureName::Pizza => {
                ec.insert(Pizza::new());
                pizza::insert(&mut ec, creature);
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct CreatureMarker {}
#[derive(Component, Debug)]
pub struct EnemyCreatureMarker {}

fn creatures_execute_move_strategies(
    lvl: Res<Level>,
    mut query: Query<(
        &mut Locomotivity,
        &PhysiologyDescription,
        &mut MovementStrategy,
        With<CreatureMarker>,
    )>,
) {
    for (mut locomotivity, phys, mut move_strat, ..) in query.iter_mut() {
        move_strat.update(&mut locomotivity, phys, &lvl);
    }
}

fn creature_move_model(mut query: Query<(&mut Transform, &Locomotivity)>) {
    for (mut transform, locomotivity) in query.iter_mut() {
        transform.translation = vec3(
            locomotivity.position().x,
            locomotivity.position().y,
            locomotivity.position().z,
        );
    }
}

fn creatures_apply_gravity(
    lvl: Res<Level>,
    mut query: Query<(&mut Locomotivity, &PhysiologyDescription)>,
) {
    for (mut locomotivity, phys) in query.iter_mut() {
        locomotivity.gravity_move(&lvl, phys);
    }
}

fn creatures_show_direction_of_sight(
    mut lines: ResMut<DebugLines>,
    lvl: Res<Level>,
    player_query: Query<(&Locomotivity, With<Player>)>,
    enemy_query: Query<(
        &Locomotivity,
        &Transform,
        &PhysiologyDescription,
        &Attack,
        With<EnemyCreatureMarker>,
    )>,
) {
    if let Some(player) = player_query.iter().next() {
        let (player_locomotivity, ..) = player;
        let mut player_position: Point = player_locomotivity.position().clone();
        player_position.add_y(1.0);

        for (locomotivity, transform, phys, attack, ..) in enemy_query.iter() {
            let pos: &Point = locomotivity.position();
            let eyes_pos: Point = phys.get_eyes_position(transform, pos);
            let start = eyes_pos.into_vec3();
            let duration = 0.0; // Duration of 0 will show the line for 1 frame.
            let last_seen_point = get_last_seen_point(&eyes_pos, &player_position, &lvl);
            let end = last_seen_point.unwrap_or_else(|| player_position.clone()).into_vec3();
            lines.line_colored(start, end, duration, Color::RED);

            attack.exec(
                phys,
                locomotivity,
                transform,
                &player_position,
                &lvl,
            );
        }
    }
}
