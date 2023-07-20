extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate image;
extern crate find_folder;

use std::process;
use piston::{RenderArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateArgs, MouseCursorEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::{rectangle, clear, Transformed, ellipse, image as img};
use opengl_graphics::{Texture, TextureSettings};

const PAD_WIDTH: f64 = 100.0;
const PAD_WIDTH_HALF: i32 = (PAD_WIDTH / 2.0) as i32;

pub struct App {
    gl: GlGraphics,
    ball_x: i32,
    ball_y: i32,
    ball_dx: i32,
    ball_dy: i32,
    pad_x: i32,
    pad_y: i32,
    pad_dx: i32,
    pad_dy: i32,
    bg_texture: Texture,
    pad_texture: Texture,
    ball_texture: Texture
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let pad = rectangle::square(0.0, 0.0, PAD_WIDTH);
        let pad_x = self.pad_x as f64;
        let pad_y = self.pad_y as f64;

        let ball = rectangle::square(0.0, 0.0, 30.0);
        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;


        self.gl.draw(args.viewport(), |c,gl| {
            clear(BACKGROUND, gl);
            img(&self.bg_texture, c.transform.trans(0.0, 0.0), gl);
            let transform_pad = c.transform.trans(pad_x, pad_y);
            img(&self.pad_texture, transform_pad, gl);
            let transform_ball = c.transform.trans(ball_x, ball_y);
            img(&self.ball_texture, transform_ball, gl);

            

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;
    }

    fn event(&mut self, x: i32, y: i32) {
        self.pad_dx = self.pad_x - x;
        self.pad_dy = self.pad_y - y;
        self.pad_x = x - PAD_WIDTH_HALF;
        self.pad_y = y;        
    }

}

fn main() {

    let opengl = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Simple game", [1024,1024])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let background = assets.join("spiral-galaxy.jpg");
    let bg_texture = Texture::from_path(background, &TextureSettings::new()).unwrap();

    let pad = assets.join("pad.png");
    let pad_texture = Texture::from_path(pad, &TextureSettings::new()).unwrap();    

    let ball: std::path::PathBuf = assets.join("ball.png");
    let ball_texture = Texture::from_path(ball, &TextureSettings::new()).unwrap();    

    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball_x: 512,
        ball_y: 512,
        ball_dx: 1,
        ball_dy: 1,
        pad_x: 0,
        pad_y: 0,
        pad_dx: 0,
        pad_dy: 0,
        bg_texture,
        pad_texture,
        ball_texture
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
            app.event(n[0] as i32, n[1] as i32)
        }
    }

}
