use gtk::prelude::*;
use gtk::DrawingArea;

use std::rc::Rc;

pub struct Graph {
    area: DrawingArea,
    scale_x_start: f64,
    scale_x_end: f64,
    scale_y_start: f64,
    scale_y_end: f64,
    lines: Vec<Line>
}

pub struct Line {
    points: Vec<(f64,f64)>,
    color: (f64, f64, f64)
}

impl Line {
    pub fn new(r: f64, g: f64, b: f64, points: Vec<(f64,f64)>) -> Self {
        Line {
            points,
            color: (r,g,b)
        }
    }
}

impl Graph {
    pub fn new(area: DrawingArea, 
        scale_x_start: f64,
        scale_x_end: f64,
        scale_y_start: f64,
        scale_y_end: f64,
        lines: Vec<Line>) -> Rc<Self> {

        let graph = Rc::new(Graph {
            area,
            scale_x_start,
            scale_x_end,
            scale_y_start,
            scale_y_end,
            lines
        });

        let graph_tmp = Rc::clone(&graph);
        graph.area.connect_draw(move |area,ctx| {
            Graph::draw(area, ctx, &graph_tmp);
            Inhibit(false)
        });

        graph
    }

    fn draw(area: &gtk::DrawingArea, 
        ctx:  &cairo::Context, 
        graph: &Rc<Graph>) {

        ctx.set_source_rgb(0.1, 0.5, 0.5);
        ctx.paint();

        let width = area.get_allocated_width() as f64;
        let height = area.get_allocated_height() as f64;
        
        Graph::draw_boxes(ctx, width, height, 40.0, 20.0, 5.0, 0.3);
        Graph::draw_boxes(ctx, width, height, 40.0, 20.0, 50.0, 0.1);

        let cell_size = 50.0;

        let v_bars = math::round::ceil(width/cell_size, 0) as i32;
        let h_bars= math::round::ceil(height/cell_size, 0) as i32;

        let h_scale = (graph.scale_x_end - graph.scale_x_start)/(v_bars - 1) as f64; // ms

        ctx.set_source_rgb(1.0, 1.0, 1.0);
        for x in 0..v_bars {
            let text = math::round::floor(x as f64 * h_scale + graph.scale_x_start, 1).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(40.0 + x as f64 * cell_size - f.width, height - 10.0);
            ctx.show_text(&text);
        }

        let v_scale = (graph.scale_y_end - graph.scale_y_start)/(h_bars - 1) as f64; // ms

        for y in (0..h_bars).rev() {
            let text = math::round::floor(y as f64 * v_scale + graph.scale_y_start, 1).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(40.0 - f.width, height - y as f64 * cell_size - f.height - 15.0);
            ctx.show_text(&text);
        }

        ctx.set_line_width(2.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        for line in graph.lines.iter() {
            ctx.set_source_rgb(line.color.0, line.color.1, line.color.2);
            for p in line.points.iter().skip(1).enumerate() { 
                let xp = line.points[p.0];
                ctx.move_to(((cell_size as f64)*(xp.0 - graph.scale_x_start))/h_scale + 40.0, height - ((cell_size as f64)*(xp.1 - graph.scale_y_start))/v_scale - 20.0);
                ctx.line_to(((cell_size as f64)*(p.1.0 - graph.scale_x_start))/h_scale + 40.0, height - ((cell_size as f64)*(p.1.1 - graph.scale_y_start))/v_scale - 20.0);
            }
            ctx.stroke();
        }
    }

    fn draw_boxes(ctx: &cairo::Context, area_width: f64, area_height: f64, src_x: f64, src_y: f64, cell_size: f64, color: f64) {
        ctx.set_source_rgb(color, color, color);
        ctx.set_line_width(1.0);
        for x  in 1..math::round::ceil(area_width/cell_size, 0) as i32 {
            let xi = x as f64 * cell_size + src_x;
            ctx.move_to(xi, 0.0);
            ctx.line_to(xi, area_height - src_y);
        }
        for y  in 1..math::round::ceil(area_height/cell_size, 0) as i32 {
            let yi = area_height - y as f64 * cell_size - src_y;
            ctx.move_to(src_x, yi);
            ctx.line_to(area_width, yi);
        }
        ctx.stroke();
    }
}
