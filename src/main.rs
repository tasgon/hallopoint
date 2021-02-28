extern crate ggez;

mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;
use ggez::conf;
use ggez::event::winit_event::{Force, Touch, WindowEvent};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};

use imgui::im_str;

struct MainState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    last_touch: Option<Touch>,
    last_force: f32,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx, hidpi_factor);
        let s = MainState {
            pos_x: 0.0,
            imgui_wrapper,
            last_touch: None,
            last_force: 0.0f32,
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
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [self.pos_x, 380.0f32],
                100.0,
                2.0,
                graphics::Color::WHITE,
            )?;
            graphics::draw(ctx, &circle, ([0.0, 0.0],))?;
        }

        let touch = self.last_touch;

        // Render game ui
        {
            self.imgui_wrapper.render(ctx, |ui| {
                ui.show_demo_window(&mut true);
                imgui::Window::new(im_str!("Main")).build(ui, || {
                    if ui.small_button(im_str!("Open song")) {}
                    ui.text(im_str!("{:?}", touch));
                });
            });
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        //let x = x / 2f32;
        //let y = y / 2f32;
        self.pos_x = x;
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
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
        println!("{}", width);
        //println!("{:?}", graphics::screen_coordinates(ctx));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }

    fn winit_event(&mut self, _ctx: &mut Context, event: WindowEvent) {
        if let WindowEvent::Touch(tev) = event {
            let pos = tev.location;
            let force = tev.force.map_or(0.0f32, |f| match f {
                Force::Normalized(n) => n as f32,
                _ => 0.0f32,
            });
            let x = pos.x as f32;
            let y = pos.y as f32;
            self.last_touch = Some(tev);
            self.mouse_motion_event(_ctx, x, y, 0f32, 0f32);
            if self.last_force < 0.1f32 && force > 0.1f32 {
                self.mouse_button_down_event(_ctx, MouseButton::Left, x, y);
            } else if self.last_force > 0.1f32 && force < 0.1f32 {
                self.mouse_button_up_event(_ctx, MouseButton::Left, x, y);
            }
            self.last_force = force;
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple with imgui", "ggez")
        .window_setup(conf::WindowSetup::default().title("super_simple with imgui"))
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (mut ctx, event_loop) = cb.build()?;

    let hidpi_factor = event_loop.primary_monitor().unwrap().scale_factor() as f32;

    let state = MainState::new(&mut ctx, hidpi_factor)?;

    event::run(ctx, event_loop, state)
}
