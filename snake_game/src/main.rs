use rand::prelude::*;
use rusty_engine::prelude::*;

//enums
enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    Still,
}

//structs
#[derive(Resource)]
struct GameState {
    score: u16,
    direction: MovementDirection,
    spawn_timer: Timer,
    target_index: u16,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            direction: MovementDirection::Still,
            spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            target_index: 0,
            lost: false,
        }
    }
}

// Constants
const MOVEMENT_SPEED: f32 = 200.0;

fn main() {
    let mut game = Game::new();

    // Window settings
    game.window_settings(Window {
        title: "Snake game".to_string(),
        ..Default::default()
    });

    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    // Adding game logic using functions
    game.add_logic(movement_logic);
    game.add_logic(spawn_logic);
    game.add_logic(collision_logic);
    game.add_logic(game_over);

    // Running game
    game.run(GameState::default());
}

fn movement_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost == true {return;}
    // changing movement direction through keyboard inputs
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        game_state.direction = MovementDirection::Up;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        game_state.direction = MovementDirection::Down;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        game_state.direction = MovementDirection::Right;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        game_state.direction = MovementDirection::Left;
    }

    let player = engine.sprites.get_mut("player").unwrap();

    // Logic for continuous movement
    match game_state.direction {
        MovementDirection::Up => player.translation.y += MOVEMENT_SPEED * engine.delta_f32,
        MovementDirection::Down => player.translation.y -= MOVEMENT_SPEED * engine.delta_f32,
        MovementDirection::Left => player.translation.x -= MOVEMENT_SPEED * engine.delta_f32,
        MovementDirection::Right => player.translation.x += MOVEMENT_SPEED * engine.delta_f32,
        _ => {}
    }

    if player.translation.y > engine.window_dimensions.y / 2.0
        || player.translation.y < -engine.window_dimensions.y / 2.0
        || player.translation.x > engine.window_dimensions.x / 2.0
        || player.translation.x < -engine.window_dimensions.x / 2.0
    {
        game_state.lost = true;
    }
}

fn spawn_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost == true {return;}

    let window_x_limit = engine.window_dimensions.x / 2.0 - 10.0;
    let window_y_limit = engine.window_dimensions.y / 2.0 - 10.0;
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("target{}", game_state.target_index);
        game_state.target_index += 1;
        let target = engine.add_sprite(label, SpritePreset::RacingBarrelRed);
        target.collision = true;
        target.scale = 0.65;
        target.translation.x = thread_rng().gen_range(-window_x_limit..window_x_limit);
        target.translation.y = thread_rng().gen_range(-window_y_limit..window_y_limit);
    }
}

fn collision_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost == true {return;}
    //handle collisions
    engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                    game_state.score += 1;
                    let score = engine.texts.get_mut("score").unwrap();
                    score.value = format!("Score: {}", game_state.score);
                }
            }
        } else if event.state == CollisionState::Begin && event.pair.either_contains("player") {
            println!("Cannot use this");
        }
    }
}

fn game_over(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost == true {
        let game_over = engine.add_text("game over", "GAME OVER");
        game_over.font_size = 150.0;
        return
    }
}
