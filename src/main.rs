use bracket_lib::prelude::*;

mod map;
mod map_builder;
mod camera;
mod prelude { 
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;
    pub const TILE_SIZE: i32 = 32;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}
use prelude::*;

struct State {
    map: Map,
    camera: Camera,
}
impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map : map_builder.map,
            camera : Camera::new(map_builder.player_start),
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        //TODO: Execute Systems
        //TODO: Render Draw Buffer
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Pias Rusty Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_SIZE, TILE_SIZE)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png",TILE_SIZE,TILE_SIZE)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
