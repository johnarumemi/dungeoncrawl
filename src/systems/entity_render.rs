use legion::world::SubWorld;
use crate::prelude::*;


#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera){

    // target graphics layer 1
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    // use camera to determine offset to apply to entity's screen position
    <(&Point, &Render)>::query() // generate query
        .iter(ecs) // transform query into an iterator
        .for_each(|(pos, render)| { // loop over grouped components for each matched entity
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
    });

    // 5000 is used as a sort order because the map may include 400 elements.
    draw_batch.submit(5000).expect("Batch render entity error");
}