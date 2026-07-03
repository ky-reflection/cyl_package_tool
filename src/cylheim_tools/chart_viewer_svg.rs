use super::{
    CylheimChartPageViewer, CylheimChartPageViewerNoteType, CylheimChartPageViewerNotes,
    CylheimChartPageViewerScanlineEventType, CylheimChartViewer,
};
use crate::cylheim_tools::{cyl_chart::CylheimChartPagePositionFunction, utils::CylToolError};
use svg::node::element::{Circle, Group, Line, Polygon, Rectangle, Text};
use svg::Document;

const PAGE_WIDTH: f64 = 512.0;
const PAGE_HEIGHT: f64 = 384.0;
const PLAYFIELD_TOP: f64 = 50.0;
const PLAYFIELD_BOTTOM: f64 = 344.0;
const PLAYFIELD_HEIGHT: f64 = PLAYFIELD_BOTTOM - PLAYFIELD_TOP;
const HORIZONTAL_PADDING: f64 = PAGE_WIDTH / 12.0;
const PAGE_GAP: usize = 20;
const OUTER_GAP: usize = 24;
const OUTER_STROKE: &str = "rgb(23,24,34)";

struct PageTransform {
    y1: f64,
    y2: f64,
    horizontal_padding: f64,
}

impl PageTransform {
    fn x(&self, x: f64) -> f64 {
        x * (PAGE_WIDTH - self.horizontal_padding * 2.0) + self.horizontal_padding
    }

    fn y(&self, y: f64) -> f64 {
        self.y1 + (self.y2 - self.y1) * y
    }

    fn transform_from_position_function(
        position_function: Option<&CylheimChartPagePositionFunction>,
    ) -> Self {
        let args = position_function
            .map(|pf| pf.position_function_arguments())
            .unwrap_or(&[1.0, 0.0]);
        let h = args[0] * PLAYFIELD_HEIGHT;
        let p = (1.0 - args[1]) / 2.0;
        let y1 = PLAYFIELD_TOP + PLAYFIELD_HEIGHT * p - h / 2.0;
        let y2 = PLAYFIELD_TOP + PLAYFIELD_HEIGHT * p + h / 2.0;
        Self {
            y1,
            y2,
            horizontal_padding: HORIZONTAL_PADDING,
        }
    }
}

struct NotePalette {
    click_outer: &'static str,
    click_middle: &'static str,
    click_middle_stroke: &'static str,
    click_inner: &'static str,
    flick: &'static str,
    drag_outer: &'static str,
    drag_inner: &'static str,
    drag_arrow: &'static str,
    click_drag_child_outer: &'static str,
    click_drag_child_inner: &'static str,
    click_drag_child_inner_stroke: &'static str,
    hold_outer: &'static str,
    hold_inner: &'static str,
}

fn palette_for_direction(direction: i32) -> NotePalette {
    if direction == 1 {
        NotePalette {
            click_outer: "rgb(175,198,206)",
            click_middle: "rgb(153,255,229)",
            click_middle_stroke: "rgb(29,133,122)",
            click_inner: "rgb(204,255,242)",
            flick: "rgb(39,191,141)",
            drag_outer: "rgb(182,180,203)",
            drag_inner: "rgb(170,102,255)",
            drag_arrow: "rgb(182,180,203)",
            click_drag_child_outer: "rgb(175,198,206)",
            click_drag_child_inner: "rgb(153,255,229)",
            click_drag_child_inner_stroke: "rgb(29,133,122)",
            hold_outer: "rgb(255,255,255)",
            hold_inner: "rgb(198,105,161)",
        }
    } else {
        NotePalette {
            click_outer: "rgb(175,190,206)",
            click_middle: "rgb(153,255,255)",
            click_middle_stroke: "rgb(29,118,143)",
            click_inner: "rgb(204,255,255)",
            flick: "rgb(41,170,220)",
            drag_outer: "rgb(172,180,203)",
            drag_inner: "rgb(246,102,255)",
            drag_arrow: "rgb(172,180,203)",
            click_drag_child_outer: "rgb(175,190,206)",
            click_drag_child_inner: "rgb(153,255,255)",
            click_drag_child_inner_stroke: "rgb(29,118,143)",
            hold_outer: "rgb(255,255,255)",
            hold_inner: "rgb(198,105,123)",
        }
    }
}

