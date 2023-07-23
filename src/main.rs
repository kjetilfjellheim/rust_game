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
    edges: Vec<Rectangle>,
    bricks: Vec<Bricks>
}

pub struct Rectangle {
    x: f32,
    y: f32,
    height: f32,
    width: f32
}

pub struct Bricks {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    state: bool,
    texture: Texture
}

pub enum Edge {
    HorizontalEdge,
    VerticalEdge,
    Both
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle { x: x, y: y, height: height, width: width }
    }
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

        let pad_collision = Self::detect_collision(self.pad_x, self.pad_y, PAD_WIDTH, PAD_HEIGHT, self.ball_x + self.ball_dx, self.ball_y + self.ball_dy, BALL_WIDTH_RADIUS);
        match pad_collision {
            Some(Edge::Both) => { self.ball_dx = -self.ball_dx;self.ball_dy = -self.ball_dy; },
            Some(Edge::HorizontalEdge) => { self.ball_dy = -self.ball_dy; },
            Some(Edge::VerticalEdge) => { self.ball_dx = -self.ball_dx; },
            None => {}
        }

        for edge in self.edges.iter_mut() {
            let edge = Self::detect_collision(edge.x, edge.y, edge.width, edge.height, self.ball_x + self.ball_dx, self.ball_y + self.ball_dy, BALL_WIDTH_RADIUS);
            match edge {
                Some(Edge::Both) => { self.ball_dx = -self.ball_dx;self.ball_dy = -self.ball_dy; },
                Some(Edge::HorizontalEdge) => { self.ball_dy = -self.ball_dy; },
                Some(Edge::VerticalEdge) => { self.ball_dx = -self.ball_dx; },
                None => {}
            }
        }

        for brick in self.bricks.iter_mut() {
            let edge = Self::detect_collision(brick.x, brick.y, brick.width, brick.height, self.ball_x + self.ball_dx, self.ball_y + self.ball_dy, BALL_WIDTH_RADIUS);
            match edge {
                Some(Edge::Both) => { self.ball_dx = -self.ball_dx;self.ball_dy = -self.ball_dy; },
                Some(Edge::HorizontalEdge) => { self.ball_dy = -self.ball_dy; },
                Some(Edge::VerticalEdge) => { self.ball_dx = -self.ball_dx; },
                None => {}
            }
        }

        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

    }

    fn detect_collision(x: f32, y: f32, width: f32, height: f32, ball_x: f32, ball_y: f32, ball_radius: f32) -> Option<Edge> {
        let ball_centerpoint_x:f32 = ball_x + ball_radius;
        let ball_centerpoint_y:f32 = ball_y + ball_radius;

        let closest_x = (ball_centerpoint_x.max(x)).min(x + width);
        let closest_y = (ball_centerpoint_y.max(y)).min(y + height);
    
        let distance_x = ball_centerpoint_x - closest_x;
        let distance_y = ball_centerpoint_y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;
    
        if distance_squared < BALL_WIDTH_SQRD {
            let vertical_collision = distance_x.abs() < BALL_WIDTH_RADIUS - 1.0;
            let horizontal_collision = distance_y.abs() < BALL_WIDTH_RADIUS - 1.0;
    
            if vertical_collision && horizontal_collision {
                return Some(Edge::Both);
            } else if vertical_collision {
                return Some(Edge::HorizontalEdge);
            } else {
                return Some(Edge::VerticalEdge);
            }            
        } 
        None
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

fn get_edges() -> Vec<Rectangle> {
    vec![
        Rectangle::new(0.0, -100.0, 1024.0, 100.0),
        Rectangle::new(-100.0, 0.0, 100.0, 1024.0),
        Rectangle::new(1024.0, 0.0, 100.0, 1024.0),
    ]
    
}

fn get_bricks() -> Vec<Bricks> {
    vec![

    ]
    
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
        edges: get_edges(),
        bricks: get_bricks()
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
