use crate::prelude::*;

// requests map and camera as resources and indicates with annotation
#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    // query for player fov
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    // append draw commands to the draw batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            if map.in_bounds(Point::new(x, y)) {
                let pt = Point::new(x, y);
                let offset = Point::new(camera.left_x, camera.top_y);
                // only draw tiles that are in the player's FOV
                let idx = map_idx(x, y);
                // draw if in bounds and either visible or revealed
                if map.in_bounds(pt)
                    && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
                {
                    // tint color based on visibility
                    let tint = if player_fov.visible_tiles.contains(&pt) {
                        WHITE
                    } else {
                        DARK_GRAY
                    };
                    let glpyh = theme.tile_to_render(map.tiles[idx]);
                    draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glpyh);
                }
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
