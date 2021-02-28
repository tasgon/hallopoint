use ggez::graphics;
use ggez::graphics::{Canvas, DrawMode, DrawParam, FillOptions};
use ggez::mint::Point2;
use graphics::{get_window_color_format, Color, MeshBuilder};

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
            brush_radius: 6.0f32,
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
        let color = graphics::Color::new(0.0f32, 0.0f32, 0.0f32, pressure);
        graphics::set_canvas(ctx, Some(&self.canvas));
        let mut mb = MeshBuilder::new();
        if let Some(lp) = last_point.into() {
            let dx = point.x - lp.x;
            let dy = point.y - lp.y;
            if (dx * dx + dy * dy).sqrt() > 4.0f32 {
                mb.line(&[lp, point], self.brush_radius * 2.0f32, color)
                    .unwrap();
            }
        }
        mb.circle(
            DrawMode::Fill(FillOptions::DEFAULT),
            point,
            self.brush_radius,
            1.0f32,
            color,
        )
        .unwrap();
        let mesh = mb.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, ([0.0, 0.0],)).unwrap();
        graphics::set_canvas(ctx, None);
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, params: Option<DrawParam>) {
        let params = params.unwrap_or_default();
        graphics::draw(ctx, &self.canvas, params).unwrap();
    }
}
