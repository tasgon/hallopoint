extern crate ggez;

mod drawing_canvas;
mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;
use drawing_canvas::DrawingCanvas;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{conf, mint::Point2};
use ggez::{
    event::winit_event::{Force, Touch, WindowEvent},
    graphics::DrawMode,
};
use ggez::{Context, GameResult};

use graphics::{Color, DrawParam, StrokeOptions};
use imgui::im_str;

struct MainState {
    board: DrawingCanvas,
    imgui_wrapper: ImGuiWrapper,
    last_touch: Option<Touch>,
    last_force: f32,
    board_scale: f32,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx, hidpi_factor);
        let s = MainState {
            board: DrawingCanvas::new(ctx, 25, 25),
            imgui_wrapper,
            last_touch: None,
            last_force: 0.0f32,
            board_scale: 1.0f32,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::BLACK);

        // Render game stuff
        {
            self.board.draw(
                ctx,
                Some(DrawParam::default().scale([self.board_scale, self.board_scale])),
            );
            if let Some(t) = self.last_touch {
                let p = Point2 {
                    x: t.location.x as f32,
                    y: t.location.y as f32,
                };
                let cursor = graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::Stroke(StrokeOptions::DEFAULT),
                    p,
                    3f32,
                    1f32,
                    Color::new(1f32, 0f32, 0f32, 1f32),
                )
                .unwrap();
                graphics::draw(ctx, &cursor, ([0.0, 0.0],))?;
            }
        }

        let touch = self.last_touch;
        let mut val = self.board.brush_radius as i32;

        // Render game ui
        {
            self.imgui_wrapper.render(
                ctx,
                (&mut self.board, &mut self.board_scale),
                |ui, c, data| {
                    let (b, s) = data;
                    imgui::Window::new(im_str!("Main")).build(ui, || {
                        if ui.small_button(im_str!("Hide cursor")) {
                            ggez::input::mouse::set_cursor_hidden(c, true);
                        }
                        if ui.small_button(im_str!("Clear")) {
                            *b = DrawingCanvas::new(c, 25, 25);
                        }
                        if let Some(t) = touch {
                            ui.text(im_str!("{:?}\n{:?}", t.location, t.force));
                        }
                        imgui::Drag::new(im_str!("Scale"))
                            .range(1f32..=10f32)
                            .build(ui, s);
                        imgui::Drag::new(im_str!("Stroke size"))
                            .range(1..=10)
                            .build(ui, &mut val);
                    });
                },
            );
        }

        self.board.brush_radius = val as f32;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_up(button);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        //self.board = DrawingCanvas::new(ctx, 25, 25);
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }

    fn winit_event(&mut self, ctx: &mut Context, event: WindowEvent) {
        if let WindowEvent::Touch(tev) = event {
            let pos = tev.location;
            let force = tev.force.map_or(0.0f32, |f| match f {
                Force::Normalized(n) => n as f32,
                _ => 0.0f32,
            });

            let x = pos.x as f32;
            let y = pos.y as f32;
            if force > 0.1f32 {
                self.board.stroke(
                    self.last_touch.map(|t| Point2 {
                        x: t.location.x as f32 / self.board_scale,
                        y: t.location.y as f32 / self.board_scale,
                    }),
                    Point2 {
                        x: x / self.board_scale,
                        y: y / self.board_scale,
                    },
                    force,
                    ctx,
                );
            }

            self.last_touch = Some(tev);
            self.mouse_motion_event(ctx, x, y, 0f32, 0f32);
            if self.last_force < 0.1f32 && force > 0.1f32 {
                self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
            } else if self.last_force > 0.1f32 && force < 0.1f32 {
                self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
            }
            self.last_force = force;
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("hallopoint", "tasgon")
        .window_setup(conf::WindowSetup::default().title("Hallopoint"))
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (mut ctx, event_loop) = cb.build()?;
    //ggez::input::mouse::set_cursor_hidden(&mut ctx, true);

    let hidpi_factor = event_loop.primary_monitor().unwrap().scale_factor() as f32;
    println!("dpi: {}", hidpi_factor);

    let state = MainState::new(&mut ctx, hidpi_factor)?;

    event::run(ctx, event_loop, state)
}
