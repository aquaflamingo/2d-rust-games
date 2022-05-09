// Use prelude and wildcard to import everything
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
// Menu -> (Play <> End) 

struct Obstacle {
    // x value defines position in world sapce
    x: i32,
    // gap_y is center of the gap 
    gap_y: i32,
    // size is the size of the gap
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rand = RandomNumberGenerator::new();

        // i32::max(2,20-score) causes the gaps to shrink as the player progresses
        Obstacle { x, gap_y: rand.range(10,40), size: i32::max(2,20-score)}
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }

    fn render(&mut self,  ctx: &mut BTerm, player_x: i32) {
        // .set is bracket-lib function to set a single cahracter on screen
        // bracket-lib has various colors you can use
        // We are rendering the '@' charcter in the name terminal font codepage 437
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('x'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('x'));
        }
    }

}


enum GameMode {
    Menu,
    Play,
    End,
}

struct State { 
    mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    score: i32,
    frame_time: f32,
}

struct Player {
    x: i32,
    y: i32,
    vel: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,y,vel: 0.0,
        }
    }

   fn render(&mut self,  ctx: &mut BTerm) {
        // .set is bracket-lib function to set a single cahracter on screen
        // bracket-lib has various colors you can use
        // We are rendering the '@' charcter in the name terminal font codepage 437
        ctx.set(3, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        // Check the current terminal velocity
        // Only apply downward momentum if less than 2
        if self.vel < 2.0 {
            self.vel += 0.5;
        }

        // Add velocity to the y position
        self.y += self.vel as i32;

        // Increment x for awareness of how far you have progressed through the level 
        self.x += 5;


        if self.y < 0 {
            self.y=0;
        }
    }

    fn flap(&mut self) {
        // Use a negative number to move upward
        // Recall that 0 is the TOP of the screen!
        self.vel = -2.0;
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // Like clear but lets you set background colour
        ctx.cls_bg(NAVY);

        // tick() runs fast so slow the game down 
        // frame_time_ms contains ms since last tick() was called
        self.frame_time += ctx.frame_time_ms;

        // if tick() time is greater than specified frame duration 
        // i.e. when something should update, trigger physics 
        // and start the frame tick again
        if self.frame_time > FRAME_DURATION {

            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }

        // Press SPACE to save ur doge plx!!
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        ctx.print(0,0, "Press SPACE to flap");
        ctx.print(0,1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);

        if self.player.x >self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH, self.score
            );
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

        // if you suck you suck, amirite?
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Play;
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
        self.player = Player::new(5,25);
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // cls = clear widnow
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Doge");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // .key method is for keyboard in bracket-lib
        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Wow! Such saddness, much death.");
        ctx.print_centered(6, &format!("Score: {}", self.score));

        ctx.print_centered(8, "(P) Pls again");
        ctx.print_centered(9, "(Q) Very angry, such rage quit");

        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}

// Implementing a trait for the GameState structure
impl GameState for State {
    // GameState requires tick() function to be implemented
    // ctx provides a window into the current bracket-terminal
    //   - this allows access to mouse and keyboard input
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Play => self.play(ctx),
        }

    }
}

// The bracket-lib provides a Result type named BError
// By returning BError, we can use the "?" syntax for Result and Error handling
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Doge")
        .build()?;


    main_loop(context, State::new())
}

