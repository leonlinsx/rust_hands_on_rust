mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    // Game state fields go here
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_grail(&mut ecs, map_builder.grail_start);
        map_builder
            .rooms
            .iter()
            .skip(1) // skip first room, where player starts
            .map(|r| r.center()) // transform to center point of room
            .for_each(|pos| {
                spawn_monster(&mut ecs, &mut rng, pos);
            });
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1) // skip first room, where player starts
            .map(|r| r.center()) // transform to center point of room
            .for_each(|pos| {
                spawn_monster(&mut self.ecs, &mut rng, pos);
            });
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.set_active_console(2); // use top layer for UI
        ctx.print_color_centered(10, RED, BLACK, "Your journey has ended.");
        ctx.print_color_centered(
            12,
            WHITE,
            BLACK,
            "Slain by an enemy, your adventure is over.",
        );
        ctx.print_color_centered(
            14,
            WHITE,
            BLACK,
            "The grail remains unclaimed, and your home town is lost.",
        );
        ctx.print_color_centered(16, WHITE, BLACK, "Don't worry, you can always try again.");
        ctx.print_color_centered(18, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.set_active_console(2); // use top layer for UI
        ctx.print_color_centered(10, GOLD, BLACK, "You found the Holy Grail!");
        ctx.print_color_centered(
            12,
            WHITE,
            BLACK,
            "With the grail in hand, you return to your home town.",
        );
        ctx.print_color_centered(
            14,
            WHITE,
            BLACK,
            "The townsfolk rejoice as you bring them salvation.",
        );
        ctx.print_color_centered(16, WHITE, BLACK, "Congratulations on your victory!");
        ctx.print_color_centered(18, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // multiple layers
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key); // insert current key press into resources
        ctx.set_active_console(0); // get mouse pos coordinates from correct layer
        self.resources.insert(Point::from_tuple(ctx.mouse_pos())); // tuple of x,y coordinates
        let current_state = self.resources.get::<TurnState>().unwrap().clone(); // requests a resource, gets an Option that needs to be unwrapped, clone to use a copy
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self
                .enemy_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0) // tracks game speed to prevent player from moving too quickly
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // allows transparency
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
