use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::{event, graphics, Context, GameResult, timer};
use ggez::nalgebra as na;

use std::env;
use std::path;

mod tiles;
mod game_state;
mod player;
use crate::game_state::GameState;

const GRID_SIZE: (i16, i16) = (35,25);
const CELL_SIZE: i16 = 32;

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.player.move_player(&self.platforms);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player.lr_v = -15.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.player.lr_v = 15.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            if self.player.jump_v == None {
                self.player.jump_v = Some(15.0)
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        
        for pos in &self.platforms {

            let platform_mesh = tiles::rec_from_tiles(
                ctx, 
                pos.0, 
                pos.1, 
                pos.2, 
                1, 
                graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT), 
                [0.0, 0.0, 0.0, 1.0]
            )?;
    
            graphics::draw(ctx, &platform_mesh, (na::Point2::new(0.0,0.0),))?;
        } 

        let player_mesh = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT), 
            na::Point2::new(
                self.player.x as f32 * 32.0 + 16.0, 
                self.player.y as f32 * 32.0 + 16.0
            ),
            16.0,
            0.01,
            [0.5, 0.5, 0.5, 1.0].into()
        )?;
        graphics::draw(ctx, &player_mesh, (na::Point2::new(0.0,0.0),))?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::S => {
                if self.player.y < 24.0  && self.player.jump_v == None{
                    self.player.y += 1.0;
                }
            }
            KeyCode::R => {
                self.platforms = tiles::gen_platforms();
            }
            _ => {}
        }
    }
}

fn main() -> GameResult {
    println!("A - move left\nD - move right\nW - jump\nS - go down a platform\nR - reload the platforms");
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("platformer", "Omrihhh").add_resource_path(resource_dir)
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions((GRID_SIZE.0 * CELL_SIZE) as f32, (GRID_SIZE.1 * CELL_SIZE) as f32))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = &mut GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
