mod template;

use crate::prelude::*;
use template::*;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player { map_level: 0 }, // tag component indicating entity is a player
        position,
        Render {
            color: ColorPair::new(BLUE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 100,
            max: 100,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_grail(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        Grail,
        position,
        Render {
            color: ColorPair::new(GOLD, BLACK),
            glyph: to_cp437('|'),
        },
        Name("The Holy Grail".to_string()),
    ));
}
