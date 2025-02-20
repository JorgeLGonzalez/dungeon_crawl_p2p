use super::*;
use crate::{player::PlayerCamera, prelude::*};
use bevy::window::PrimaryWindow;

pub type CameraQuery<'w, 's, 'c, 'g> =
    Query<'w, 's, (&'c Camera, &'g GlobalTransform), With<PlayerCamera>>;

pub type PlayerQuery<'w, 's, 'p, 'f, 't> =
    Query<'w, 's, (&'p Player, &'f FieldOfView, &'t Transform)>;

pub type TooltipEntityQuery<'w, 's, 'l, 't> =
    Query<'w, 's, (Entity, &'l TooltipLabel, &'t Transform)>;

pub type TooltipUIQuery<'w, 's, 'n, 't, 'u> = Query<'w, 's, (&'n Node, &'t Text, &'u TooltipUI)>;

pub type TooltipUIMutQuery<'w, 's, 'n, 't, 'u> =
    Query<'w, 's, (&'n mut Node, &'t mut Text, &'u mut TooltipUI)>;

pub type WindowQuery<'w, 's, 'wnd> = Query<'w, 's, &'wnd Window, With<PrimaryWindow>>;
