//! This part do manage most of work realted to drawing graph

use gtk::prelude::*;
use gtk::DrawingArea;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
/// A single line
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
/// Tools to draw Graph
pub struct Graph {
    pub area: DrawingArea,
    pub scale_x_start: f64,             // start of x on pankti
    pub scale_x_size: f64,              // size of pankti to show
    pub scale_y_start: f64,             // start of y on stambh
    pub scale_y_size: f64,              // size of stambh to show
    pub draw_patch: bool,               // enable to draw circle spot on line
    pub draw_box: bool,                 // enable to show boxes linke graph paper
    pub draw_baarik_box: bool,          // enable to show baarik(similar meaning to smaller) linke graph paper
    pub auto_adjust_y: bool,            // enable to automatically adjust y axis
    pub lines: HashMap<String, Line>,   
    pub pankti_sankya: f64              // use used while adding to point in lines to see last count of graphable input
}

impl Graph {
    pub fn new(area: DrawingArea, 
        scale_x_start: f64,
        scale_x_size: f64,
        scale_y_start: f64,
        scale_y_size: f64,
        draw_patch: bool,
        draw_box: bool,
        draw_baarik_box: bool,
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
            draw_box,
            draw_baarik_box,
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

    /// used to draw box and baarik box
    fn draw_boxes(ctx: &cairo::Context, area_width: f64, area_height: f64, src_x: f64, cell_size: f64, color: f64) {
        ctx.set_source_rgb(color, color, color);
        ctx.set_line_width(1.0);
        // lines parallel to stambh
        for i  in 1..math::round::ceil(area_width/cell_size, 0) as i32 {
            let xi = i as f64 * cell_size + src_x;
            ctx.move_to(xi, 0.0);
            ctx.line_to(xi, area_height);
        }
        // lines parallel to pankti
        for i  in 1..math::round::ceil(area_height/cell_size, 0) as i32 {
            let yi = area_height - i as f64 * cell_size;
            ctx.move_to(src_x, yi);
            ctx.line_to(area_width + src_x, yi);
        }
        ctx.stroke();
    }

    /// transform point to show on graph
    fn transform_on_graph(
        p_start: f64,
        s_start: f64, 
        p: f64, 
        s: f64, 
        aa_dumm_pankti: f64, 
        aa_dumm_stambh: f64, 
        scale_x_size: f64,
        scale_y_size: f64,
        height: f64,
        stambh_scale_width: f64) -> (f64, f64){
        (
            ((p - p_start) * aa_dumm_pankti)/scale_x_size + stambh_scale_width,
            height - ((s - s_start) * aa_dumm_stambh) / scale_y_size
        )
    }

    /// callback of drawing area to draw graph
    fn draw(area: &gtk::DrawingArea, 
        ctx:  &cairo::Context, 
        graph: &Rc<RefCell<Graph>>) {
        
        let graph = graph.borrow();

        ctx.set_source_rgb(0.1, 0.5, 0.5);
        ctx.paint();
        
        let pankti_scale_height = 50.0;
        let stambh_scale_width = 60.0;
        let width = area.get_allocated_width() as f64 - stambh_scale_width;
        let height = area.get_allocated_height() as f64 - pankti_scale_height;
        
        let manjusa_maap = 50.0;

        let rekha_sankhya_pankti = math::round::floor(width / manjusa_maap, 0);
        let rekha_sankhya_stambh = math::round::floor(height / manjusa_maap, 0);

        let aa_dumm_pankti = rekha_sankhya_pankti * manjusa_maap;
        let aa_dumm_stambh = rekha_sankhya_stambh * manjusa_maap;

        let anupat_pankti = (manjusa_maap * graph.scale_x_size) / aa_dumm_pankti;
        let anupat_stambh = (manjusa_maap * graph.scale_y_size) / aa_dumm_stambh;

        // drawing boxes to show area as graph paper
        if graph.draw_box {
            if graph.draw_baarik_box {
                Graph::draw_boxes(ctx, width, height, stambh_scale_width, 5.0, 0.3);
            }
            Graph::draw_boxes(ctx, width, height, stambh_scale_width, 50.0, 0.1);
        }

        // Drawing point and line on graph area
        ctx.set_line_width(2.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        let draw_patch = graph.draw_patch;
        for (_,line) in graph.lines.iter() {
            for (i, (p,s)) in line.points.iter().enumerate() { 
                // check if point is last poin on line
                let (p_dumm, s_dumm) = if i < line.points.len() - 1  {
                    line.points[i + 1] 
                } else {
                    line.points[i] 
                };

                let bindu_t = Graph::transform_on_graph(
                    graph.scale_x_start,
                    graph.scale_y_start,
                    *p,
                    *s,
                    aa_dumm_pankti,
                    aa_dumm_stambh,
                    graph.scale_x_size,
                    graph.scale_y_size,
                    height,
                    stambh_scale_width,
                );

                let bindu_dumm_t = Graph::transform_on_graph(
                    graph.scale_x_start,
                    graph.scale_y_start,
                    p_dumm,
                    s_dumm,
                    aa_dumm_pankti,
                    aa_dumm_stambh,
                    graph.scale_x_size,
                    graph.scale_y_size,
                    height,
                    stambh_scale_width,
                );

                ctx.set_source_rgb(line.color.0, line.color.1, line.color.2);
                ctx.move_to(bindu_dumm_t.0, bindu_dumm_t.1);
                ctx.line_to(bindu_t.0, bindu_t.1);
                ctx.stroke();

                // draw circle over point
                if draw_patch {
                    ctx.set_source_rgb(0.0, 0.0, 1.0);
                    ctx.arc(bindu_dumm_t.0, bindu_dumm_t.1,
                        5.0, 0.0, std::f64::consts::PI * 2.0);
                    ctx.stroke();
                }
            }
        }

        // draw darker recragle over scales
        ctx.set_source_rgb(0.1, 0.4, 0.4);
        ctx.rectangle(0.0, 0.0, stambh_scale_width, height + pankti_scale_height);
        ctx.rectangle(stambh_scale_width, height, width, pankti_scale_height);
        ctx.fill();

        ctx.set_source_rgb(1.0, 1.0, 1.0);
        // write numbers on pankti scale
        for i in 0..(rekha_sankhya_pankti as i32 + 1) {
            let text = math::round::floor(i as f64 * anupat_pankti + graph.scale_x_start, 4).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(i as f64 * manjusa_maap - f.width + stambh_scale_width + f.height / 0.866, height  + f.width * 0.5 + f.height);
            ctx.save();
            ctx.rotate(std::f64::consts::PI / -6.0);
            ctx.show_text(&text);
            ctx.restore();
        }
        // write numbers on stambh scale
        for i in (0..aa_dumm_stambh as i32 + 1).rev() {
            let text = math::round::floor(i as f64 * anupat_stambh + graph.scale_y_start, 4).to_string();
            let f = ctx.text_extents(&text);
            ctx.move_to(stambh_scale_width - f.width, height - i as f64 * manjusa_maap + f.width * 0.5);
            ctx.save();
            ctx.rotate(std::f64::consts::PI / -6.0);
            ctx.show_text(&text);
            ctx.restore();
        }
    }

    /// Adjust stambh and pankti as needed , trim lines and redraws
    pub fn redraw(&mut self) {
        let (mx_x, mi_x, mx_y, mi_y) = self.get_extremes();

        // stambh
        if self.auto_adjust_y {
            let spread = (mx_y - mi_y).abs();
            self.scale_y_start = mi_y - spread * 0.1;
            self.scale_y_size = spread * 1.2;
        }
        
        // pankti
        let spread = (mx_x - mi_x).abs();
        if spread <  self.scale_x_size {
            self.scale_x_start = mi_x; 
        } else {
            self.scale_x_start =  mx_x - self.scale_x_size;
        }

        self.trim_lines(); // trim
        self.area.queue_draw(); // redraw
    }

    /// find minimun and maximum value from all lines
    pub fn get_extremes(&self) -> (f64, f64, f64, f64){
        // trick to avoid no lines 
        if self.lines.len() == 0 {
            return (0.0,0.0,0.0,0.0);
        }

        let mut mx_x:Option<f64> = None;
        let mut mi_x:Option<f64> = None;
        let mut mx_y:Option<f64> = None;
        let mut mi_y:Option<f64> = None;

        for (_, line) in self.lines.iter() {
            if line.points.len() == 0 {
                mx_x = Some(0.0);
                mi_x = Some(0.0);
                mx_y = Some(0.0);
                mi_y = Some(0.0);
                continue;
            }

            // keeping these in here is a good idea than picking line[0].point[0]
            // because consider if 0th line have no points and 1st too than that fails
            // and also if we keep 0 as default value to start then a graph with
            // 100 to 200 will have 0 as minimum
            if let None = mx_x {
                mx_x = Some(line.points[0].0);
            }

            if let None = mi_x {
                mi_x = Some(line.points[0].0);
            }

            if let None = mx_y {
                mx_y = Some(line.points[0].1);
            }

            if let None = mi_y {
                mi_y = Some(line.points[0].1);
            }

            for (x,y) in line.points.iter().skip(1) {
                mx_x = Some(f64::max(mx_x.unwrap(), *x));
                mi_x = Some(f64::min(mi_x.unwrap(), *x));
                mx_y = Some(f64::max(mx_y.unwrap(), *y));
                mi_y = Some(f64::min(mi_y.unwrap(), *y));
            }
        }

        (mx_x.unwrap(), mi_x.unwrap(), mx_y.unwrap(), mi_y.unwrap())
    }

    // tims line form left side of line. Why left?? to avoid wasting time in ponits in range
    // it skips i point out of screen to keep a non terminating line experience
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
