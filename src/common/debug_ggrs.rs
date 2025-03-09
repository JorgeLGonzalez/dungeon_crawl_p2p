use crate::{
    items::{Grabbable, MagicItem},
    monsters::Monster,
    player::MoveThrottle,
    prelude::*,
};
use bevy_ggrs::RollbackFrameCount;

/// Log debugging info for GGRS. Note some of these lines (i.e. those that contain
/// a '|') are parsed by the `ggrs-utils` project/program.
pub fn debug_ggrs(
    items: Query<(Entity, &MagicItem, &Transform, &Grabbable), With<MagicItem>>,
    monsters: Query<(Entity, &Health, &Monster, &Transform), With<Monster>>,
    players: Query<(Entity, &Health, Option<&MoveThrottle>, &Player, &Transform), With<Player>>,
    frame: Res<RollbackFrameCount>,
) {
    let frame = frame.0;

    for (entity, item, transform, _) in &items {
        let item = item.label();
        let pos = transform.translation.truncate().as_ivec2();
        info!("Frame={frame}|Item={item}|Entity={entity}|Pos={pos}");
    }
    info!("Frame={frame} Total Items={}", items.iter().count());

    for (entity, health, move_throttle, player, transform) in &players {
        let player_id = player.id;
        let pos = transform.translation.truncate().as_ivec2();
        let health = health.current;
        let throttle = move_throttle.map(|t| t.elapsed_secs()).unwrap_or(0.0);

        info!(
            "Frame={frame}|Player={player_id}|Entity={entity}|Pos={pos}|\
            Health={health}|Throttle={throttle}"
        );
    }
    info!("Frame={frame} Total Players={}", players.iter().count());

    for (entity, health, monster, transform) in &monsters {
        let monster = monster.name();
        let pos = transform.translation.truncate().as_ivec2();
        let health = health.current;

        info!("Frame={frame}|Monster={monster}|Entity={entity}|Pos={pos}|Health={health}");
    }
    info!("Frame={frame} Total Monsters={}", monsters.iter().count());
}
