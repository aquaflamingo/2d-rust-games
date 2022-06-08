use bracket_lib::prelude::*;

mod map;
mod map_builder;
mod player;
mod camera;

// prelude is a common and minimal set of imports/exports 
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_W: i32 = 80;
    pub const SCREEN_H: i32 = 50;
    pub const DISPLAY_W: i32 = SCREEN_W / 2;
    pub const DISPLAY_H: i32 = SCREEN_H / 2;
    // you can refer to anything within the +main.rs+ scope as "crate"
    // in this case we are referencing the +map.rs+ file.
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;


enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode,
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            mode: GameMode::Menu,
            player: Player::new(mb.player_start),
            camera: Camera::new(mb.player_start),
        }
    }

    fn game_playing(&mut self,  ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);

        self.mode = GameMode::Playing
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
     self.mode = GameMode::End
    }

    fn game_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // match self.mode {
        //     GameMode::Menu => self.game_menu(ctx),
        //     GameMode::End => self.game_over(ctx),
            // GameMode::Playing => self.game_playing(ctx),
        // TODO
        self.game_playing(ctx)
        // }
    }

}

fn main() -> BError {
    // FIXME: Can't render the map
    let ctx = BTermBuilder::simple80x50()
        .with_title("Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_W, DISPLAY_H)
        .with_tile_dimensions(32,32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_W, DISPLAY_H, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_W, DISPLAY_H, "dungeonfont.png")
        .build()?;

    main_loop(ctx, State::new())
}
