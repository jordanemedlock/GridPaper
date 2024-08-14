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

struct CalendarSettings<'a> {
    columns: Vec<&'a str>,
    rows: Vec<&'a str>
}

struct TextSettings<'a> {
    text: &'a str,
    grid_loc: (i32, i32)
}


trait GridPaper {
    fn header(self, grid_settings: &GridSettings) -> Self;
    fn grid(self, grid_settings: &GridSettings) -> Self;
    fn calendar(self, grid_settings: &GridSettings, calendar_settings: &CalendarSettings) -> Self;

    fn cut_lines(self, color: &str) -> Self;
    
    fn default_page(self, offset: (f32, f32), color: &str) -> Self;
    fn goals_page(self, offset: (f32, f32), color: &str) -> Self;
    fn calendar_page(self, offset: (f32, f32), color: &str, calendar_settings: &CalendarSettings) -> Self;

    fn full_calendar_page(self, color: &str, calendar_settings: &CalendarSettings) -> Self;
    fn full_default_page(self, color: &str) -> Self;
    fn full_goals_page(self, color: &str) -> Self;
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

    fn calendar(self, grid_settings: &GridSettings, calendar_settings: &CalendarSettings) -> Self {
        let mut doc = self;

        let box_width = 2.0;
        let days = &calendar_settings.columns;
        let start_cell = (
            CELL_SIZE.0 * (grid_settings.num_cells.0 - (2*days.len()) - 1) as f32 + grid_settings.offset.0, 
            CELL_SIZE.1 * 3.0 + grid_settings.offset.1
        );


        let rows = &calendar_settings.rows;
        
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

    fn calendar_page(self, offset: (f32, f32), color: &str, calendar_settings: &CalendarSettings) -> Self {
        let num_cells = (
            ((PAPER_SIZE.0/2.0 - OUTTER_MARGINS.0 - INNER_MARGINS.0) / CELL_SIZE.0).ceil() as usize,
            ((PAPER_SIZE.1/2.0 - OUTTER_MARGINS.1 - INNER_MARGINS.1) / CELL_SIZE.1).ceil() as usize
        );

        let grid_settings = GridSettings { 
            color: color, offset: offset, num_cells: num_cells 
        };

        self.grid(&grid_settings)
            .header(&grid_settings)
            .calendar(&grid_settings, calendar_settings)
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

    fn full_calendar_page(self, color: &str, calendar_settings: &CalendarSettings) -> Self {
        self.calendar_page(OUTTER_MARGINS, color, calendar_settings)
            .calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0, OUTTER_MARGINS.1), color, calendar_settings)
            .calendar_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color, calendar_settings)
            .calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color, calendar_settings)
            .cut_lines(color)
    }

    fn full_default_page(self, color: &str) -> Self {
        self.default_page(OUTTER_MARGINS, color)
            .default_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0, OUTTER_MARGINS.1), color)
            .default_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .default_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .cut_lines(color)
    }

    fn goals_page(self, offset: (f32, f32), color: &str) -> Self {
        let questions = vec![
            TextSettings { text: "Goal:", grid_loc: (0, 1) },
            TextSettings { text: "Values underlying my goal:", grid_loc: (0,2) },
            TextSettings { text: "Actions to take to acheive that goal:", grid_loc: (0, 4) },
            TextSettings { text: "The * I'm willing to make room for:", grid_loc: (0, 8) },
            TextSettings { text: "Thoughts/memories:", grid_loc: (0, 9) },
            TextSettings { text: "Feelings:", grid_loc: (0, 11) },
            TextSettings { text: "Sensations/Urges:", grid_loc: (0, 13) },
            TextSettings { text: "It would be useful to remind myself:", grid_loc: (0, 15) },
            TextSettings { text: "Smaller steps to acheive goal:", grid_loc: (0, 18) },
            TextSettings { text: "Smallest step to begin with:", grid_loc: (0, 22) },
            TextSettings { text: "Datetime to take that first step:", grid_loc: (0, 24) },
        ];

        let mut new_doc = self.default_page(offset, color);

        for TextSettings { text, grid_loc} in questions {
            new_doc = new_doc.add(
                Text::new(text)
                .set("x", mm(offset.0 + CELL_SIZE.0 * grid_loc.0 as f32))
                .set("y", mm(offset.1 + CELL_SIZE.1 * grid_loc.1 as f32 - 1.5))
                .set("style", format!("fill: {}; font-size: 14px;", color))
            );
        }

        return new_doc;
    }


    fn full_goals_page(self, color: &str) -> Self {
        self.goals_page(OUTTER_MARGINS, color)
            .goals_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0, OUTTER_MARGINS.1), color)
            .goals_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .goals_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0 + MID_SPLIT_WIDTH/2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), color)
            .cut_lines(color)
    }
}


fn main() {

    let empty_calendar = CalendarSettings {
        rows: vec!["", "", "", "", "", "", "", "", "", ""], 
        columns: vec!["Mon", "Tue", "Wed", "Thu", "Fri"]
    };
    let abcde_calendar = CalendarSettings {
        rows: vec!["Art", "Brush Teeth", "Chores", "Dread", "Entropy", "Flee", "Grand", "Heart", "Immediate", "", "", "", ""], 
        columns: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    };


    for color in ["red", "green", "blue", "black"] {
        svg::save(
                format!("output/grid_paper_{color}.svg"), 
                &Document::new().full_default_page(color))
            .and(svg::save(
                format!("output/empty_calendar_{color}.svg"), 
                &Document::new().full_calendar_page(color, &empty_calendar)))
            .and(svg::save(
                format!("output/abcde_calendar_{color}.svg"), 
                &Document::new().full_calendar_page(color, &abcde_calendar)))
            .and(svg::save(
                format!("output/goals_{color}.svg"), 
                &Document::new().full_goals_page(color)))
            .expect(format!("{color} files saved correctly").as_str());
    }
}