struct NoteLayerOptions {
    opacity: Option<f64>,
    show_labels: bool,
}

fn draw_hold_bodies(notes: &[CylheimChartPageViewerNotes], transform: &PageTransform) -> Group {
    let mut layer = Group::new();
    for note in notes.iter().rev() {
        let x = transform.x(note.position.x);
        let mut group = Group::new();

        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::LongHold => {
                if let Some(hold_body_handler) = &note.hold_body_handler {
                    let y = transform.y(hold_body_handler.start_y);
                    let end_y = transform.y(hold_body_handler.end_y);

                    group = group.add(
                        Line::new()
                            .set("x1", x - 15.0)
                            .set("y1", y)
                            .set("x2", x - 15.0)
                            .set("y2", end_y)
                            .set("stroke", "rgb(255,255,255)")
                            .set("stroke-width", 4),
                    );
                    group = group.add(
                        Line::new()
                            .set("x1", x + 15.0)
                            .set("y1", y)
                            .set("x2", x + 15.0)
                            .set("y2", end_y)
                            .set("stroke", "rgb(255,255,255)")
                            .set("stroke-width", 4),
                    );
                    group = group.add(
                        Line::new()
                            .set("x1", x)
                            .set("y1", y)
                            .set("x2", x)
                            .set("y2", end_y)
                            .set("stroke", "rgb(255,204,102)")
                            .set("stroke-width", 20)
                            .set("stroke-dasharray", "4,4"),
                    );
                    if hold_body_handler.with_tail {
                        group = group.add(
                            Line::new()
                                .set("x1", x - 20.0)
                                .set("y1", end_y)
                                .set("x2", x + 20.0)
                                .set("y2", end_y)
                                .set("stroke", "rgb(255,255,255)")
                                .set("stroke-width", 5),
                        );
                    }
                }
            }
            CylheimChartPageViewerNoteType::Hold => {
                if let Some(hold_body_handler) = &note.hold_body_handler {
                    let y = transform.y(hold_body_handler.start_y);
                    let end_y = transform.y(hold_body_handler.end_y);
                    group = group.add(
                        Line::new()
                            .set("x1", x)
                            .set("y1", y)
                            .set("x2", x)
                            .set("y2", end_y)
                            .set("stroke", "rgb(255,255,255)")
                            .set("stroke-width", 20)
                            .set("stroke-dasharray", "4,4"),
                    );
                }
            }
            _ => (),
        }
        layer = layer.add(group);
    }
    layer
}

fn draw_drag_links(notes: &[CylheimChartPageViewerNotes], transform: &PageTransform) -> Group {
    let mut layer = Group::new();
    for note in notes.iter().rev() {
        let mut group = Group::new();

        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::Drag
            | CylheimChartPageViewerNoteType::DragChild
            | CylheimChartPageViewerNoteType::ClickDrag
            | CylheimChartPageViewerNoteType::ClickDragChild => {
                let x = transform.x(note.position.x);
                let y = transform.y(note.position.y);
                if let Some(drag_child_pos) = &note.drag_child_pos {
                    let drag_child_x = transform.x(drag_child_pos.x);
                    let drag_child_y = transform.y(drag_child_pos.y);
                    group = group.add(
                        Line::new()
                            .set("x1", x)
                            .set("y1", y)
                            .set("x2", drag_child_x)
                            .set("y2", drag_child_y)
                            .set("stroke", "rgb(255, 255, 255)")
                            .set("stroke-width", 5)
                            .set("stroke-dasharray", "3,3")
                            .set("stroke-opacity", 0.7),
                    );
                }
            }
            _ => (),
        }
        layer = layer.add(group);
    }
    layer
}

fn draw_click_note(x: f64, y: f64, palette: &NotePalette) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 29)
            .set("fill", palette.click_outer)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 4),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 20.5)
            .set("fill", palette.click_middle)
            .set("stroke", palette.click_middle_stroke)
            .set("stroke-width", 5),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 9)
            .set("fill", palette.click_inner),
    );
    group
}

