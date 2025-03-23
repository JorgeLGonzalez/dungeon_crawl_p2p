use crate::{
    dungeon::RespawnState,
    items::{Grabbable, MagicItem},
    monsters::Monster,
    player::MoveThrottle,
    prelude::*,
    startup::checksum_transform,
};
use bevy_ggrs::RollbackFrameCount;

/// Log debugging info for GGRS. Note some of these lines (i.e. those that contain
/// a '|') are parsed by the `ggrs-utils` project/program.
pub fn debug_ggrs(
    items: Query<(Entity, &MagicItem, &Transform, &Grabbable), With<MagicItem>>,
    monsters: Query<(Entity, &Health, &Monster, &Transform), With<Monster>>,
    players: Query<(Entity, &Health, Option<&MoveThrottle>, &Player, &Transform), With<Player>>,
    frame: Res<RollbackFrameCount>,
    respawn: Res<RespawnState>,
    rng: Res<RandomGenerator>,
    state: Res<State<GameState>>,
) {
    let frame = frame.0;

    info!(
        "Frame={frame} State={state:?} Respawn={respawn:?} rng={}",
        rng.counter
    );

    let mut sorted_items: Vec<_> = items.iter().collect();
    sorted_items.sort_by_key(|(entity, _, _, _)| format!("{entity}"));
    for (entity, item, transform, _) in sorted_items {
        let item = item.label();
        let pos = transform.translation.truncate().as_ivec2();
        let transform_check = checksum_transform(transform);
        info!(
            "Frame={frame}|Item={item}|Entity={entity}|Pos={pos}|\
        Transform={transform_check:X}"
        );
    }

    info!("Frame={frame} Total Items={}", items.iter().count());

    let mut sorted_players: Vec<_> = players.iter().collect();
    sorted_players.sort_by_key(|(entity, _, _, _, _)| format!("{entity}"));
    for (entity, health, move_throttle, player, transform) in sorted_players {
        let player_id = player.id;
        let pos = transform.translation.truncate().as_ivec2();
        let health = health.current;
        let throttle = move_throttle.map(|t| t.elapsed_secs()).unwrap_or(0.0);
        let transform_check = checksum_transform(transform);
        info!(
            "Frame={frame}|Player={player_id}|Entity={entity}|Pos={pos}|\
            Health={health}|Throttle={throttle}|Transform={transform_check:X}"
        );
    }
    info!("Frame={frame} Total Players={}", players.iter().count());

    let mut sorted_monsters: Vec<_> = monsters.iter().collect();
    sorted_monsters.sort_by_key(|(entity, _, _, _)| format!("{entity}"));
    for (entity, health, monster, transform) in sorted_monsters {
        let monster = monster.name();
        let pos = transform.translation.truncate().as_ivec2();
        let health = health.current;
        let transform_check = checksum_transform(transform);
        info!(
            "Frame={frame}|Monster={monster}|Entity={entity}|Pos={pos}|\
        Health={health}|Transform={transform_check:X}"
        );
    }
    info!("Frame={frame} Total Monsters={}", monsters.iter().count());
}
