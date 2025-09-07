use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            grail_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.grail_start = mb.find_most_distant();

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        // mutably iterate over all tiles in map
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut count_neighbours = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                // dont count current tile, only neighbours; incl diagonals
                if !(dx == 0 && dy == 0) && map.tiles[map_idx(x + dx, y + dy)] == TileType::Wall {
                    count_neighbours += 1;
                }
            }
        }
        count_neighbours
    }

    fn iteration(&self, map: &mut Map) {
        // copy of map to count neighbours without affecting current iteration
        let mut new_tiles = map.tiles.clone();
        // ignore edges of map to avoid out of bounds error
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let idx = map_idx(x, y);
                let neighbours = self.count_neighbours(x, y, map);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            // enumerate to get index and tile type
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            // now have iterator of (index, tile type) for all floor tiles
            // calc distance from center for each tile
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            // find minimum distance tile to center and returns Option
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}
