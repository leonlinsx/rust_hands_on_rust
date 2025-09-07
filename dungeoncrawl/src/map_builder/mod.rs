mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;

use crate::prelude::*;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardWalkArchitect;
use empty::EmptyArchitect;
use prefab::apply_prefab;
use rooms::RoomsArchitect;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub grail_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(CellularAutomataArchitect {}),
            1 => Box::new(DrunkardWalkArchitect {}),
            _ => Box::new(RoomsArchitect {}),
        };
        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);
        mb
    }

    // pub fn new(rng: &mut RandomNumberGenerator) -> Self {
    //     let mut mb = MapBuilder {
    //         map: Map::new(),
    //         rooms: Vec::new(),
    //         player_start: Point::zero(),
    //         grail_start: Point::zero(),
    //     };
    //     mb.fill(TileType::Wall);
    //     mb.build_random_rooms(rng);
    //     mb.build_corridors(rng);
    //     // place player in center of first room so they start in valid tile
    //     mb.player_start = mb.rooms[0].center();

    //     let dijkstra_map = DijkstraMap::new(
    //         SCREEN_WIDTH,
    //         SCREEN_HEIGHT,
    //         &vec![mb.map.point2d_to_index(mb.player_start)],
    //         &mb.map,
    //         1024.0,
    //     );

    //     // find furthest point from player start
    //     const UNREACHABLE: &f32 = &f32::MAX;
    //     mb.grail_start = mb.map.index_to_point2d(
    //         dijkstra_map
    //             .map
    //             .iter()
    //             .enumerate()
    //             .filter(|(_, distance)| *distance < UNREACHABLE) // filter out unreachable tiles
    //             // cant use max since entry is borrowed, so use max_by with partial_cmp
    //             .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // get furthest reachable tile
    //             .unwrap()
    //             .0, // get first item of tuple, the index
    //     );
    //     mb
    // }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        // find furthest point from player start
        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance < UNREACHABLE) // filter out unreachable tiles
                // cant use max since entry is borrowed, so use max_by with partial_cmp
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // get furthest reachable tile
                .unwrap()
                .0, // get first item of tuple, the index
        )
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        // flips x1 and x2 if x2 < x1 to always go left to right
        for x in min(x1, x2)..=max(x1, x2) {
            // check if point is in bounds, then set to floor
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                // try_idx returns an Option for convenience, have to convert to usize
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // get sorted rooms (by center point) to reduce long corridors
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // skip first entry so we can reference previous room
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // randomly choose to go horizontal then vertical, or vice versa
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // keep generating rooms until we have enough
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                // if no overlap, check they are within boundaries, set contents to floor
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            // filter to only floor tiles and those far enough from player start
            .filter(|(idx, tile)| {
                **tile == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            // randomly select a tile from slice of available spawnable tiles
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            // remove from available tiles so we dont spawn multiple monsters in same spot
            spawnable_tiles.remove(target_index);
        }

        spawns
    }
}
