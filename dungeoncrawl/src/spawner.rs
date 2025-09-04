use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player, // tag component indicating entity is a player
        position,
        Render {
            color: ColorPair::new(BLUE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    ecs.push((
        Enemy, // tag component indicating entity is an enemy
        position,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}
