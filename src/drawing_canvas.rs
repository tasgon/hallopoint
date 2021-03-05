use ggez::graphics;
use ggez::graphics::{Canvas, DrawParam, FilterMode};
use ggez::mint::Point2;
use graphics::{
    get_window_color_format, screen_coordinates, set_canvas, set_screen_coordinates, Color,
    MeshBuilder, Rect,
};

// This is abhorrent but `https://github.com/ggez/ggez/issues/497` necessitates it.
#[inline(always)]
pub fn with_canvas(
    ctx: &mut ggez::Context,
    canvas: &Canvas,
    mut f: impl FnMut(&mut ggez::Context),
) -> Result<(), ggez::GameError> {
    let scoords = ggez::graphics::screen_coordinates(ctx);
    graphics::set_canvas(ctx, Some(&canvas));
    graphics::set_screen_coordinates(
        ctx,
        Rect::new(
            0f32,
            0f32,
            canvas.image().width() as f32,
            canvas.image().height() as f32,
        ),
    )?;
    f(ctx);
    graphics::set_canvas(ctx, None);
    graphics::set_screen_coordinates(ctx, scoords)?;

    Ok(())
}

pub struct DrawingCanvas {
    canvas: Canvas,
    pub brush_radius: f32,
    pub grid: bool,
}

impl DrawingCanvas {
    pub fn new(ctx: &mut ggez::Context, width: u16, height: u16) -> Self {
        let mut canvas = Canvas::new(
            ctx,
            width,
            height,
            ggez::conf::NumSamples::One,
            get_window_color_format(ctx),
        )
        .unwrap();
        canvas.set_filter(FilterMode::Nearest);
        graphics::set_canvas(ctx, Some(&canvas));
        graphics::clear(ctx, Color::WHITE);
        graphics::set_canvas(ctx, None);
        Self {
            canvas,
            brush_radius: 3.0f32,
            grid: false,
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

        let last_point = last_point.into();
        let radius = self.brush_radius;

        with_canvas(ctx, &self.canvas, move |ctx2| {
            let mut mb = MeshBuilder::new();
            if let Some(lp) = last_point {
                if let Err(e) = mb.line(&[lp, point], radius, color) {
                    println!("{:?}", e);
                }
            }
            match mb.build(ctx2) {
                Ok(mesh) => graphics::draw(ctx2, &mesh, ([0.0, 0.0],)).unwrap(),
                Err(e) => println!("{:?}", e),
            };
        })
        .unwrap();
    }

    pub fn resize(&mut self, ctx: &mut ggez::Context, width: u16, height: u16) {
        let mut canvas2 = Canvas::new(
            ctx,
            width,
            height,
            ggez::conf::NumSamples::One,
            get_window_color_format(ctx),
        )
        .unwrap();
        canvas2.set_filter(FilterMode::Nearest);
        let cv = &self.canvas;

        with_canvas(ctx, &canvas2, |ctx2| {
            ggez::graphics::draw(ctx2, cv, ([0.0f32, 0.0f32],)).unwrap();
        })
        .unwrap();

        self.canvas = canvas2;
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, params: Option<DrawParam>) {
        let params = params.unwrap_or_default();
        graphics::draw(ctx, &self.canvas, params).unwrap();

        if self.grid {
            if let graphics::Transform::Values { scale, .. } = params.trans {
                let mut mb = MeshBuilder::new();
                let w = self.canvas.image().width() as f32 * scale.x;
                let h = self.canvas.image().height() as f32 * scale.y;
                let (mut x, mut y) = (scale.x, scale.y);
                while x < w {
                    mb.line(
                        &[Point2 { x, y: 0.0f32 }, Point2 { x, y: h }],
                        1.0f32,
                        Color::BLACK,
                    )
                    .unwrap();

                    x += scale.x;
                }

                while y < h {
                    mb.line(
                        &[Point2 { x: 0.0f32, y }, Point2 { x: w, y }],
                        1.0f32,
                        Color::BLACK,
                    )
                    .unwrap();

                    y += scale.y;
                }

                let params = params.scale([1.0f32, 1.0f32]);

                let mesh = mb.build(ctx).unwrap();
                ggez::graphics::draw(ctx, &mesh, params).unwrap();
            }
        }
    }
}
