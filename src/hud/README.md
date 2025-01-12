# HUD

The HUD at this point is just the a bar at the top. It is rendered using Bevy UI, so it uses screen coordinates rather than world coordinates. And it uses `Display::Flex` as the layout manager. This all makes it really easy to display text and the health bar and align them etc.

Noteworthy is that it uses a different camera, the `HudCamera`. This ensures any scaling, movement etc on the `PlayerCamera` does not affect the `HudCamera`. They render to different `RenderLayers` with the HUD camera having a higher order. (In fact, Bevy UI defaults to rendering on the highest layer, but we specify the render layer anyway to be more explicit and clear how the HUD relates to the HudCamera.)

## Tooltips

See [tooltips README](./tooltips/README.md).