fn draw_flick_note(x: f64, y: f64, palette: &NotePalette) -> Group {
    let flick_color = palette.flick;
    let mut group = Group::new();
    group = group.add(
        Line::new()
            .set("x1", x - 35.0)
            .set("y1", y)
            .set("x2", x - 8.0)
            .set("y2", y + 28.0)
            .set("stroke", "rgb(255,255,255)")
            .set("stroke-width", 2),
    );
    group = group.add(
        Line::new()
            .set("x1", x - 35.0)
            .set("y1", y)
            .set("x2", x - 8.0)
            .set("y2", y - 28.0)
            .set("stroke", "rgb(255,255,255)")
            .set("stroke-width", 2),
    );
    group = group.add(
        Line::new()
            .set("x1", x + 35.0)
            .set("y1", y)
            .set("x2", x + 8.0)
            .set("y2", y + 28.0)
            .set("stroke", "rgb(255,255,255)")
            .set("stroke-width", 2),
    );
    group = group.add(
        Line::new()
            .set("x1", x + 35.0)
            .set("y1", y)
            .set("x2", x + 8.0)
            .set("y2", y - 28.0)
            .set("stroke", "rgb(255,255,255)")
            .set("stroke-width", 2),
    );
    group = group.add(
        Polygon::new()
            .set(
                "points",
                format!(
                    "{},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{}",
                    x - 5.0,
                    y - 24.0,
                    x - 26.0,
                    y - 3.0,
                    x - 26.0,
                    y + 3.0,
                    x - 5.0,
                    y + 24.0,
                    x + 5.0,
                    y + 24.0,
                    x + 26.0,
                    y + 3.0,
                    x + 26.0,
                    y - 3.0,
                    x + 5.0,
                    y - 24.0
                ),
            )
            .set("fill", flick_color),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 11.0)
            .set("fill", "rgb(255,255,255)"),
    );
    group = group.add(
        Polygon::new()
            .set(
                "points",
                format!(
                    "{},{} {},{} {},{} {},{} {},{} {},{}",
                    x - 8.0,
                    y - 7.5,
                    x,
                    y - 15.5,
                    x + 8.0,
                    y - 7.5,
                    x + 8.0,
                    y + 7.5,
                    x,
                    y + 15.5,
                    x - 8.0,
                    y + 7.5
                ),
            )
            .set("fill", "rgb(255,255,255)"),
    );
    group = group.add(
        Rectangle::new()
            .set("x", x - 2.0)
            .set("y", y - 18.0)
            .set("height", 37.0)
            .set("width", 4.0)
            .set("fill", flick_color),
    );
    group = group.add(
        Polygon::new()
            .set(
                "points",
                format!(
                    "{},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{}",
                    x - 2.0,
                    y - 28.0,
                    x - 27.0,
                    y - 3.0,
                    x - 27.0,
                    y + 3.0,
                    x - 2.0,
                    y + 28.0,
                    x - 2.0,
                    y + 23.0,
                    x - 22.0,
                    y + 3.0,
                    x - 22.0,
                    y - 3.0,
                    x - 2.0,
                    y - 23.0
                ),
            )
            .set("fill", "rgb(255,255,255)"),
    );
    group = group.add(
        Polygon::new()
            .set(
                "points",
                format!(
                    "{},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{}",
                    x + 2.0,
                    y - 28.0,
                    x + 27.0,
                    y - 3.0,
                    x + 27.0,
                    y + 3.0,
                    x + 2.0,
                    y + 28.0,
                    x + 2.0,
                    y + 23.0,
                    x + 22.0,
                    y + 3.0,
                    x + 22.0,
                    y - 3.0,
                    x + 2.0,
                    y - 23.0
                ),
            )
            .set("fill", "rgb(255,255,255)"),
    );
    group
}

fn draw_drag_note(
    note: &CylheimChartPageViewerNotes,
    x: f64,
    y: f64,
    transform: &PageTransform,
    palette: &NotePalette,
) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 23)
            .set("fill", palette.drag_outer)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 3),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 16)
            .set("fill", palette.drag_inner)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 4),
    );
    if let Some(drag_child_pos) = &note.drag_child_pos {
        let dx = transform.x(drag_child_pos.x);
        let dy = transform.y(drag_child_pos.y);

        let dx_val = dx - x;
        let dy_val = dy - y;
        let angle = if dx_val == 0.0 && dy_val == 0.0 {
            -45.0
        } else {
            dy_val.atan2(dx_val).to_degrees()
        };
        let arrow_point1_x = x - 8.0;
        let arrow_point1_y = y + 6.0;
        let arrow_point2_x = x + 8.0;
        let arrow_point2_y = y + 6.0;
        let arrow_point3_x = x;
        let arrow_point3_y = y - 8.0;

        let arrow_points = format!(
            "{},{} {},{} {},{} {},{}",
            arrow_point1_x,
            arrow_point1_y,
            x,
            y,
            arrow_point2_x,
            arrow_point2_y,
            arrow_point3_x,
            arrow_point3_y
        );

        group = group.add(
            Polygon::new()
                .set("points", arrow_points)
                .set("fill", palette.drag_arrow)
                .set("transform", format!("rotate({} {} {})", angle + 90.0, x, y)),
        );
    }
    group
}

