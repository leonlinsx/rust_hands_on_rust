use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player, // tag component indicating entity is a player
        position,
        Render {
            color: ColorPair::new(BLUE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    // destructure data out of tuple
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy, // tag component indicating entity is an enemy
        position,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph,
        },
        MovingRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name), // different because of tuple struct
    ));

    // hitpoint, name, ascii char reference
    fn goblin() -> (i32, String, FontCharType) {
        (1, "Goblin".to_string(), to_cp437('g'))
    }

    fn orc() -> (i32, String, FontCharType) {
        (2, "Orc".to_string(), to_cp437('o'))
    }
}
