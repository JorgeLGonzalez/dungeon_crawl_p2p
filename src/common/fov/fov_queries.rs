use crate::prelude::*;

pub type FovQuery<'w, 's, 'f, 'p> =
    Query<'w, 's, (&'f mut FieldOfView, Option<&'p Player>), With<FieldOfView>>;
