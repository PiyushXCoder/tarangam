use gtk::prelude::*;
use gtk::DrawingArea;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Graph {
    pub area: DrawingArea,
    pub scale_x_start: f64,
    pub scale_x_size: f64,
    pub scale_y_start: f64,
    pub scale_y_size: f64,
    pub draw_patch: bool,
    pub auto_adjust_y: bool,
    pub lines: HashMap<String, Line>,
    pub pankti_sankya: f64
}

pub struct Line {
    pub points: Vec<(f64,f64)>,
    pub color: (f64, f64, f64)
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
        scale_x_size: f64,
        scale_y_start: f64,
        scale_y_size: f64,
        draw_patch: bool,
        auto_adjust_y: bool,
        lines: HashMap<String, Line>,
        pankti_sankya: f64) -> Rc<RefCell<Self>> {

        let graph = Rc::new(RefCell::new(Graph {
            area,
            scale_x_start,
            scale_x_size,
            scale_y_start,
            scale_y_size,
            draw_patch,
            auto_adjust_y,
            lines,
            pankti_sankya
        }));

        let graph_tmp = Rc::clone(&graph);
        graph.borrow().area.connect_draw(move |area,ctx| {
            Graph::draw(area, ctx, &graph_tmp);
            Inhibit(false)
        });

        graph
    }

    fn draw(area: &gtk::DrawingArea, 
        ctx:  &cairo::Context, 
        graph: &Rc<RefCell<Graph>>) {
        
        ctx.set_source_rgb(0.1, 0.5, 0.5);
        ctx.paint();

        let width = area.get_allocated_width() as f64;
        let height = area.get_allocated_height() as f64;
        
        if graph.borrow().auto_adjust_y {
            graph.borrow_mut().adjust_scale_automatic_y();
        }
        graph.borrow_mut().adjust_scale_automatic_x();
        graph.borrow_mut().trim_lines();

        Graph::draw_boxes(ctx, width, height, 40.0, 20.0, 5.0, 0.3);
        Graph::draw_boxes(ctx, width, height, 40.0, 20.0, 50.0, 0.1);
        
        let cell_size = 50.0;

        let v_bars = math::round::ceil(width/cell_size, 0) as i32;
        let h_bars= math::round::ceil(height/cell_size, 0) as i32;

        let h_scale = (graph.borrow().scale_x_size)/(v_bars - 1) as f64; // ms
        let v_scale = (graph.borrow().scale_y_size)/(h_bars - 1) as f64; // ms
        
        ctx.set_line_width(2.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        let draw_patch = graph.borrow().draw_patch;
        for (_,line) in graph.borrow().lines.iter() {
            for p in line.points.iter().enumerate() { 
                let xp = if p.0 < line.points.len() - 1  {
                    line.points[p.0 + 1] 
                } else {
                    line.points[p.0] 
                };
                ctx.set_source_rgb(line.color.0, line.color.1, line.color.2);
                ctx.move_to(((cell_size as f64)*(xp.0 - graph.borrow().scale_x_start))/h_scale + 40.0, height - ((cell_size as f64)*(xp.1 - graph.borrow().scale_y_start))/v_scale - 20.0);
                ctx.line_to(((cell_size as f64)*(p.1.0 - graph.borrow().scale_x_start))/h_scale + 40.0, height - ((cell_size as f64)*(p.1.1 - graph.borrow().scale_y_start))/v_scale - 20.0);
                ctx.stroke();

                if draw_patch {
                    ctx.set_source_rgb(0.0, 0.0, 1.0);
                    ctx.arc(((cell_size as f64)*(p.1.0 - graph.borrow().scale_x_start))/h_scale + 40.0, height - ((cell_size as f64)*(p.1.1 - graph.borrow().scale_y_start))/v_scale - 20.0, 5.0, 0.0, std::f64::consts::PI * 2.0);
                    ctx.stroke();
                }
            }
        }

        ctx.set_source_rgb(0.1, 0.4, 0.4);
        ctx.rectangle(0.0, 0.0, 40.0, height);
        ctx.rectangle(0.0, height - 20.0, width, 20.0);
        ctx.fill();

        ctx.set_source_rgb(1.0, 1.0, 1.0);
        for x in 0..v_bars {
            let text = math::round::floor(x as f64 * h_scale + graph.borrow().scale_x_start, 1).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(40.0 + x as f64 * cell_size - f.width, height - 10.0);
            ctx.show_text(&text);
        }

        for y in (0..h_bars).rev() {
            let text = math::round::floor(y as f64 * v_scale + graph.borrow().scale_y_start, 1).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(40.0 - f.width, height - y as f64 * cell_size - f.height - 15.0);
            ctx.show_text(&text);
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

    pub fn adjust_scale_automatic_y(&mut self) {
        if self.lines.len() == 0 {
            return;
        }

        let mut mx:Option<f64> = None;
        let mut mi:Option<f64> = None;
        
        for (_, line) in self.lines.iter() {
            if line.points.len() == 0 {
                mx = Some(0.0);
                mi = Some(0.0);
                continue;
            }

            if let None = mx {
                mx = Some(line.points[0].1);
            }

            if let None = mi {
                mi = Some(line.points[0].1);
            }

            for (_,y) in line.points.iter().skip(1) {
                mx = Some(f64::max(mx.unwrap(), *y));
                mi = Some(f64::min(mi.unwrap(), *y));
            }
        }

        let mx  = mx.unwrap();
        let mi  = mi.unwrap();

        let spread = (mx - mi).abs();
         
        self.scale_y_start = mi - spread * 0.1;
        self.scale_y_size = spread * 1.2;
    }

    pub fn adjust_scale_automatic_x(&mut self) {
        if self.lines.len() == 0 {
            return;
        }

        let mut mx:Option<f64> = None;
        let mut mi:Option<f64> = None;
        
        for (_, line) in self.lines.iter() {
            if line.points.len() == 0 {
                mx = Some(0.0);
                mi = Some(0.0);
                continue;
            }

            if let None = mx {
                mx = Some(line.points[0].0);
            }

            if let None = mi {
                mi = Some(line.points[0].0);
            }

            for (x,_) in line.points.iter().skip(1) {
                mx = Some(f64::max(mx.unwrap(), *x));
                mi = Some(f64::min(mi.unwrap(), *x));
            }
        }

        let spread = (mx.unwrap() - mi.unwrap()).abs();

        if spread <  self.scale_x_size {
            self.scale_x_start = mi.unwrap();
        } else {
            self.scale_x_start =  mx.unwrap() - self.scale_x_size;
        }
    }

    pub fn trim_lines(&mut self) {
        for (_, line) in self.lines.iter_mut() {
            let mut i = 0;
            while i < line.points.len() {
                match line.points.get(i + 2) {
                    Some(_) => {
                        if line.points[i+1].0 < self.scale_x_start {
                            line.points.remove(i);
                        }
                    },
                    None => {
                        if line.points[line.points.len() - 1].0 < self.scale_x_start {
                            line.points.clear();
                            break;
                        }
                    }
                }
                i += 1;
            }
        }
    }
}
