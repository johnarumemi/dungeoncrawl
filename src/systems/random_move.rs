use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;


#[system]
#[read_component(Point)]           // get read reference
#[read_component(MovingRandomly)]  // get read reference
pub fn random_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
){

    <(Entity, &Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|(entity, pos)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = *pos + match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1)
            };

            commands.push(((), WantsToMove{entity: *entity, destination}));
        });
}