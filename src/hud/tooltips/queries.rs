use crate::{
    fov::FieldOfView,
    hud::components::{HudCamera, TooltipLabel, TooltipUI},
    player::{Player, PlayerCamera},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub type CameraQuery<'w, 's, 'c, 'g> =
    Query<'w, 's, (&'c Camera, &'g GlobalTransform), With<PlayerCamera>>;

pub type HudCameraQuery<'w, 's, 'c, 't> =
    Query<'w, 's, (&'c Camera, &'t GlobalTransform), With<HudCamera>>;

pub type PlayerQuery<'w, 's, 'p, 'f> = Query<'w, 's, (&'p Player, &'f FieldOfView)>;

pub type TooltipEntityQuery<'w, 's, 'l, 't> =
    Query<'w, 's, (Entity, &'l TooltipLabel, &'t Transform)>;

pub type TooltipUIQuery<'w, 's, 'n, 't, 'u> =
    Query<'w, 's, (&'n mut Node, &'t mut Text, &'u mut TooltipUI)>;

pub type WindowQuery<'w, 's, 'wnd> = Query<'w, 's, &'wnd Window, With<PrimaryWindow>>;
