extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use rand::prelude::*;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0]; // r, g, b, opacity

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GRAY, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args, 20);
    }

    fn update(&mut self) {
        self.snake.update();
        self.snake.check_borders();
        let eaten = self.snake.check_collision(self.food.x, self.food.y);
        if eaten {
            let rnum_x = thread_rng().gen_range(0..20);
            let rnum_y = thread_rng().gen_range(0..20);

            if !self.snake.body.contains(&(rnum_x, rnum_y)) {
                self.food.x = rnum_x as u32;
                self.food.y = rnum_y as u32;
            }

            let snake_last = self.snake.body.back().unwrap().to_owned();
            match &self.snake.dir {
                Direction::Up => self.snake.body.push_back((snake_last.0, snake_last.1 + 1)),
                Direction::Down => self.snake.body.push_back((snake_last.0, snake_last.1 - 1)),
                Direction::Left => self.snake.body.push_back((snake_last.0 - 1, snake_last.1)),
                Direction::Right => self.snake.body.push_back((snake_last.0 + 1, snake_last.1)),
            }
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        let body = &self.snake.body;

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down
                    && body.front().unwrap().clone().1 == body.iter().nth(1).unwrap().clone().1 =>
            {
                Direction::Up
            }
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up
                    && body.front().unwrap().clone().1 == body.iter().nth(1).unwrap().clone().1 =>
            {
                Direction::Down
            }
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left
                    && body.front().unwrap().clone().0 == body.iter().nth(1).unwrap().clone().0 =>
            {
                Direction::Right
            }
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right
                    && body.front().unwrap().clone().0 == body.iter().nth(1).unwrap().clone().0 =>
            {
                Direction::Left
            }
            _ => last_direction,
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(GREEN, square, transform, gl));
        })
    }

    fn check_borders(&mut self) {
        for part in self.body.iter_mut() {
            if part.0 > 19 {
                part.0 = 0
            } else if part.0 < 0 {
                part.0 = 19
            } else if part.1 > 19 {
                part.1 = 0
            } else if part.1 < 0 {
                part.1 = 19
            }
        }
    }

    fn check_collision(&self, x: u32, y: u32) -> bool {
        if self.body.front().unwrap().0 as u32 == x && self.body.front().unwrap().1 as u32 == y {
            return true;
        };
        false
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone(); // clone, or else mutate an existing part

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back(); // make the snake's tail shorten
    }
}

struct Food {
    x: u32,
    y: u32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl)
        })
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap(); // unwrap, else returns result

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(vec![(0, 0), (1, 0)].into_iter()),
            dir: Direction::Right,
        },
        food: Food { x: 1, y: 1 },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button)
            }
        }
    }
}
