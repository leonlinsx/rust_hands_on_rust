use crate::prelude::*;

// requests map and camera as resources and indicates with annotation
#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // append draw commands to the draw batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            if map.in_bounds(Point::new(x, y)) {
                let pt = Point::new(x, y);
                let offset = Point::new(camera.left_x, camera.top_y);
                if map.in_bounds(pt) {
                    let idx = map_idx(x, y);
                    let glpyh = match map.tiles[idx] {
                        TileType::Floor => to_cp437('.'),
                        TileType::Wall => to_cp437('#'),
                    };
                    draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glpyh);
                }
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
