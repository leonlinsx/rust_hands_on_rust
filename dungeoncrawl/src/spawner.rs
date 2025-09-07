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
            current: 100,
            max: 100,
        },
        FieldOfView::new(8),
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
        ChasingPlayer {}, // TODO make some random movers
        Health {
            current: hp,
            max: hp,
        },
        Name(name),          // different because of tuple struct
        FieldOfView::new(6), // enemies have smaller fov than player
    ));

    // hitpoint, name, ascii char reference
    fn goblin() -> (i32, String, FontCharType) {
        (1, "Goblin".to_string(), to_cp437('g'))
    }

    fn orc() -> (i32, String, FontCharType) {
        (2, "Orc".to_string(), to_cp437('o'))
    }
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
