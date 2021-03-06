extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate image;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate rand;
extern crate sprite;
extern crate viewport;
extern crate fps_counter;

mod object;
mod tree;
mod theme;
use theme::Lawn;
use object::Object;
use tree::Tree;

use piston::window::WindowSettings;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };

pub struct Cube {
    gl: GlGraphics,
    player: Object,
    trees: Vec<Tree>,
    terrain: Vec<Lawn>,
    width: f64,
    height: f64,
    viewx: f64,
    viewy: f64,
    chunk_size_x: f64,
    chunk_size_y: f64,
    chunk_amount_x: f64,
    chunk_amount_y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Cube {
    pub fn check_chunks(&mut self) {
        let player_chunk_x = self.player.x/self.chunk_size_x;
        let player_chunk_y = self.player.y/self.chunk_size_y;
        let mut done = false;
        //println!("{} {}", player_chunk_x, player_chunk_y);
        for lawn in &self.terrain {
            if lawn.x > player_chunk_x + (self.chunk_amount_x - 1.0) / 2.0 ||
                lawn.x < player_chunk_x - (self.chunk_amount_x - 1.0) / 2.0 ||
                lawn.y > player_chunk_y + (self.chunk_amount_y - 1.0) / 2.0 ||
                lawn.y < player_chunk_y - (self.chunk_amount_y - 1.0) / 2.0 {
                //println!("UNLOAD CHUNK", )
            }
        }
        let mut i = player_chunk_x - (self.chunk_amount_x - 1.0) / 2.0;
        let mut j = player_chunk_y - (self.chunk_amount_y - 1.0) / 2.0;
        while i <= player_chunk_x + (self.chunk_amount_x - 1.0) / 2.0 {
            while j <= player_chunk_y + (self.chunk_amount_y - 1.0) / 2.0 {
                self.terrain.push(Lawn::new(i, j));
                //println!("{} {}", i, j);
                //println!("LOAD CHUNKS",);
                while !done {
                    if self.terrain.contains(&Lawn::new(i, j)) {
                        println!("{} {}", i, j);
                        done = true;
                    }
                }
            }
        }
    }
    /*fn on_load(&mut self) {
        for _j in 0.0..1.0 {
            for i in 0.0..1.0 {
                self.terrain.push(Lawn::new(i));
               // self.trees.push(Tree::new(i));
            }
        }
    }*/
    fn on_draw(&mut self, args: &RenderArgs) {
        let fuck_this = &self.player;
        let fuck_trees = &self.trees;
        let fuck_terrain = &self.terrain;
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let textx = self.player.x.to_string();
        let texty = self.player.y.to_string();
        let (w, h) = (self.width, self.height);
        self.gl.draw(args.viewport(), |c, gl| {
            let _view = c.transform.trans(w, h);
            let center = c.transform.trans(w / 2.0, h / 2.0);
            clear([0.0, 1.0, 0.0, 0.0], gl);
            for lawn in fuck_terrain {
                lawn.renderterrain(gl, center);
            }
            for tree in fuck_trees {
                tree.moar_trees(gl, center);
            }
            fuck_this.render(gl, center);
            text::Text::new_color([1.0, 0.0, 0.0, 1.0], 25).draw(&textx,
                                                                     &mut glyph_cache,
                                                                     &DrawState::default(),
                                                                     c.transform
                                                                         .trans(10.0, 25.0),
                                                                     gl).unwrap();
            text::Text::new_color([1.0, 0.0, 0.0, 1.0], 25).draw(&texty,
                                                                     &mut glyph_cache,
                                                                     &DrawState::default(),
                                                                     c.transform
                                                                         .trans(10.0, 50.0),
                                                                     gl).unwrap();
        });
    }
    fn update(&mut self, upd: &UpdateArgs) {
        if self.up_d {
            self.player.mov(0.0, -500.0 * upd.dt);
        }
        if self.down_d {
            self.player.mov(0.0, 500.0 * upd.dt);
        }
        if self.left_d {
            self.player.mov(-500.0 * upd.dt, 0.0);
        }
        if self.right_d {
            self.player.mov(500.0 * upd.dt, 0.0);
        }
        self.width = self.viewx - self.player.x * 2.0;
        self.height = self.viewy - self.player.y * 2.0;
    }
    fn on_input(&mut self, button_args: &ButtonArgs) {
        match button_args.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::D => self.right_d = true,
                        Key::A => self.left_d = true,
                        Key::S => self.down_d = true,
                        Key::W => self.up_d = true,
                        _ => {}
                    }
                }
            }
            ButtonState::Release => {

                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::D => self.right_d = false,
                        Key::A => self.left_d = false,
                        Key::S => self.down_d = false,
                        Key::W => self.up_d = false,
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 800;
    let height = 600;
    let mut window: PistonWindow = WindowSettings::new("Welcome to the bonezone", (width, height))
        .fullscreen(false)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut cube = Cube {
        gl: GlGraphics::new(opengl),
        player : Object::new(),
        trees : Vec::new(),
        terrain : Vec::new(),
        width: width as f64,
        height: height as f64,
        viewx: width as f64,
        viewy: height as f64,
        chunk_size_x : 256.0/2.0,
        chunk_size_y : 256.0/2.0,
        chunk_amount_x : 5.0,
        chunk_amount_y : 5.0,
        up_d: false,
        down_d: false,
        left_d: false,
        right_d: false
    };
    //cube.on_load();
    let mut events = Events::new(EventSettings {
        max_fps: 100,
        ups: 50,
        ups_reset: 0,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    });
    cube.check_chunks();
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            cube.update(&u);
        }
        if let Some(r) = e.render_args() {
            cube.on_draw(&r);
        }
        if let Some(i) = e.button_args() {
            cube.on_input(&i);
        }
        if let Some(IdleArgs) = e.idle_args() {
        }
    }
}