fn draw_drag_child_note(x: f64, y: f64, palette: &NotePalette) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 12)
            .set("fill", palette.drag_outer)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 2),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 8)
            .set("fill", palette.drag_inner)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 2),
    );
    group
}

fn draw_click_drag_note(x: f64, y: f64, palette: &NotePalette) -> Group {
    draw_click_note(x, y, palette)
}

fn draw_click_drag_child_note(x: f64, y: f64, palette: &NotePalette) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 12)
            .set("fill", palette.click_drag_child_outer)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 2),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 8)
            .set("fill", palette.click_drag_child_inner)
            .set("stroke", palette.click_drag_child_inner_stroke)
            .set("stroke-width", 2),
    );
    group
}

fn draw_hold_head(x: f64, y: f64, palette: &NotePalette) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 30)
            .set("fill", palette.hold_outer)
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 6),
    );
    group = group.add(
        Rectangle::new()
            .set("x", x - 33.0)
            .set("y", y - 2.0)
            .set("width", 66)
            .set("height", 4)
            .set("fill", palette.hold_outer),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 19)
            .set("fill", palette.hold_outer)
            .set("stroke", palette.hold_inner)
            .set("stroke-width", 6),
    );
    group
}

fn draw_long_hold_head(x: f64, y: f64) -> Group {
    let mut group = Group::new();
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 30)
            .set("fill", "rgb(255,255,255)")
            .set("stroke", OUTER_STROKE)
            .set("stroke-width", 6),
    );
    group = group.add(
        Rectangle::new()
            .set("x", x - 34.0)
            .set("y", y - 2.0)
            .set("width", 67)
            .set("height", 4)
            .set("fill", "rgb(255,255,255)"),
    );
    group = group.add(
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 22)
            .set("fill", "rgb(255,204,102)"),
    );
    group = group.add(
        Rectangle::new()
            .set("x", x - 7.0)
            .set("y", y - 23.0)
            .set("width", 14)
            .set("height", 46)
            .set("fill", "rgb(255,255,255)"),
    );
    group
}

fn maybe_add_fingering_label(
    mut group: Group,
    note: &CylheimChartPageViewerNotes,
    x: f64,
    y: f64,
    show_label: bool,
) -> Group {
    if show_label {
        if let Some(fingering) = note.label.get("fingering") {
            group = group.add(
                Text::new(fingering.clone())
                    .set("x", x)
                    .set("y", y)
                    .set("font-family", "Consolas")
                    .set("font-size", 20)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "middle")
                    .set("font-weight", "bold")
                    .set("fill", "red"),
            );
        }
    }
    group
}

fn draw_note_head(
    note: &CylheimChartPageViewerNotes,
    transform: &PageTransform,
    palette: &NotePalette,
    show_label: bool,
) -> Group {
    let x = transform.x(note.position.x);
    let y = transform.y(note.position.y);

    let group = match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
        CylheimChartPageViewerNoteType::Click => draw_click_note(x, y, palette),
        CylheimChartPageViewerNoteType::Flick => draw_flick_note(x, y, palette),
        CylheimChartPageViewerNoteType::Drag => draw_drag_note(note, x, y, transform, palette),
        CylheimChartPageViewerNoteType::DragChild => draw_drag_child_note(x, y, palette),
        CylheimChartPageViewerNoteType::ClickDrag => draw_click_drag_note(x, y, palette),
        CylheimChartPageViewerNoteType::ClickDragChild => {
            draw_click_drag_child_note(x, y, palette)
        }
        CylheimChartPageViewerNoteType::Hold => {
            if note.hold_body_handler.is_some() {
                return Group::new();
            }
            draw_hold_head(x, y, palette)
        }
        CylheimChartPageViewerNoteType::LongHold => {
            if note.hold_body_handler.is_some() {
                return Group::new();
            }
            draw_long_hold_head(x, y)
        }
        _ => Group::new(),
    };

    maybe_add_fingering_label(group, note, x, y, show_label)
}

