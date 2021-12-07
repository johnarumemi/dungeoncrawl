use crate::prelude::*;



pub struct State {
    ecs: World,           // use by Legion for storing all entities and components
    resources: Resources, // shared data between systems

    // 3 separate systems based on current turn state
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default(); // list of resources

        // random number generator for the game
        let mut rng = RandomNumberGenerator::new();

        // map builder
        let map_builder = MapBuilder::new(&mut rng);

        // spawn player entity and add to ecs/World
        spawn_player(&mut ecs, map_builder.player_start);

        // spawn monsters
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Add camera and generated map to list of resources
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen and layers
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();

        ctx.set_active_console(2);
        ctx.cls();
        
        // when we add a resource of the same type, it replaces previous resource of same type
        // hence the below insert will not result in duplicate resources
        self.resources.insert(ctx.key);

        // execute the Schedule to run the various systems
        // Schedule requires access to the World (entities & components) and resources (shared data)
        // execute specific scheduler based on turn state
        // self.resources.get::<TYPE> request resource of a specific type
        // result returned as an Option
        // use .unwrap() to access the contents
        // use .clone() to duplicate the state, ensures resource is no longer borrowed
        let current_state: TurnState  = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources)
        };

        render_draw_buffer(ctx).expect("Render error");
    }
}
