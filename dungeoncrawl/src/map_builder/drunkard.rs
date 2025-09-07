use super::MapArchitect;
use crate::prelude::*;
pub struct DrunkardWalkArchitect {}

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR_TILES: usize = NUM_TILES / 3;

impl MapArchitect for DrunkardWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            grail_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR_TILES
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, d)| !d.is_finite()) // or: **d == f32::MAX
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.grail_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut staggered_distance = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            // carve out floor at drunkard position
            map.tiles[drunk_idx] = TileType::Floor;
            // randomly move drunkard in one of four cardinal directions
            match rng.range(0, 4) {
                0 => drunkard_pos.x += 1,
                1 => drunkard_pos.x -= 1,
                2 => drunkard_pos.y += 1,
                _ => drunkard_pos.y -= 1,
            }
            if !map.in_bounds(drunkard_pos) || staggered_distance > STAGGER_DISTANCE {
                break;
            } else {
                staggered_distance += 1;
            }
        }
    }
}