fn draw_note_heads(
    notes: &[CylheimChartPageViewerNotes],
    direction: i32,
    transform: &PageTransform,
    show_labels: bool,
) -> Group {
    let palette = palette_for_direction(direction);
    let mut layer = Group::new();
    for note in notes.iter().rev() {
        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::Hold | CylheimChartPageViewerNoteType::LongHold
                if note.hold_body_handler.is_some() =>
            {
                continue;
            }
            _ => (),
        }
        layer = layer.add(draw_note_head(note, transform, &palette, show_labels));
    }
    layer
}

fn draw_note_layers(
    notes: &[CylheimChartPageViewerNotes],
    direction: i32,
    transform: &PageTransform,
    options: NoteLayerOptions,
) -> Group {
    let mut layer = Group::new();
    if let Some(opacity) = options.opacity {
        layer = layer.set("opacity", opacity);
    }
    layer = layer.add(draw_hold_bodies(notes, transform));
    layer = layer.add(draw_drag_links(notes, transform));
    layer = layer.add(draw_note_heads(
        notes,
        direction,
        transform,
        options.show_labels,
    ));
    layer
}

fn draw_scanline_boundary_lines(document: Document, y1: f64, y2: f64) -> Document {
    let document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", PLAYFIELD_TOP)
            .set("x2", PAGE_WIDTH)
            .set("y2", PLAYFIELD_TOP)
            .set("stroke", "white")
            .set("stroke-dasharray", "1,20.2917")
            .set("stroke-width", 2),
    );
    let document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", PLAYFIELD_BOTTOM)
            .set("x2", PAGE_WIDTH)
            .set("y2", PLAYFIELD_BOTTOM)
            .set("stroke", "white")
            .set("stroke-dasharray", "1,20.2917")
            .set("stroke-width", 2),
    );
    let document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", y1)
            .set("x2", PAGE_WIDTH)
            .set("y2", y1)
            .set("stroke", "white")
            .set("stroke-opacity", 0.7)
            .set("stroke-width", 2),
    );
    document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", y2)
            .set("x2", PAGE_WIDTH)
            .set("y2", y2)
            .set("stroke", "white")
            .set("stroke-opacity", 0.7)
            .set("stroke-width", 2),
    )
}

