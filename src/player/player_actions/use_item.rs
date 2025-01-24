use super::UseItemEvent;
use crate::prelude::*;

pub fn use_item(mut use_item_event: EventReader<UseItemEvent>) {
    use_item_event.read().for_each(|event| {
        info!("Use item event: {:?}", event.item_index);
    });
}
