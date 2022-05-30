use bracket_lib::prelude::*;

mod map;
mod player;

// prelude is a common and minimal set of imports/exports 
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_W: i32 = 80;
    pub const SCREEN_H: i32 = 50;
    // you can refer to anything within the +main.rs+ scope as "crate"
    // in this case we are referencing the +map.rs+ file.
    pub use crate::map::*;
    pub use crate::player::*;
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
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            mode: GameMode::Menu,
            player: Player::new(
                Point::new(SCREEN_W / 2, SCREEN_H / 2)
            ),
        }
    }

    fn game_playing(&mut self,  ctx: &mut BTerm) {
        ctx.cls();

        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);

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
    let ctx = BTermBuilder::simple80x50()
        .with_title("Starter")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
}