fn draw_scanline_text_and_events(
    mut document: Document,
    page_viewer: &CylheimChartPageViewer,
    transform: &PageTransform,
) -> Document {
    if page_viewer.scanline.is_stable {
        document = document.add(
            Text::new("SCANLINE:".to_string())
                .set("x", 409.6)
                .set("y", 20)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
        document = document.add(
            Text::new(format!("{:.2}", page_viewer.scanline.max_scanline))
                .set("x", 501.76)
                .set("y", 20)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
    } else {
        document = document.add(
            Text::new("MAX SCANLINE:".to_string())
                .set("x", 409.6)
                .set("y", 20)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
        document = document.add(
            Text::new(format!("{:.2}", page_viewer.scanline.max_scanline))
                .set("x", 501.76)
                .set("y", 20)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
        document = document.add(
            Text::new("MIN SCANLINE:".to_string())
                .set("x", 409.6)
                .set("y", 40)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
        document = document.add(
            Text::new(format!("{:.2}", page_viewer.scanline.min_scanline))
                .set("x", 501.76)
                .set("y", 40)
                .set("font-family", "Electrolize")
                .set("font-size", 16)
                .set("font-weight", "bold")
                .set("text-anchor", "end")
                .set("fill", "white"),
        );
    }

    for event in page_viewer.scanline.scanline_event.iter() {
        let line_color = match event.event_type {
            CylheimChartPageViewerScanlineEventType::Accelerate => "red",
            CylheimChartPageViewerScanlineEventType::Decelerate => "green",
            CylheimChartPageViewerScanlineEventType::Stable => "white",
        };
        let event_y = transform.y(event.y);
        document = document.add(
            Line::new()
                .set("x1", 0)
                .set("y1", event_y)
                .set("x2", PAGE_WIDTH)
                .set("y2", event_y)
                .set("stroke", line_color)
                .set("stroke-opacity", 0.7)
                .set("stroke-width", 2),
        );
        document = document.add(
            Text::new(format!("{:.2}", event.scanline))
                .set("x", 502)
                .set("y", event_y - 12.0 * page_viewer.scan_line_direction as f64)
                .set("font-family", "Electrolize")
                .set("font-size", 12)
                .set("text-anchor", "end")
                .set("fill", line_color),
        );
    }

    document
}

fn scanline_direction_arrow(scan_line_direction: i32) -> Polygon {
    let arrow_points = if scan_line_direction == 1 {
        "0,87 0,307 16,307 16,142 32,142"
    } else {
        "0,307 0,87 16,87 16,252 32,252"
    };

    Polygon::new()
        .set("points", arrow_points)
        .set("fill", "white")
}

pub(crate) fn draw_svg_page(
    page_viewer: CylheimChartPageViewer,
    show_ghost_note: bool,
) -> Result<Document, CylToolError> {
    let transform = PageTransform::transform_from_position_function(page_viewer.position_function.as_ref());

    let mut document = Document::new()
        .set("viewBox", (0, 0, PAGE_WIDTH as u32, PAGE_HEIGHT as u32))
        .set("width", PAGE_WIDTH.to_string())
        .set("height", PAGE_HEIGHT.to_string());

    document = document.add(
        Rectangle::new()
            .set("width", PAGE_WIDTH.to_string())
            .set("height", PAGE_HEIGHT.to_string())
            .set("fill", "rgb(0,0,0)"),
    );

    document = document.add(
        Text::new(format!("{}", page_viewer.page_index))
            .set("x", 10)
            .set("y", 24)
            .set("font-family", "Electrolize")
            .set("font-size", 24)
            .set("font-weight", "bold")
            .set("fill", "white"),
    );

    if show_ghost_note {
        let ghost_transform = PageTransform::transform_from_position_function(
            page_viewer.ghost_position_function.as_ref(),
        );
        document = document.add(draw_note_layers(
            &page_viewer.ghost_note_list,
            page_viewer.ghost_scan_line_direction,
            &ghost_transform,
            NoteLayerOptions {
                opacity: Some(0.3),
                show_labels: false,
            },
        ));
    }

    document = draw_scanline_boundary_lines(document, transform.y1, transform.y2);
    document = draw_scanline_text_and_events(document, &page_viewer, &transform);

    document = document.add(draw_note_layers(
        &page_viewer.note_list,
        page_viewer.scan_line_direction,
        &transform,
        NoteLayerOptions {
            opacity: None,
            show_labels: true,
        },
    ));

    document = document.add(scanline_direction_arrow(page_viewer.scan_line_direction));

    Ok(document)
}

pub(crate) fn draw_all_svg_pages(
    chart_viewer: &CylheimChartViewer,
    columns: usize,
    show_ghost_note: bool,
) -> Result<Document, CylToolError> {
    if columns == 0 {
        return Err(CylToolError::from("columns must be greater than 0"));
    }

    let page_width = PAGE_WIDTH as usize;
    let page_height = PAGE_HEIGHT as usize;
    let rows = chart_viewer.pages.len().div_ceil(columns);

    let total_width = (page_width + PAGE_GAP) * columns - PAGE_GAP + 2 * OUTER_GAP;
    let total_height = (page_height + PAGE_GAP) * rows - PAGE_GAP + 2 * OUTER_GAP;

    let mut document = Document::new()
        .set("viewBox", (0, 0, total_width, total_height))
        .set("width", total_width)
        .set("height", total_height);

    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "rgb(54,54,54)"),
    );

    for (index, page_viewer) in chart_viewer.pages.iter().enumerate() {
        let col = index % columns;
        let row = index / columns;

        let page_svg = draw_svg_page(page_viewer.clone(), show_ghost_note)?;

        let mut translated_group = Group::new().set(
            "transform",
            format!(
                "translate({}, {})",
                col * (page_width + PAGE_GAP) + OUTER_GAP,
                row * (page_height + PAGE_GAP) + OUTER_GAP
            ),
        );

        for child in page_svg.get_children() {
            translated_group = translated_group.add(child.clone());
        }
        document = document.add(translated_group);
    }

    Ok(document)
}
