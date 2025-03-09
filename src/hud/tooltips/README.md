# Tooltip

A tooltip is displayed (for the local player) over a relevant entity (i.e. an entity with the `TooltipLabel` component) when the mouse pointer hovers over it or when the local player steps on the item. We use a UI entity (marked with the `TooltipUI` component) to render the tooltip. The `TooltipUI` marker component specifies the game entity (i.e. magic item, monster, or player) that has the tooltip, when the tooltip is visible. The `TooltipLabel` component specifies the tooltip text string to use for the tooltip.

## Toggling

Tooltip visibility is toggled on/off based on events that are configured to run systems via the `TooltipPlugin`:

- `PlayerMovesEvent` triggers [on_player_move](./on_player_move.rs)
- `MonsterMovesEvent` triggers [on_monster_move](./on_monster_move.rs)
- Mouse `CursorMoved` triggers [on_mouse_move](./on_mouse_move.rs)
- Dungeon `ZoomEvent` triggers [on_zoom](./on_zoom.rs)
- Entering `GameState::DungeonSpawning` triggers [on_exit_level](./on_exit_level.rs)

All these systems create a [TooltipToggleTrigger](./toggle/trigger.rs) observable trigger event observed by the [toggle_tooltip](./toggle_tooltip.rs) system, which ultimately shows or hides the tooltip based on the `TooltipToggleTrigger` variant.

### on_player_move

If the local player moves, we check if they have moved onto an entity that supports a tooltip and show or hide the tooltip as needed. If the move was for the remote player who had a tooltip on its sprite, we hide it since we assume the player moved out from under the mouse cursor.

### on_monster_move

When a monster that had a visible tooltip moves, we hide the tooltip since we assume the monster moved out from under the tooltip.

### on_mouse_move

This system toggles the tooltip visibility based on mouse movement. We want to avoid the more expensive check against all entity positions, so we try to short-circuit that as follows:

1. If the mouse game position is not in the local player's FOV, we know no tooltip should be visible, so we hide it if necessary. (Even if the tooltip was shown because the player moved onto the item, we consider a mouse move a toggling event, just to keep things a bit simpler.)
2. If the mouse position is unavailable, we also hide the tooltip (if visible)
3. If the tooltip is visible and the mouse is still on it, we do NOT toggle.

Finally, if we get through all that, we then check if the mouse is over any entity that supports a tooltip and show the tooltip. If none are found, we hide the tooltip if it is visible.

### on_exit_level

This is not a Bevy event. Rather, when any player exits a Dungeon level, the game state changes to `GameState::DungeonSpawning`, which triggers this system, which simply hides any visible tooltip for either player (since both players change levels simultaneously).

### Coordinate Systems

We need to deal with 3 coordinate systems:

- **Game**. The 2D game coordinates is where the tooltip-able and player entities live and where we use the `PlayerCamera`. The origin here is in the center and the scale may differ based on player-controlled zoom level.
- **Screen**. The mouse cursor screen or viewport coordinates are unrelated to the `PlayerCamera`'s scale and have an origin at the top left.
- **HUD**. The HUD coordinates are based on the `HudCamera`, which is also independent of the `PlayerCamera` and I think the origin is at the bottom left.

So to check mouse position, we convert screen coordinates to game coordinates. And to display the tooltip, we convert from game coordinates to HUD UI coordinates.

### on_zoom

When the the local player zooms the dungeon map in or out, we hide the visible tooltip since it is likely the mouse cursor will no longer be on the post-zoom entity. (This is not true when the tooltip was displayed because the player stepped on the entity, but whatever...)
