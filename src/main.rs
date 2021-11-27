use bracket_lib::prelude::*; // make everything from bracket_lib available

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(0, "Hello Dungeon Crawler!");
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;

    main_loop(context, State{})
}
