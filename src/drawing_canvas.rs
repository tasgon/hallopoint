use ggez::graphics::{Canvas, DrawMode, DrawParam, FillOptions};
use ggez::mint::Point2;
use ggez::{graphics, Context};
use graphics::{
    get_window_color_format, screen_coordinates, set_screen_coordinates, size, Color, MeshBuilder,
    Rect,
};

pub struct DrawingCanvas {
    canvas: Canvas,
    pub brush_radius: f32,
}

impl DrawingCanvas {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let canvas = Canvas::new(
            ctx,
            500,
            500,
            ggez::conf::NumSamples::One,
            get_window_color_format(ctx),
        )
        .unwrap();
        graphics::set_canvas(ctx, Some(&canvas));
        graphics::clear(ctx, Color::WHITE);
        graphics::set_canvas(ctx, None);
        Self {
            canvas,
            brush_radius: 3.0f32,
        }
    }

    pub fn stroke(
        &mut self,
        last_point: impl Into<Option<Point2<f32>>>,
        point: impl Into<Point2<f32>>,
        pressure: f32,
        ctx: &mut ggez::Context,
    ) {
        let point = point.into();
        let old_rect = graphics::screen_coordinates(ctx);
        let color = graphics::Color::new(0.0f32, 0.0f32, 0.0f32, pressure);
        graphics::set_canvas(ctx, Some(&self.canvas));

        // This is abhorrent but `https://github.com/ggez/ggez/issues/497` necessitates it.
        graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, 500f32, 500f32)).unwrap();

        let mut mb = MeshBuilder::new();
        if let Some(lp) = last_point.into() {
            if let Err(e) = mb.line(&[lp, point], self.brush_radius * 2.0f32, color) {
                println!("{:?}", e);
            }
        }
        if let Ok(mesh) = mb.build(ctx) {
            graphics::draw(ctx, &mesh, ([0.0, 0.0],)).unwrap();
        }
        graphics::set_canvas(ctx, None);
        graphics::set_screen_coordinates(ctx, old_rect).unwrap();
    }

    pub fn clear(&mut self, ctx: &mut ggez::Context) {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, Color::WHITE);
        graphics::set_canvas(ctx, None);
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, params: Option<DrawParam>) {
        let params = params.unwrap_or_default();
        graphics::draw(ctx, &self.canvas, params).unwrap();
    }
}
