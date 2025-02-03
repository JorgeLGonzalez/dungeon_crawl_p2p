# GGRS

Remember to test with `GameMode::GgrsSyncTest` to see if things are still working.
Last tested on 2025-02-03.

## Rollbacks

GGRS depends on rollbacks. We need to do at least two things:

1. Call `add_rollback` on the spawn command for each entity bundle that may need rollbacks. This adds the Bevy GGRS `Rollback` component.
2. Register specific components for rollback snapshots with a specific strategy (clone or copy) in `main` via the `rollback_component_with_clone` or `rollback_component_with_copy` methods. This tells Bevy GGRS to store a snapshot of these for every frame (discarding them as they lose utility).
3. If the resource or component is critical to the state, add it to the frame checksum via the `checksum_component` or `checksum_resource` App method. This way it will be included in the frame checksum used to detect if clients fall out of sync.

So for `Player` for example (in [spawn_players](../../player/spawn_players.rs)) we use `add_rollback` and in main we register `Player` and `Transform` for rollback. We probably will need to add other Components that are added to Player when we add Sprite etc, especially if a Player will be despawned.

See [Extreme Bevy Detecting Desyncs tutorial](https://johanhelsing.studio/posts/extreme-bevy-desync-detection) for more info

Entities Spawned and Rollback Info

- `spawn_players`. With rollback.
- `spawn_camera`. No rollback because it only follows the local player.
- `spawn_dungeon`. Tiles have no rollback as they are only generated at the start of each dungeon level and we rely on the shared random seed to have both players get the same dungeon.

### Time

Note that `bevy_ggrs` replaces the `Time` resource with one that is kept in sync.

### Checksums

The rollback depends on checksums. Transform is a special case where we need to create a custom checksum, which is why we have the `checksum_component` method in `main`. So is `RandomGenerator`. Note that GGRS will not complain about rollback components or resources that cannot be hashed. I guess they will just not be checked to ensure they are in sync. (But I could be wrong...)

## Understanding GGRS

Some key notes:

1. The game must be deterministic. For example, the random number generator must start with the same seed for each player. This way all we need to worry about is sending the player inputs to each client.
2. When the inputs fail to match the GGRS predictions, it will rollback components and resources. These mus be registered in two ways a) the `add_rollback` method when spawning the entity will add a `Rollback` component, which is what `bevy_ggrs` uses to know what needs rolling back. b) the `rollback_component_*` and `rollback_resource_*` methods on the App will create `GgrsSnapshots` resources for each registered Type. This keeps track of snapshots per frame (a short set of frames). The methods indicate the strategy for saving/loading snapshots (clone, copy).
3. Adding a checksum on a component or resource ensures it is part of the overall frame checksum used to ensure state is un sync. So basically, (I believe) the system gathers all entities that have a `Rollback` component and then adds any components on these entities to the state snapshots it keeps for each component/resource. Finally, it any components/resources with a checksum method are included in the overall checksum used to detect desync.
4. `bevy_ggrs` sits on top of `ggrs` and `ggrs` is an implementation of GGPO in Rust.

### Troubleshooting Desyncs

Since the PRs etc have not gotten any attention, I have developed another way which is:

1. Set `config::GGRS_DEBUG` to true.
2. Capture the output logged into a file called `p0.log` in the `ggrs-utils` project. When debugging MultiPlayer mode, save the second player's log as `p1.log`.
3. Run it ggrs-utils. This will parse the log(s) and generate useful output files around the mismatched frame.

There are two ways to auto-detect desync events:

1. Handle the `GgrsEvent::DesyncDetected`, which is generated when `DesyncDetection` is enabled in the P2P session. Enabling happens in `create_p2p_session` while handling happens in `handle_ggrs_events`. The handling tries to log the snapshots, but see [this issue](https://github.com/gschup/bevy_ggrs/issues/117) I logged.
2. Run the app in `synctest` mode. The problem here is that no event is generated so all you get is the fact that a desync happened when logged out. Not details.
3. Enable Trace and/or Debug level logging (via the bevy App LogPlugin config) as bevy_ggrs logs useful stuff out. (Can do RUST_DEBUG env var, but that gets SUPER noisy).

Despite the above, it took me over a week to figure out why the monster positions (Transforms) were getting out of sync. It was clear it was the monsters, but unclear why. I eventually theorized it was because the Bevy queries do not return entities in the same order every single time, which means one client can iterate through the list of monsters and move monster A while the other client picks monster B. Same move, different monster. (I could see this happening once I started tracking, logging and saving monster moves.)
(At least I HOPE that solved the bug!)

## References

The P2P stuff (among other things) is based on the [Extreme Bevy tutorial](https://johanhelsing.studio/posts/extreme-bevy)

## Debugging Notes

### January 26, 2025

The issue this time was that using a healing potion occured in one frame, but drinking it occured in the next frame. This is because the `drink_potion` system is part of the `health` module which was set to run BEFORE the player systems. So the relevant sequence was:

1. Frame F: Player uses a healing potion. Drink event is sent, but not processed yet.
2. Frame F + 1: Drink event is processed so player heals.
3. GGRS detects a prediction failure and rolls back to frame F.
4. Frame F + 1 is run again, but there is no use item so therefore no drink event so therefore no healing.

The solution is to move the `drink_potion` system to run AFTER the player systems and BEFORE the monsters, to ensure a deterministic sequence.

ASIDE: Also found that the spawning of player, monster and item entities was non-deterministic. This meant that entity IDs were inconsistent between clients. Fixed this by adding criteria to when these entities are spawned.
