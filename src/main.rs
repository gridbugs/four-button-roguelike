extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate cgmath;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate entity_store_helper;

// public so we can generate documentation
pub mod entity_store { include_entity_store!("entity_store.rs"); }

use cgmath::vec2;
use entity_store::{EntityStore, SpatialHashTable, insert};

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

fn main() {
    let mut entity_store = EntityStore::new();
    let mut spatial_hash = SpatialHashTable::new(WIDTH, HEIGHT);

    let mut changes = Vec::new();
    changes.push(insert::coord(0, vec2(0, 0)));
    changes.push(insert::solid(0));
    changes.push(insert::tile(0, '@'));

    for change in changes.drain(..) {
        spatial_hash.update(&entity_store, &change, 0);
        entity_store.commit(change);
    }
}
