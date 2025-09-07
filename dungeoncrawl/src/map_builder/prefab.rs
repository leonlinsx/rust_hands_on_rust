use crate::prelude::*;

// variables representing the prefab layout, width, and height
const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        // create rect with size of prefab
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );
        let mut can_place = false;
        dimensions.for_each(|p| {
            let idx = mb.map.point2d_to_index(p);
            let distance = dijkstra_map.map[idx];
            // check if tile is floor and far enough from player start and not on grail start
            if distance < 2000.0 && distance > 20.0 && mb.grail_start != p {
                can_place = true;
            }
        });
        // erase any random monster spawns in the area
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|p| !points.contains(p));
        }
        attempts += 1;
    }

    // apply prefab to map at placement point
    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            // remove newlines and carriage returns
            .chars()
            .filter(|c| *c != '\r' && *c != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    'M' => {
                        // monsters need a floor tile
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    _ => println!("Unknown char in prefab: {}", c),
                }
                i += 1;
            }
        }
    }
}
