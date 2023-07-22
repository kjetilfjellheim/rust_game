extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate nalgebra as na;

use glutin_window::GlutinWindow;
use graphics::{clear, image as img, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::{Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseCursorEvent, RenderEvent, UpdateArgs};
use piston::window::WindowSettings;
use piston::{RenderArgs, UpdateEvent};

const PAD_WIDTH: f32 = 100.0;
const PAD_WIDTH_HALF: f32 = PAD_WIDTH / 2.0;
const BALL_WIDTH: f32 = 50.0;
const BALL_WIDTH_RADIUS: f32 = BALL_WIDTH / 2.0;
const BALL_WIDTH_SQRD: f32 = BALL_WIDTH_RADIUS * BALL_WIDTH_RADIUS;
const WINDOW_HEIGHT: f32 = 1024.0;
const WINDOW_WIDTH: f32 = 1024.0;
const PAD_MIN_HEIGTH: f32 = 0.0;
const PAD_MAX_HEIGTH: f32 = 950.0;
const PAD_HEIGHT: f32 = 20.0;

pub struct App {
    gl: GlGraphics,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    pad_x: f32,
    pad_y: f32,
    pad_dx: f32,
    pad_dy: f32,
    bg_texture: Texture,
    pad_texture: Texture,
    ball_texture: Texture,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let pad_x = self.pad_x as f64;
        let pad_y = self.pad_y as f64;

        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            img(&self.bg_texture, c.transform.trans(0.0, 0.0), gl);
            let transform_pad = c.transform.trans(pad_x, pad_y);
            img(&self.pad_texture, transform_pad, gl);
            let transform_ball = c.transform.trans(ball_x, ball_y);
            img(&self.ball_texture, transform_ball, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {

        let ball_centerpoint_x:f32 = self.ball_x + BALL_WIDTH_RADIUS + self.ball_dx;
        let ball_centerpoint_y:f32 = self.ball_y + BALL_WIDTH_RADIUS + self.ball_dy;

        let closest_x = (ball_centerpoint_x.max(self.pad_x)).min(self.pad_x + PAD_WIDTH);
        let closest_y = (ball_centerpoint_y.max(self.pad_y)).min(self.pad_y + PAD_HEIGHT);
    
        let distance_x = ball_centerpoint_x - closest_x;
        let distance_y = ball_centerpoint_y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;
    
        if distance_squared < BALL_WIDTH_SQRD {
            let vertical_collision = distance_x.abs() < BALL_WIDTH_RADIUS - 1.0;
            let horizontal_collision = distance_y.abs() < BALL_WIDTH_RADIUS - 1.0;
    
            if vertical_collision && horizontal_collision {
                println!("Both");
                self.ball_dx = -self.ball_dx;
                self.ball_dy = -self.ball_dy;
            } else if vertical_collision {
                println!("Vertical");
                if self.ball_y < closest_y {
                    self.ball_dy = -self.ball_dy;
                } else {
                    self.ball_dy = -self.ball_dy;
                }
            } else {
                println!("Horizontal");
                if self.ball_x < closest_x {
                    self.ball_dx = -self.ball_dx;
                } else {
                    self.ball_dx = -self.ball_dx;
                }
            }
        } 
        
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        if self.ball_y > (WINDOW_HEIGHT - BALL_WIDTH) || self.ball_y < 0.0 {
            self.ball_dy = -self.ball_dy;
        }
        if self.ball_x > (WINDOW_WIDTH - BALL_WIDTH) || self.ball_x < 0.0 {
            self.ball_dx = -self.ball_dx;
        }

    }

    fn event(&mut self, x: f32, y: f32) {        
        self.pad_dx = self.pad_x - x;
        self.pad_dy = self.pad_y - y;        
        self.pad_x = x - PAD_WIDTH_HALF;        
        self.pad_y = y;
        if self.pad_y < PAD_MIN_HEIGTH {
            self.pad_y = PAD_MIN_HEIGTH;
        }
        if self.pad_y > PAD_MAX_HEIGTH {
            self.pad_y = PAD_MAX_HEIGTH;
        }
    }

    
}

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Simple game", [WINDOW_HEIGHT as f64, WINDOW_WIDTH as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Could not find assets.");
    let background = assets.join("spiral-galaxy.jpg");
    let bg_texture = Texture::from_path(background, &TextureSettings::new()).expect("Could not find background asset");

    let pad = assets.join("pad.png");
    let pad_texture = Texture::from_path(pad, &TextureSettings::new()).expect("Could not find pad asset");

    let ball: std::path::PathBuf = assets.join("ball.png");
    let ball_texture = Texture::from_path(ball, &TextureSettings::new()).expect("Could not find ball asset");

    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball_x: 512.0,
        ball_y: 512.0,
        ball_dx: 1.0,
        ball_dy: 1.0,
        pad_x: 550.0,
        pad_y: 550.0,
        pad_dx: 0.0,
        pad_dy: 0.0,
        bg_texture,
        pad_texture,
        ball_texture,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r)
        }
        if let Some(u) = e.update_args() {
            app.update(&u)
        }
        if let Some(n) = e.mouse_cursor_args() {
            app.event(n[0] as f32, n[1] as f32)
        }
    }
}
