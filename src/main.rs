use svg::Document;
use svg::node::element::*;


const PAPER_SIZE: (f32, f32) = (215.9, 279.4);
const OUTTER_MARGINS: (f32, f32) = (10.0,10.0);
const INNER_MARGINS: (f32, f32) = (5.0,5.0);
const CELL_SIZE: (f32, f32) = (5.0,5.0);
const MID_SPLIT_WIDTH: f32 = 4.0;

fn mm(x: f32) -> String {
    format!("{}mm", x)
}

fn dot(color: &str) -> Circle {
    return Circle::new()
        .set("stroke-width", 1)
        .set("fill", color)
        .set("r", "0.25mm");
}

struct GridSettings<'a> {
    color: &'a str,
    offset: (f32, f32),
    num_cells: (usize, usize)
}

trait GridPaper {
    fn header(self, grid_settings: &GridSettings) -> Self;
    fn grid(self, grid_settings: &GridSettings) -> Self;
    fn default_page(self, offset: (f32, f32), color: &str) -> Self;
    fn calendar(self, grid_settings: &GridSettings) -> Self;
    fn calendar_page(self, offset: (f32, f32), color: &str) -> Self;
    fn cut_lines(self, color: &str) -> Self;
    fn full_calendar_page(self, color: &str) -> Self;
    fn full_default_page(self, color: &str) -> Self;
}

impl GridPaper for SVG {
    fn header(self, grid_settings: &GridSettings) -> Self {
        let right_box_width = 5.0;
        let mid_box_width = 2.0;
        let left_box_width = (grid_settings.num_cells.0 as f32) - right_box_width - mid_box_width - 1.0;

        self.add(
            Rectangle::new() // Left Box
                .set("width", mm(CELL_SIZE.0 * left_box_width))
                .set("height", mm(CELL_SIZE.1))
                .set("x", mm(grid_settings.offset.0))
                .set("y", mm(grid_settings.offset.1))
                .set("fill", "none")
                .set("stroke-width", "0.25mm")
                .set("stroke", grid_settings.color)
        ).add(
            Rectangle::new() // Mid Box
                .set("width", mm(CELL_SIZE.0 * mid_box_width))
                .set("height", mm(CELL_SIZE.1))
                .set("x", mm(grid_settings.offset.0 + CELL_SIZE.0 * left_box_width))
                .set("y", mm(grid_settings.offset.1))
                .set("fill", "none")
                .set("stroke-width", "0.25mm")
                .set("stroke", grid_settings.color)
        ).add(
            Rectangle::new() // Right Box
                .set("width", mm(CELL_SIZE.0 * right_box_width))
                .set("height", mm(CELL_SIZE.1))
                .set("x", mm(grid_settings.offset.0 + CELL_SIZE.0 * (left_box_width + mid_box_width)))
                .set("y", mm(grid_settings.offset.1))
                .set("fill", "none")
                .set("stroke-width", "0.25mm")
                .set("stroke", grid_settings.color)
        )
    }

    fn grid(self, grid_settings: &GridSettings) -> Self {
        let mut new_doc = self;
    
        for x in 0..grid_settings.num_cells.0 {
            for y in 0..grid_settings.num_cells.1 {
                let loc = (
                    (x as f32) * CELL_SIZE.0 + grid_settings.offset.0, 
                    (y as f32) * CELL_SIZE.1 + grid_settings.offset.1
                );
                new_doc = new_doc.add(
                    dot(grid_settings.color)
                        .set("cx", mm(loc.0))
                        .set("cy", mm(loc.1))
                );
            }
        }
    
        return new_doc;
    }

    fn default_page(self, offset: (f32, f32), color: &str) -> Self {
        let num_cells = (
            ((PAPER_SIZE.0/2.0 - OUTTER_MARGINS.0 - INNER_MARGINS.0) / CELL_SIZE.0).ceil() as usize,
            ((PAPER_SIZE.1/2.0 - OUTTER_MARGINS.1 - INNER_MARGINS.1) / CELL_SIZE.1).ceil() as usize
        );

        let grid_settings = GridSettings { 
            color: color, offset: offset, num_cells: num_cells 
        };

        self.grid(&grid_settings)
            .header(&grid_settings)
    }

