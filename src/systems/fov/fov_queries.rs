use crate::components::{FieldOfView, Player};
use bevy::prelude::{Query, With};

pub type FovQuery<'w, 's, 'f, 'p> =
    Query<'w, 's, (&'f mut FieldOfView, Option<&'p Player>), With<FieldOfView>>;
