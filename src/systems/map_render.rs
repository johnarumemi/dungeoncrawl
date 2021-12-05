use crate::prelude::*;


#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera
){
    let mut draw_batch = DrawBatch::new();

    // starts a drawing batch
    draw_batch.target(0);

    // use camera for rendering the map
    for y in camera.top_y..camera.bottom_y {
        // outer loop on y index, faster for row-first striding due to memory cache usage
        for x in camera.left_x..camera.right_x {

            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            // check that the camera point is within map boundary and draw it if valid
            if map.in_bounds(pt) {

                let idx = map_idx(x, y);

                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                    };

                // screen coordinates shifted by left_x and top_y
                // to make them relative to camera
                // i.e. convert from map co-ordinate to camera co-ordinate.
                // since map is going to be much large than actual screen size
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(
                        WHITE,
                        BLACK
                    ),
                    glyph
                );

                }
            };
        };

    // submitting the batch adds it to the global command list. Integer parameter
    // acts as a sort order. 0 renders first, ensuring that your map is drawn at the
    // beginning of the render cycle
    draw_batch.submit(0     ).expect("Batch draw error");
}