    fn calendar(self, grid_settings: &GridSettings) -> Self {
        let mut doc = self;

        let box_width = 2.0;
        let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let start_cell = (
            CELL_SIZE.0 * (grid_settings.num_cells.0 - (2*days.len()) - 1) as f32 + grid_settings.offset.0, 
            CELL_SIZE.1 * 3.0 + grid_settings.offset.1
        );
        let rows = vec![
            "Art", "Brush Teeth", "Chores", "Dread", "Entropy", "Flee", 
            "Grand", "Immediate", "", "", "", "", ""
        ];
        
        for (c, day) in days.iter().enumerate() {
            doc = doc.add(
                Text::new(day.to_string())
                    .set("x", mm(start_cell.0 + CELL_SIZE.0 * box_width * (c as f32)))
                    .set("y", mm(start_cell.1 - 1.0))
                    .set("style", format!("fill: {}; font-weight: lighter; font-size: 14px;", grid_settings.color))
            );
            
        }

        for (r, row) in rows.iter().enumerate() {
            doc = doc.add(
                Text::new(row.to_string())
                    .set("x", mm(grid_settings.offset.0))
                    .set("y", mm(start_cell.1 + CELL_SIZE.1 * ((r + 1) as f32) - 1.0))
                    .set("style", format!("fill: {}; font-weight: lighter; font-size: 14px;", grid_settings.color))
            );
            for (c, _day) in days.iter().enumerate() {
                doc = doc.add(
                    Rectangle::new()
                        .set("width", mm(box_width * CELL_SIZE.0))
                        .set("height", mm(CELL_SIZE.1))
                        .set("x", mm(start_cell.0 + CELL_SIZE.0 * box_width * (c as f32)))
                        .set("y", mm(start_cell.1 + CELL_SIZE.1 * (r as f32)))
                        .set("fill", "none")
                        .set("stroke-width", "0.25mm")
                        .set("stroke", grid_settings.color)
                );
            }
        }

        return doc;
    }

    fn calendar_page(self, offset: (f32, f32), color: &str) -> Self {
        let num_cells = (
            ((PAPER_SIZE.0/2.0 - OUTTER_MARGINS.0 - INNER_MARGINS.0) / CELL_SIZE.0).ceil() as usize,
            ((PAPER_SIZE.1/2.0 - OUTTER_MARGINS.1 - INNER_MARGINS.1) / CELL_SIZE.1).ceil() as usize
        );

        let grid_settings = GridSettings { 
            color: color, offset: offset, num_cells: num_cells 
        };

        self.grid(&grid_settings)
            .header(&grid_settings)
            .calendar(&grid_settings)
    }

    fn cut_lines(self, color: &str) -> Self {
        self.add(
            Line::new() // Left Vert Line
                .set("x1", mm(PAPER_SIZE.0/2.0-2.0))
                .set("x2", mm(PAPER_SIZE.0/2.0-2.0))
                .set("y1", "0mm")
                .set("y2", mm(PAPER_SIZE.1))
                .set("stroke-width", "0.25mm")
                .set("stroke", color)
        ).add(
            Line::new()// Right Vert Line
                .set("x1", mm(PAPER_SIZE.0/2.0+2.0))
                .set("x2", mm(PAPER_SIZE.0/2.0+2.0))
                .set("y1", "0mm")
                .set("y2", mm(PAPER_SIZE.1))
                .set("stroke-width", "0.25mm")
                .set("stroke", color)
        ).add(
            Line::new()// Horiz Line
                .set("y1", mm(PAPER_SIZE.1/2.0))
                .set("y2", mm(PAPER_SIZE.1/2.0))
                .set("x1", "0mm")
                .set("x2", mm(PAPER_SIZE.0))
                .set("stroke-width", "0.25mm")
                .set("stroke", color)
        )
    }

    fn full_calendar_page(self, color: &str) -> Self {
        self.calendar_page(OUTTER_MARGINS, color)
            .calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0, OUTTER_MARGINS.1), color)
            .calendar_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .cut_lines(color)
    }

    fn full_default_page(self, color: &str) -> Self {
        self.default_page(OUTTER_MARGINS, color)
            .default_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0, OUTTER_MARGINS.1), color)
            .default_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .default_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .cut_lines(color)
    }
}


fn main() {
    svg::save("grid_paper_red.svg", &Document::new().full_default_page("red"))
        .and(svg::save("grid_paper_green.svg", &Document::new().full_default_page("green")))
        .and(svg::save("grid_paper_blue.svg", &Document::new().full_default_page("blue")))
        .and(svg::save("calendar_blue.svg", &Document::new().full_calendar_page("blue")))
        .expect("Files save correctly");
}
