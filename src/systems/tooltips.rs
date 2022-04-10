use legion::world::SubWorld;
use crate::prelude::*;

#[system]
#[read_component(Point)]    // make readable access to entities with Point component
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera
){
    // use Entity to also retrieve the parent entity of the components
    let mut positions = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y); // camera top left corner map/screen coordinates
    let map_pos = *mouse_pos + offset; // convert from camera coordinates to map coordinates

    let mut draw_batch = DrawBatch::new();

    // target drawing for HUD layer
    draw_batch.target(2);

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            // monster position is in coordinates that align with monsters layer (1)
            // tooltip is in hud layer that is 4 times larger
            // multiply mouse position by 4 to get the screen position for the tooltip layer
            let screen_pos = *mouse_pos * 4;

            // access entities components from outside of a query
            let display = if let Ok(health) = ecs.entry_ref(*entity)
                .unwrap()
                .get_component::<Health>() // receive component from ECS for this entity without using a query
            {
                format!("{} : {} hp", &name.0, health.current)
            } else {
                // if entity does not have Health component (only Name component), just return name
                // use clone to make copy of the string rather than borrow it
                name.0.clone()
            };

            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}