use svg::Document;
use svg::node::element::*;
use svg::node::element::path::Data;


const PAPER_SIZE: (f32, f32) = (215.9, 279.4);
const OUTTER_MARGINS: (f32, f32) = (10.0,10.0);
const INNER_MARGINS: (f32, f32) = (5.0,5.0);
const CELL_SIZE: (f32, f32) = (5.0,5.0);

fn mm(x: f32) -> String {
    format!("{}mm", x)
}

fn dot(color: &str) -> Circle {
    return Circle::new()
        .set("stroke-width", 1)
        .set("fill", color)
        .set("r", "0.25mm");
}

fn paint_header(offset: (f32, f32), num_cells: (usize, usize), document: Document, color: &str) -> Document {
    let right_box_width = 5.0;
    let mid_box_width = 2.0;
    let left_box_width = (num_cells.0 as f32) - right_box_width - mid_box_width - 1.0;

    return document.add(
        Rectangle::new() // Left Box
            .set("width", mm(CELL_SIZE.0 * left_box_width))
            .set("height", mm(CELL_SIZE.1))
            .set("x", mm(offset.0))
            .set("y", mm(offset.1))
            .set("fill", "none")
            .set("stroke-width", "0.25mm")
            .set("stroke", color)
    ).add(
        Rectangle::new() // Mid Box
            .set("width", mm(CELL_SIZE.0 * mid_box_width))
            .set("height", mm(CELL_SIZE.1))
            .set("x", mm(offset.0 + CELL_SIZE.0 * left_box_width))
            .set("y", mm(offset.1))
            .set("fill", "none")
            .set("stroke-width", "0.25mm")
            .set("stroke", color)
    ).add(
        Rectangle::new() // Right Box
            .set("width", mm(CELL_SIZE.0 * right_box_width))
            .set("height", mm(CELL_SIZE.1))
            .set("x", mm(offset.0 + CELL_SIZE.0 * (left_box_width + mid_box_width)))
            .set("y", mm(offset.1))
            .set("fill", "none")
            .set("stroke-width", "0.25mm")
            .set("stroke", color)
    );
}

fn paint_grid(offset: (f32, f32), num_cells: (usize, usize), doc: Document, color: &str) -> Document {
    let mut new_doc = doc;

    for x in 0..num_cells.0 {
        for y in 0..num_cells.1 {
            let loc = (
                (x as f32) * CELL_SIZE.0 + offset.0, 
                (y as f32) * CELL_SIZE.1 + offset.1
            );
            new_doc = new_doc.add(
                dot(color)
                    .set("cx", mm(loc.0))
                    .set("cy", mm(loc.1))
            );
        }
    }

    return new_doc;
}

fn paint_default_page(offset: (f32, f32), document: Document, color: &str) -> Document {
    let mut doc = document;

    let num_cells = (
        ((PAPER_SIZE.0/2.0 - OUTTER_MARGINS.0 - INNER_MARGINS.0) / CELL_SIZE.0).ceil() as usize,
        ((PAPER_SIZE.1/2.0 - OUTTER_MARGINS.1 - INNER_MARGINS.1) / CELL_SIZE.1).ceil() as usize
    );
    doc = paint_grid(offset, num_cells, doc, color);
    doc = paint_header(offset, num_cells, doc, color);
    return doc;
}

fn paint_calendar(offset: (f32, f32), num_cells: (usize, usize), document: Document, color: &str) -> Document {
    let mut doc = document;

    let box_width = 2.0;
    let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let cell_offset = (CELL_SIZE.0 * (num_cells.0 - (2*days.len()) - 1) as f32 + offset.0, CELL_SIZE.1 * 3.0 + offset.1);
    let rows = vec!["Art", "Brush Teeth", "Chores", "Dread", "Entropy", "Flee", "Grand", "Immediate"];

    // doc = doc.add(
    //     Script::new(format!(".text {{ font: {}; }}", color))
    // );

    
    for (c, day) in days.iter().enumerate() {
        doc = doc.add(
            Text::new(day.to_string())
                .set("x", mm(cell_offset.0 + CELL_SIZE.0 * box_width * (c as f32)))
                .set("y", mm(cell_offset.1 - 1.0))
                .set("style", format!("fill: {}; font-weight: lighter; font-size: 14px;", color))
        );
        
    }

    for (r, row) in rows.iter().enumerate() {
        doc = doc.add(
            Text::new(row.to_string())
                .set("x", mm(offset.0))
                .set("y", mm(cell_offset.1 + CELL_SIZE.1 * ((r + 1) as f32) - 1.0))
                .set("style", format!("fill: {}; font-weight: lighter; font-size: 14px;", color))
        );
        for (c, day) in days.iter().enumerate() {
            doc = doc.add(
                Rectangle::new()
                    .set("width", mm(box_width * CELL_SIZE.0))
                    .set("height", mm(CELL_SIZE.1))
                    .set("x", mm(cell_offset.0 + CELL_SIZE.0 * box_width * (c as f32)))
                    .set("y", mm(cell_offset.1 + CELL_SIZE.1 * (r as f32)))
                    .set("fill", "none")
                    .set("stroke-width", "0.25mm")
                    .set("stroke", color)
            );
        }
    }

    return doc;
}

fn paint_calendar_page(offset: (f32, f32), document: Document, color: &str) -> Document {
    let mut doc = document;

    let num_cells = (
        ((PAPER_SIZE.0/2.0 - OUTTER_MARGINS.0 - INNER_MARGINS.0) / CELL_SIZE.0).ceil() as usize,
        ((PAPER_SIZE.1/2.0 - OUTTER_MARGINS.1 - INNER_MARGINS.1) / CELL_SIZE.1).ceil() as usize
    );

    doc = paint_grid(offset, num_cells, doc, color);
    doc = paint_header(offset, num_cells, doc, color);
    doc = paint_calendar(offset, num_cells, doc, color);



    return doc;
}

fn save_page(color: &str, file_name: &str) {

    let mut document = Document::new()
        .set("viewBox", (0, 0, mm(PAPER_SIZE.0), mm(PAPER_SIZE.1)));

    document = paint_calendar_page(OUTTER_MARGINS, document, color);
    document = paint_calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0+2.0,OUTTER_MARGINS.1), document, color);
    document = paint_calendar_page((OUTTER_MARGINS.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), document, color);
    document = paint_calendar_page((PAPER_SIZE.0/2.0 + INNER_MARGINS.0+2.0,PAPER_SIZE.1/2.0 + OUTTER_MARGINS.1), document, color);

    document = document.add(
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
    );

    // document = document.add(dot());

    svg::save(file_name, &document).unwrap();

}

fn main() {
    save_page("red", "grid_paper_red.svg");
    save_page("green", "grid_paper_green.svg");
    save_page("blue", "grid_paper_blue.svg");
}
