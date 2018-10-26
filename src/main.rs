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

mod object;
mod tree;
mod theme;
use theme::Theme;
use object::Object;
use tree::Tree;

use std::path::Path;
use piston::window::WindowSettings;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, Texture };

pub struct Cube {
    gl: GlGraphics,
    player: Object,
    trees: Tree,
    theme: Theme,
    fuckx: i32,
    fucky: i32,
    map_width: i32,
    map_height: i32,
    width: f64,
    height: f64,
    draw_height: f64,
    draw_width: f64,
    size: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Cube {
    fn on_load(&mut self, _w: &PistonWindow) {
        let p1_sprite = Texture::from_path(
                &Path::new("./assets/fuck.png"),
                &TextureSettings::new()).unwrap();
        self.player.set_sprite(p1_sprite);
        let background = Texture::from_path(
                &Path::new("./assets/background.png"),
                &TextureSettings::new())
                .unwrap();
        self.theme.set_sprite(background);
        let tree = Texture::from_path(
                    &Path::new("./assets/Tree.png"),
                    &TextureSettings::new())
                    .unwrap();
        self.trees.set_sprite(tree);
    }
    fn on_draw(&mut self, args: &RenderArgs) {
        let fuck_this = &self.player;
        let fuck_trees = &self.trees;
        let fuck_theme = &self.theme;
        let fuck_width = &self.width;
        let fuck_height = &self.height;
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let textx = self.player.x.to_string();
        let texty = self.player.y.to_string();
        let viewport = Viewport {
            rect: [self.fuckx, self.fucky, self.map_width, self.map_height],
            window_size: [self.width as u32, self.height as u32],
            draw_size: [self.draw_width as u32, self.draw_height as u32],
        };
        self.gl.draw(viewport, |c, gl| {
            let center = c.transform.trans((fuck_width / 2.0) as f64, (fuck_height / 2.0) as f64);
            clear([0.0, 0.0, 0.0, 0.0], gl);
            fuck_theme.rendertheme(gl, center);
            fuck_trees.moar_trees(gl, center);
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
        let widthcol = (self.draw_width / 2.0) as f64;
        let rad = (self.size / 2.0) as f64;
        if self.player.x <= -widthcol + 100.0 {
            if self.left_d == true {
                self.fuckx = -self.player.x as i32 - 300;
            }
        }
        if self.player.x >= 300.0 {
            if self.right_d == true {
                self.fuckx = -self.player.x as i32 + 300;
                self.map_width += -500;
            }
        }
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
        trees : Tree::new(),
        theme : Theme::new(),
        fuckx: 0,
        fucky: 0,
        width: width as f64,
        height: height as f64,
        draw_height: height as f64,
        draw_width: width as f64,
        map_width: 800,
        map_height: 600,
        size: 50.0,
        up_d: false,
        down_d: false,
        left_d: false,
        right_d: false
    };
    cube.on_load(&window);
    while let Some(e) = window.next() {
        if let Some(u) = e.update_args() {
            cube.update(&u);
        }
        if let Some(r) = e.render_args() {
            cube.on_draw(&r);
        }
        if let Some(i) = e.button_args() {
            cube.on_input(&i);
        }
    }
}
