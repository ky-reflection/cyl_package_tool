use std::{cmp::min, collections::HashMap};

use super::{
    cyl_chart::{CylheimChart, CylheimChartPagePositionFunction, CylheimChartTempo},
    utils::CylToolError,
};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use svg::{
    node::element::{Circle, Group, Line, Polygon, Rectangle, Text},
    Document,
};
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub struct CylheimChartViewer {
    #[getset(get = "pub", set = "pub")]
    chart: CylheimChart,
    #[getset(get = "pub", set = "pub")]
    pages: Vec<CylheimChartPageViewer>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub struct CylheimChartPageViewer {
    #[getset(get = "pub", set = "pub")]
    time_base: u32,
    #[getset(get = "pub", set = "pub")]
    page_index: u32,
    #[getset(get = "pub", set = "pub")]
    start_tick: u32,
    #[getset(get = "pub", set = "pub")]
    end_tick: u32,
    #[getset(get = "pub", set = "pub")]
    scan_line_direction: i32,
    #[getset(get = "pub", set = "pub")]
    ghost_scan_line_direction: i32,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    tempo_list: Vec<CylheimChartTempo>,
    #[getset(get = "pub", set = "pub")]
    note_list: Vec<CylheimChartPageViewerNotes>,
    #[getset(get = "pub", set = "pub")]
    ghost_note_list: Vec<CylheimChartPageViewerNotes>,
    #[getset(get = "pub", set = "pub")]
    #[serde(rename = "PositionFunction", skip_serializing_if = "Option::is_none")]
    position_function: Option<CylheimChartPagePositionFunction>,
    #[getset(get = "pub", set = "pub")]
    #[serde(
        rename = "GhostPositionFunction",
        skip_serializing_if = "Option::is_none"
    )]
    ghost_position_function: Option<CylheimChartPagePositionFunction>,
    #[getset(get = "pub", set = "pub")]
    scanline: CylheimChartPageViewerScanlineHandler,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerNotes {
    #[getset(get = "pub", set = "pub")]
    page_index: u32,
    #[serde(rename = "type")]
    #[getset(get = "pub", set = "pub")]
    note_type: u32,
    #[getset(get = "pub", set = "pub")]
    id: u32,
    #[getset(get = "pub", set = "pub")]
    tick: u32,
    #[getset(get = "pub", set = "pub")]
    position: CylheimChartPageViewerNotePostion,
    #[getset(get = "pub", set = "pub")]
    has_sibling: bool,
    #[getset(get = "pub", set = "pub")]
    /// None means this is not a long hold.
    hold_body_handler: Option<CylheimChartPageViewerHoldBodyHandler>,
    #[getset(get = "pub", set = "pub")]
    /// None means there is not a parent drag.
    drag_parent_pos: Option<CylheimChartPageViewerNotePostion>,
    #[getset(get = "pub", set = "pub")]
    /// None means there is not a child drag.
    drag_child_pos: Option<CylheimChartPageViewerNotePostion>,
    #[getset(get = "pub", set = "pub")]
    hold_tick: u32,
    #[getset(get = "pub", set = "pub")]
    next_id: i32,
    #[getset(get = "pub", set = "pub")]
    is_forward: bool,
    #[getset(get = "pub", set = "pub")]
    #[serde(rename = "NoteDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    falling_note_direction: Option<u32>,
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    approach_rate: Option<f64>,
    #[getset(get = "pub", set = "pub")]
    label: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerHoldBodyHandler {
    #[getset(get = "pub", set = "pub")]
    is_long_hold: bool,
    #[getset(get = "pub", set = "pub")]
    with_tail: bool,
    #[getset(get = "pub", set = "pub")]
    start_y: f64,
    #[getset(get = "pub", set = "pub")]
    end_y: f64,
}
impl CylheimChartPageViewerHoldBodyHandler {
    #[allow(dead_code)]
    fn check_start_end(&self) -> bool {
        self.start_y <= self.end_y
    }
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters, Default)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerNotePostion {
    #[getset(get = "pub", set = "pub")]
    x: f64,
    #[getset(get = "pub", set = "pub")]
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
pub enum CylheimChartPageViewerNoteType {
    Click,
    Hold,
    LongHold,
    Drag,
    DragChild,
    Flick,
    ClickDrag,
    ClickDragChild,
    DropClick,
    DropDrag,
    CustomNoteType(u32),
}
impl Into<u32> for CylheimChartPageViewerNoteType {
    fn into(self) -> u32 {
        match self {
            CylheimChartPageViewerNoteType::Click => 0,
            CylheimChartPageViewerNoteType::Hold => 1,
            CylheimChartPageViewerNoteType::LongHold => 2,
            CylheimChartPageViewerNoteType::Drag => 3,
            CylheimChartPageViewerNoteType::DragChild => 4,
            CylheimChartPageViewerNoteType::Flick => 5,
            CylheimChartPageViewerNoteType::ClickDrag => 6,
            CylheimChartPageViewerNoteType::ClickDragChild => 7,
            CylheimChartPageViewerNoteType::DropClick => 8,
            CylheimChartPageViewerNoteType::DropDrag => 9,
            CylheimChartPageViewerNoteType::CustomNoteType(r#type) => r#type,
        }
    }
}
impl CylheimChartPageViewerNoteType {
    #[allow(unused)]

    pub fn get_id(&self) -> u32 {
        match self {
            CylheimChartPageViewerNoteType::Click => 0,
            CylheimChartPageViewerNoteType::Hold => 1,
            CylheimChartPageViewerNoteType::LongHold => 2,
            CylheimChartPageViewerNoteType::Drag => 3,
            CylheimChartPageViewerNoteType::DragChild => 4,
            CylheimChartPageViewerNoteType::Flick => 5,
            CylheimChartPageViewerNoteType::ClickDrag => 6,
            CylheimChartPageViewerNoteType::ClickDragChild => 7,
            CylheimChartPageViewerNoteType::DropClick => 8,
            CylheimChartPageViewerNoteType::DropDrag => 9,
            CylheimChartPageViewerNoteType::CustomNoteType(r#type) => *r#type,
        }
    }
    #[allow(unused)]
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(CylheimChartPageViewerNoteType::Click),
            1 => Some(CylheimChartPageViewerNoteType::Hold),
            2 => Some(CylheimChartPageViewerNoteType::LongHold),
            3 => Some(CylheimChartPageViewerNoteType::Drag),
            4 => Some(CylheimChartPageViewerNoteType::DragChild),
            5 => Some(CylheimChartPageViewerNoteType::Flick),
            6 => Some(CylheimChartPageViewerNoteType::ClickDrag),
            7 => Some(CylheimChartPageViewerNoteType::ClickDragChild),
            8 => Some(CylheimChartPageViewerNoteType::DropClick),
            9 => Some(CylheimChartPageViewerNoteType::DropDrag),
            id => Some(CylheimChartPageViewerNoteType::CustomNoteType(id)),
            _ => None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerScanlineHandler {
    is_stable: bool,
    max_scanline: f64,
    min_scanline: f64,
    scanline_event: Vec<CylheimChartPageViewerScanlineEvent>,
}
impl Default for CylheimChartPageViewerScanlineHandler {
    fn default() -> Self {
        Self {
            is_stable: Default::default(),
            max_scanline: -f64::INFINITY,
            min_scanline: f64::INFINITY,
            scanline_event: Default::default(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters, Default)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerScanlineEvent {
    scanline: f64,
    y: f64,
    event_type: CylheimChartPageViewerScanlineEventType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
pub enum CylheimChartPageViewerScanlineEventType {
    Accelerate,
    Decelerate,
    Stable,
}
impl Default for CylheimChartPageViewerScanlineEventType {
    fn default() -> Self {
        CylheimChartPageViewerScanlineEventType::Stable
    }
}
#[allow(dead_code)]
pub(crate) fn build_cylheim_chart_viewer(
    chart: CylheimChart,
    labels: Option<HashMap<u32, HashMap<String, String>>>,
) -> Result<CylheimChartViewer, CylToolError> {
    let mut chart = chart.clone();
    chart.sort_internal_data();
    chart.sort_and_reassign_note_ids();
    let mut chart_viewer = CylheimChartViewer {
        chart: chart.clone(),
        pages: Vec::new(),
    };
    let note_list = chart.note_list().clone();
    let mut note_viewer_list: Vec<CylheimChartPageViewerNotes> = Vec::new();
    for note in &note_list {
        let mut note_viewer = CylheimChartPageViewerNotes {
            page_index: *note.page_index() - if *note.is_forward() { 1 } else { 0 },
            note_type: *note.note_type(),
            id: *note.id(),
            tick: *note.tick(),
            position: CylheimChartPageViewerNotePostion::default(),
            has_sibling: *note.has_sibling(),
            hold_body_handler: None,
            drag_parent_pos: None,
            drag_child_pos: None,
            hold_tick: *note.hold_tick(),
            next_id: *note.next_id(),
            is_forward: *note.is_forward(),
            falling_note_direction: None,
            approach_rate: None,
            label: HashMap::new(),
        };

        let note_page = &chart.page_list()[note_viewer.page_index as usize];
        let note_y = note_y_from_tick(
            *note.tick(),
            *note_page.start_tick(),
            *note_page.end_tick(),
            *note_page.scan_line_direction(),
        );
        note_viewer.position.set_x(*note.x());
        note_viewer.position.set_y(note_y);
        note_viewer_list.push(note_viewer);
    }
    note_viewer_list.sort_by_key(|note_viewer| *note_viewer.id());
    if let Some(labels) = labels {
        for note_viewer in &mut note_viewer_list {
            if let Some(label) = labels.get(note_viewer.id()) {
                note_viewer.label = label.clone();
            }
        }
    }
    for note in &note_list {
        if *note.next_id() >= 0
            && (*note.note_type() == CylheimChartPageViewerNoteType::Drag.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::DragChild.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::ClickDrag.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::ClickDragChild.get_id())
        {
            let next_note_index = *note.next_id() as usize;
            let current_note_index = *note.id() as usize;
            let next_note_pos = note_viewer_list[next_note_index].position().clone();
            let current_note_pos = note_viewer_list[current_note_index].position().clone();

            note_viewer_list[next_note_index].set_drag_parent_pos(Some(current_note_pos));
            note_viewer_list[current_note_index].set_drag_child_pos(Some(next_note_pos));
        }
    }

    for (page_index, page) in chart.clone().page_list().iter().enumerate() {
        let mut page_viewer = CylheimChartPageViewer {
            time_base: *chart.time_base(),
            page_index: page_index as u32,
            start_tick: *page.start_tick(),
            end_tick: *page.end_tick(),
            scan_line_direction: *page.scan_line_direction(),
            ghost_scan_line_direction: 0.into(),
            tempo_list: Vec::new(),
            note_list: Vec::new(),
            ghost_note_list: Vec::new(),
            position_function: page.position_function().clone(),
            ghost_position_function: None,
            scanline: CylheimChartPageViewerScanlineHandler::default(),
        };
        page_viewer.tempo_list = chart.tempo_list()[chart
            .tempo_list()
            .partition_point(|tempo| *tempo.tick() < page_viewer.start_tick)
            ..chart
                .tempo_list()
                .partition_point(|tempo| *tempo.tick() < page_viewer.end_tick)]
            .to_vec();
        page_viewer.note_list = note_viewer_list
            .iter()
            .filter(|note_viewer| note_viewer.page_index == page_index as u32)
            .cloned()
            .collect();

        chart_viewer.pages.push(page_viewer);
    }
    for note in chart.note_list() {
        match CylheimChartPageViewerNoteType::from_id(*note.note_type()) {
            Some(CylheimChartPageViewerNoteType::LongHold) => {
                let page_index = *note.page_index() as usize;
                let (left_pages, right_pages) = chart_viewer.pages.split_at_mut(page_index + 1);
                let note_page = &mut left_pages[page_index];
                let mut note_hold_handler = CylheimChartPageViewerNotes {
                    page_index: *note_page.page_index(),
                    note_type: *note.note_type(),
                    id: *note.id(),
                    tick: *note.tick(),
                    position: CylheimChartPageViewerNotePostion {
                        x: *note.x(),
                        y: 0.0,
                    },
                    has_sibling: *note.has_sibling(),
                    hold_body_handler: None,
                    drag_parent_pos: None,
                    drag_child_pos: None,
                    hold_tick: *note.hold_tick(),
                    next_id: *note.next_id(),
                    is_forward: *note.is_forward(),
                    falling_note_direction: None,
                    approach_rate: None,
                    label: HashMap::new(),
                };
                note_hold_handler.hold_body_handler = Some(CylheimChartPageViewerHoldBodyHandler {
                    is_long_hold: true,
                    with_tail: (note.tick() + note.hold_tick()) <= note_page.end_tick,
                    start_y: note_y_from_tick(
                        *note.tick(),
                        note_page.start_tick,
                        note_page.end_tick,
                        note_page.scan_line_direction,
                    ),
                    end_y: note_y_from_tick(
                        min(note.tick() + note.hold_tick(), note_page.end_tick),
                        note_page.start_tick,
                        note_page.end_tick,
                        note_page.scan_line_direction,
                    ),
                });
                note_page.note_list.push(note_hold_handler);
                for page in right_pages {
                    if note.tick() + note.hold_tick() <= *page.start_tick() {
                        continue;
                    } else {
                        let mut note_hold_handler = CylheimChartPageViewerNotes {
                            page_index: *page.page_index(),
                            note_type: *note.note_type(),
                            id: *note.id(),
                            tick: *note.tick(),
                            position: CylheimChartPageViewerNotePostion {
                                x: *note.x(),
                                y: 0.0,
                            },
                            has_sibling: *note.has_sibling(),
                            hold_body_handler: None,
                            drag_parent_pos: None,
                            drag_child_pos: None,
                            hold_tick: *note.hold_tick(),
                            next_id: *note.next_id(),
                            is_forward: *note.is_forward(),
                            falling_note_direction: None,
                            approach_rate: None,
                            label: HashMap::new(),
                        };
                        note_hold_handler.hold_body_handler =
                            Some(CylheimChartPageViewerHoldBodyHandler {
                                is_long_hold: true,
                                with_tail: (note.tick() + note.hold_tick()) <= page.end_tick,
                                start_y: note_y_from_tick(
                                    page.start_tick,
                                    page.start_tick,
                                    page.end_tick,
                                    page.scan_line_direction,
                                ),
                                end_y: note_y_from_tick(
                                    min(note.tick() + note.hold_tick(), page.end_tick),
                                    page.start_tick,
                                    page.end_tick,
                                    page.scan_line_direction,
                                ),
                            });
                        page.note_list.push(note_hold_handler);
                    }
                }
            }
            Some(CylheimChartPageViewerNoteType::Hold) => {
                let page_index = *note.page_index() as usize;
                let note_page = &mut chart_viewer.pages[page_index];
                let mut note_hold_handler = CylheimChartPageViewerNotes {
                    page_index: *note_page.page_index(),
                    note_type: *note.note_type(),
                    id: *note.id(),
                    tick: *note.tick(),
                    position: CylheimChartPageViewerNotePostion {
                        x: *note.x(),
                        y: 0.0,
                    },
                    has_sibling: *note.has_sibling(),
                    hold_body_handler: None,
                    drag_parent_pos: None,
                    drag_child_pos: None,
                    hold_tick: *note.hold_tick(),
                    next_id: *note.next_id(),
                    is_forward: *note.is_forward(),
                    falling_note_direction: None,
                    approach_rate: None,
                    label: HashMap::new(),
                };
                note_hold_handler.hold_body_handler = Some(CylheimChartPageViewerHoldBodyHandler {
                    is_long_hold: false,
                    with_tail: true,
                    start_y: note_y_from_tick(
                        *note.tick(),
                        note_page.start_tick,
                        note_page.end_tick,
                        note_page.scan_line_direction,
                    ),
                    end_y: note_y_from_tick(
                        min(note.tick() + note.hold_tick(), note_page.end_tick),
                        note_page.start_tick,
                        note_page.end_tick,
                        note_page.scan_line_direction,
                    ),
                });
                note_page.note_list.push(note_hold_handler);
            }
            Some(_) | None => (),
        }
    }
    for index in 0..chart_viewer.pages.len() {
        if index >= chart_viewer.pages.len() - 1 {
            break;
        } else {
            chart_viewer.pages[index].ghost_scan_line_direction =
                chart_viewer.pages[index + 1].scan_line_direction;
            chart_viewer.pages[index].ghost_note_list =
                chart_viewer.pages[index + 1].note_list.clone();
            chart_viewer.pages[index].ghost_position_function =
                chart_viewer.pages[index + 1].position_function.clone();
        }
    }
    let mut current_tempo = chart.tempo_list()[0].clone();
    let mut current_scanline = tempo_to_scanline(
        *current_tempo.value(),
        chart.page_list()[0].get_page_size() as u32,
        chart.page_list()[0]
            .position_function()
            .as_ref()
            .map_or(1.0, |x| x.position_function_arguments()[0]),
    );
    for page in chart_viewer.pages.iter_mut() {
        let page_size = page.end_tick - page.start_tick;
        let page_position_function_arg = page
            .position_function
            .as_ref()
            .map_or(1.0, |x| x.position_function_arguments()[0]);
        if page.tempo_list.is_empty() {
            page.scanline.is_stable = true;
            let page_scanline = tempo_to_scanline(
                *current_tempo.value(),
                page_size,
                page_position_function_arg,
            );
            page.scanline.max_scanline = page_scanline;
            page.scanline.min_scanline = page_scanline;
            if current_scanline != page_scanline {
                let event_type = if page_scanline > current_scanline {
                    CylheimChartPageViewerScanlineEventType::Accelerate
                } else if page_scanline < current_scanline {
                    CylheimChartPageViewerScanlineEventType::Decelerate
                } else {
                    CylheimChartPageViewerScanlineEventType::Stable
                };
                let event_y = note_y_from_tick(
                    page.start_tick,
                    page.start_tick,
                    page.end_tick,
                    page.scan_line_direction,
                );
                page.scanline
                    .scanline_event
                    .push(CylheimChartPageViewerScanlineEvent {
                        scanline: page_scanline,
                        y: event_y,
                        event_type: event_type,
                    });
                current_scanline = page_scanline;
            }
        } else {
            page.scanline.is_stable = true;
            let mut start_scanline = current_scanline;
            for tempo in &page.tempo_list {
                let page_scanline =
                    tempo_to_scanline(*tempo.value(), page_size, page_position_function_arg);
                if *tempo.tick() == page.start_tick {
                    start_scanline = page_scanline;
                }
                if page_scanline > page.scanline.max_scanline {
                    page.scanline.max_scanline = page_scanline;
                }
                if page_scanline < page.scanline.min_scanline {
                    page.scanline.min_scanline = page_scanline;
                }
                if page_scanline != start_scanline {
                    page.scanline.is_stable = false
                }
                let event_type = if page_scanline > current_scanline {
                    CylheimChartPageViewerScanlineEventType::Accelerate
                } else if page_scanline < current_scanline {
                    CylheimChartPageViewerScanlineEventType::Decelerate
                } else {
                    CylheimChartPageViewerScanlineEventType::Stable
                };
                let event_y = note_y_from_tick(
                    *tempo.tick(),
                    page.start_tick,
                    page.end_tick,
                    page.scan_line_direction,
                );
                page.scanline
                    .scanline_event
                    .push(CylheimChartPageViewerScanlineEvent {
                        scanline: page_scanline,
                        y: event_y,
                        event_type: event_type,
                    });
                current_tempo = tempo.clone();
                current_scanline = page_scanline;
            }
            if start_scanline > page.scanline.max_scanline {
                page.scanline.max_scanline = start_scanline;
            }
            if start_scanline < page.scanline.min_scanline {
                page.scanline.min_scanline = start_scanline;
            }
        }
    }
    Ok(chart_viewer)
}

pub(crate) fn draw_svg_page(
    page_viewer: CylheimChartPageViewer,
    show_ghost_note: bool,
) -> Result<Document, CylToolError> {
    let mut document = Document::new()
        .set("viewBox", (0, 0, 512, 384))
        .set("width", "512")
        .set("height", "384");

    document = document.add(
        Rectangle::new()
            .set("width", "512")
            .set("height", "384")
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

    let args = page_viewer
        .position_function
        .as_ref()
        .map(|pf| pf.position_function_arguments())
        .unwrap_or(&[1.0, 0.0]);
    let h = args[0] * 294.0; // 344 - 50 = 294
    let p = (1.0 - args[1]) / 2.0;
    let y1 = 50.0 + 294.0 * p - h / 2.0;
    let y2 = 50.0 + 294.0 * p + h / 2.0;
    let padding = 512.0 / 12.0;
    let calc_x = |x: f64| x * (512.0 - padding * 2.0) + padding;
    let calc_y = |y: f64| y1 + (y2 - y1) * y;
    if show_ghost_note {
        let args = page_viewer
            .ghost_position_function
            .as_ref()
            .map(|pf| pf.position_function_arguments())
            .unwrap_or(&[1.0, 0.0]);
        let h = args[0] * 294.0; // 344 - 50 = 294
        let p = (1.0 - args[1]) / 2.0;
        let y1 = 50.0 + 294.0 * p - h / 2.0;
        let y2 = 50.0 + 294.0 * p + h / 2.0;
        let padding = 512.0 / 12.0;
        let calc_x_g = |x: f64| x * (512.0 - padding * 2.0) + padding;
        let calc_y_g = |y: f64| y1 + (y2 - y1) * y;
        let mut ghost_group = Group::new().set("opacity", 0.3);

        //bottom to top: longhold body, hold body, drag link, other note(including longhold/hold head, from larger id to smaller id)

        //draw hold body
        for note in page_viewer.ghost_note_list.iter().rev() {
            let x = calc_x_g(note.position.x);
            let mut group = Group::new();

            match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
                CylheimChartPageViewerNoteType::LongHold => {
                    if let Some(hold_body_handler) = &note.hold_body_handler {
                        let y = calc_y_g(hold_body_handler.start_y);
                        let end_y = calc_y_g(hold_body_handler.end_y);

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
                        let y = calc_y_g(hold_body_handler.start_y);
                        let end_y = calc_y_g(hold_body_handler.end_y);
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
            ghost_group = ghost_group.add(group);
        }
        //draw link
        for note in page_viewer.ghost_note_list.iter().rev() {
            let mut group = Group::new();

            match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
                CylheimChartPageViewerNoteType::Drag
                | CylheimChartPageViewerNoteType::DragChild
                | CylheimChartPageViewerNoteType::ClickDrag
                | CylheimChartPageViewerNoteType::ClickDragChild => {
                    let x = calc_x_g(note.position.x);
                    let y = calc_y_g(note.position.y);
                    if let Some(drag_child_pos) = &note.drag_child_pos {
                        let drag_child_x = calc_x_g(drag_child_pos.x);
                        let drag_child_y = calc_y_g(drag_child_pos.y);
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
            ghost_group = ghost_group.add(group);
        }
        //draw notes
        for note in page_viewer.ghost_note_list.iter().rev() {
            let x = calc_x_g(note.position.x);
            let y = calc_y_g(note.position.y);
            let mut group = Group::new();
            match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
                CylheimChartPageViewerNoteType::Click => {
                    let (outer_color, middle_color, middle_stroke, inner_color) =
                        if page_viewer.ghost_scan_line_direction == 1 {
                            (
                                "rgb(175,198,206)",
                                "rgb(153,255,229)",
                                "rgb(29,133,122)",
                                "rgb(204,255,242)",
                            )
                        } else {
                            (
                                "rgb(175,190,206)",
                                "rgb(153,255,255)",
                                "rgb(29,118,143)",
                                "rgb(204,255,255)",
                            )
                        };
                    let outer_stroke = "rgb(23,24,34)";
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 29)
                            .set("fill", outer_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 4),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 20.5)
                            .set("fill", middle_color)
                            .set("stroke", middle_stroke)
                            .set("stroke-width", 5),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 9)
                            .set("fill", inner_color),
                    );
                }
                CylheimChartPageViewerNoteType::Flick => {
                    let flick_color = if page_viewer.ghost_scan_line_direction == 1 {
                        "rgb(39,191,141)"
                    } else {
                        "rgb(41,170,220)"
                    };
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

                    // 添加圆形
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 11.0)
                            .set("fill", "rgb(255,255,255)"),
                    );

                    // 添加多边形
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

                    // 添加矩形
                    group = group.add(
                        Rectangle::new()
                            .set("x", x - 2.0)
                            .set("y", y - 18.0)
                            .set("height", 37.0)
                            .set("width", 4.0)
                            .set("fill", flick_color),
                    );

                    // 添加多边形
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
                }
                CylheimChartPageViewerNoteType::Drag => {
                    let (outer_color, inner_color) = if page_viewer.ghost_scan_line_direction == 1 {
                        ("rgb(182,180,203)", "rgb(170,102,255)")
                    } else {
                        ("rgb(172,180,203)", "rgb(246,102,255)")
                    };
                    let outer_stroke = "rgb(23,24,34)";
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 23)
                            .set("fill", outer_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 3),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 16)
                            .set("fill", inner_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 4),
                    );
                    if let Some(drag_child_pos) = &note.drag_child_pos {
                        let dx = calc_x_g(drag_child_pos.x);
                        let dy = calc_y_g(drag_child_pos.y);

                        let dx_val = dx - x;
                        let dy_val = dy - y;
                        let angle = if dx_val == 0.0 && dy_val == 0.0 {
                            -45.0
                        } else {
                            dy_val.atan2(dx_val).to_degrees()
                        };
                        // 定义箭头的点
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

                        // 定义箭头颜色
                        let fill_color = if page_viewer.ghost_scan_line_direction == 1 {
                            "rgb(182,180,203)"
                        } else {
                            "rgb(172,180,203)"
                        };

                        // 创建箭头的多边形
                        group = group.add(
                            Polygon::new()
                                .set("points", arrow_points)
                                .set("fill", fill_color)
                                .set("transform", format!("rotate({} {} {})", angle + 90.0, x, y)),
                        );
                    }
                }
                CylheimChartPageViewerNoteType::DragChild => {
                    let (outer_color, inner_color) = if page_viewer.ghost_scan_line_direction == 1 {
                        ("rgb(182,180,203)", "rgb(170,102,255)")
                    } else {
                        ("rgb(172,180,203)", "rgb(246,102,255)")
                    };
                    let outer_stroke = "rgb(23,24,34)";
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 12)
                            .set("fill", outer_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 2),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 8)
                            .set("fill", inner_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 2),
                    );
                }
                CylheimChartPageViewerNoteType::ClickDrag => {
                    let (outer_color, middle_color, middle_stroke, inner_color) =
                        if page_viewer.ghost_scan_line_direction == 1 {
                            (
                                "rgb(175,198,206)",
                                "rgb(153,255,229)",
                                "rgb(29,133,122)",
                                "rgb(204,255,242)",
                            )
                        } else {
                            (
                                "rgb(175,190,206)",
                                "rgb(153,255,255)",
                                "rgb(29,118,143)",
                                "rgb(204,255,255)",
                            )
                        };
                    let outer_stroke = "rgb(23,24,34)";
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 29)
                            .set("fill", outer_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 4),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 20.5)
                            .set("fill", middle_color)
                            .set("stroke", middle_stroke)
                            .set("stroke-width", 5),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 9)
                            .set("fill", inner_color),
                    );
                }
                CylheimChartPageViewerNoteType::ClickDragChild => {
                    let (outer_color, inner_color) = if page_viewer.ghost_scan_line_direction == 1 {
                        ("rgb(175,198,206)", "rgb(153,255,229)")
                    } else {
                        ("rgb(175,190,206)", "rgb(153,255,255)")
                    };
                    let outer_stroke = "rgb(23,24,34)";
                    let inner_stroke = if page_viewer.ghost_scan_line_direction == 1 {
                        "rgb(29,133,122)"
                    } else {
                        "rgb(29,118,143)"
                    };
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 12)
                            .set("fill", outer_color)
                            .set("stroke", outer_stroke)
                            .set("stroke-width", 2),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 8)
                            .set("fill", inner_color)
                            .set("stroke", inner_stroke)
                            .set("stroke-width", 2),
                    );
                }
                CylheimChartPageViewerNoteType::Hold => {
                    if note.hold_body_handler.is_some() {
                        continue;
                    }
                    let (outer_color, inner_color) = if page_viewer.ghost_scan_line_direction == 1 {
                        ("rgb(255,255,255)", "rgb(198,105,161)")
                    } else {
                        ("rgb(255,255,255)", "rgb(198,105,123)")
                    };
                    // group = group.add(
                    //     Line::new()
                    //         .set("x1", x)
                    //         .set("x2", x)
                    //         .set("y1", y)
                    //         .set("y2", end_y)
                    //         .set("stroke", "rgb(255,255,255)")
                    //         .set("stroke-dasharray", "4,4")
                    //         .set("stroke-width", 23),
                    // );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 30)
                            .set("fill", outer_color)
                            .set("stroke", "rgb(23,24,34)")
                            .set("stroke-width", 6),
                    );
                    group = group.add(
                        Rectangle::new()
                            .set("x", x - 33.0)
                            .set("y", y - 2.0)
                            .set("width", 66)
                            .set("height", 4)
                            .set("fill", outer_color),
                    );
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 19)
                            .set("fill", outer_color)
                            .set("stroke", inner_color)
                            .set("stroke-width", 6),
                    );
                }
                CylheimChartPageViewerNoteType::LongHold => {
                    if note.hold_body_handler.is_some() {
                        continue;
                    }
                    group = group.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 30)
                            .set("fill", "rgb(255,255,255)")
                            .set("stroke", "rgb(23,24,34)")
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
                }
                _ => (),
            }
            ghost_group = ghost_group.add(group);
        }
        document = document.add(ghost_group);
    }
    document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 50)
            .set("x2", 512)
            .set("y2", 50)
            .set("stroke", "white")
            .set("stroke-dasharray", "1,20.2917")
            .set("stroke-width", 2),
    );
    document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", 344)
            .set("x2", 512)
            .set("y2", 344)
            .set("stroke", "white")
            .set("stroke-dasharray", "1,20.2917")
            .set("stroke-width", 2),
    );
    document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", y1)
            .set("x2", 512)
            .set("y2", y1)
            .set("stroke", "white")
            .set("stroke-opacity", 0.7)
            .set("stroke-width", 2),
    );
    document = document.add(
        Line::new()
            .set("x1", 0)
            .set("y1", y2)
            .set("x2", 512)
            .set("y2", y2)
            .set("stroke", "white")
            .set("stroke-opacity", 0.7)
            .set("stroke-width", 2),
    );
    let arrow_points = if page_viewer.scan_line_direction == 1 {
        "0,87 0,307 16,307 16,142 32,142"
    } else {
        "0,307 0,87 16,87 16,252 32,252"
    };

    let arrow = Polygon::new()
        .set("points", arrow_points)
        .set("fill", "white");

    // println!("SCANLINE: {:>8.2}", page_viewer.scanline.max_scanline);
    if page_viewer.scanline.is_stable {
        document = document.add(
            Text::new(format!("SCANLINE:"))
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
            Text::new(format!("MAX SCANLINE:"))
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
            Text::new(format!("MIN SCANLINE:"))
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
        let event_y = calc_y(event.y);
        document = document.add(
            Line::new()
                .set("x1", 0)
                .set("y1", event_y)
                .set("x2", 512)
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
    //bottom to top: longhold body, hold body, drag link, other note(including longhold/hold head, from larger id to smaller id)

    //draw hold body
    for note in page_viewer.note_list.iter().rev() {
        let x = calc_x(note.position.x);
        let mut group = Group::new();

        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::LongHold => {
                if let Some(hold_body_handler) = &note.hold_body_handler {
                    let y = calc_y(hold_body_handler.start_y);
                    let end_y = calc_y(hold_body_handler.end_y);

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
                    let y = calc_y(hold_body_handler.start_y);
                    let end_y = calc_y(hold_body_handler.end_y);
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
        document = document.add(group);
    }
    //draw link
    for note in page_viewer.note_list.iter().rev() {
        let mut group = Group::new();

        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::Drag
            | CylheimChartPageViewerNoteType::DragChild
            | CylheimChartPageViewerNoteType::ClickDrag
            | CylheimChartPageViewerNoteType::ClickDragChild => {
                let x = calc_x(note.position.x);
                let y = calc_y(note.position.y);
                if let Some(drag_child_pos) = &note.drag_child_pos {
                    let drag_child_x = calc_x(drag_child_pos.x);
                    let drag_child_y = calc_y(drag_child_pos.y);
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
        document = document.add(group);
    }
    //draw notes
    for note in page_viewer.note_list.iter().rev() {
        let x = calc_x(note.position.x);
        let y = calc_y(note.position.y);
        let mut group = Group::new();
        match CylheimChartPageViewerNoteType::from_id(note.note_type).unwrap() {
            CylheimChartPageViewerNoteType::Click => {
                let (outer_color, middle_color, middle_stroke, inner_color) =
                    if page_viewer.scan_line_direction == 1 {
                        (
                            "rgb(175,198,206)",
                            "rgb(153,255,229)",
                            "rgb(29,133,122)",
                            "rgb(204,255,242)",
                        )
                    } else {
                        (
                            "rgb(175,190,206)",
                            "rgb(153,255,255)",
                            "rgb(29,118,143)",
                            "rgb(204,255,255)",
                        )
                    };
                let outer_stroke = "rgb(23,24,34)";
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 29)
                        .set("fill", outer_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 4),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 20.5)
                        .set("fill", middle_color)
                        .set("stroke", middle_stroke)
                        .set("stroke-width", 5),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 9)
                        .set("fill", inner_color),
                );
            }
            CylheimChartPageViewerNoteType::Flick => {
                let flick_color = if page_viewer.scan_line_direction == 1 {
                    "rgb(39,191,141)"
                } else {
                    "rgb(41,170,220)"
                };
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

                // 添加圆形
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 11.0)
                        .set("fill", "rgb(255,255,255)"),
                );

                // 添加多边形
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

                // 添加矩形
                group = group.add(
                    Rectangle::new()
                        .set("x", x - 2.0)
                        .set("y", y - 18.0)
                        .set("height", 37.0)
                        .set("width", 4.0)
                        .set("fill", flick_color),
                );

                // 添加多边形
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
            }
            CylheimChartPageViewerNoteType::Drag => {
                let (outer_color, inner_color) = if page_viewer.scan_line_direction == 1 {
                    ("rgb(182,180,203)", "rgb(170,102,255)")
                } else {
                    ("rgb(172,180,203)", "rgb(246,102,255)")
                };
                let outer_stroke = "rgb(23,24,34)";
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 23)
                        .set("fill", outer_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 3),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 16)
                        .set("fill", inner_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 4),
                );
                if let Some(drag_child_pos) = &note.drag_child_pos {
                    let dx = calc_x(drag_child_pos.x);
                    let dy = calc_y(drag_child_pos.y);

                    let dx_val = dx - x;
                    let dy_val = dy - y;
                    let angle = if dx_val == 0.0 && dy_val == 0.0 {
                        -45.0
                    } else {
                        dy_val.atan2(dx_val).to_degrees()
                    };
                    // 定义箭头的点
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

                    // 定义箭头颜色
                    let fill_color = if page_viewer.scan_line_direction == 1 {
                        "rgb(182,180,203)"
                    } else {
                        "rgb(172,180,203)"
                    };

                    // 创建箭头的多边形
                    group = group.add(
                        Polygon::new()
                            .set("points", arrow_points)
                            .set("fill", fill_color)
                            .set("transform", format!("rotate({} {} {})", angle + 90.0, x, y)),
                    );
                }
            }
            CylheimChartPageViewerNoteType::DragChild => {
                let (outer_color, inner_color) = if page_viewer.scan_line_direction == 1 {
                    ("rgb(182,180,203)", "rgb(170,102,255)")
                } else {
                    ("rgb(172,180,203)", "rgb(246,102,255)")
                };
                let outer_stroke = "rgb(23,24,34)";
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 12)
                        .set("fill", outer_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 2),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 8)
                        .set("fill", inner_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 2),
                );
            }
            CylheimChartPageViewerNoteType::ClickDrag => {
                let (outer_color, middle_color, middle_stroke, inner_color) =
                    if page_viewer.scan_line_direction == 1 {
                        (
                            "rgb(175,198,206)",
                            "rgb(153,255,229)",
                            "rgb(29,133,122)",
                            "rgb(204,255,242)",
                        )
                    } else {
                        (
                            "rgb(175,190,206)",
                            "rgb(153,255,255)",
                            "rgb(29,118,143)",
                            "rgb(204,255,255)",
                        )
                    };
                let outer_stroke = "rgb(23,24,34)";
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 29)
                        .set("fill", outer_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 4),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 20.5)
                        .set("fill", middle_color)
                        .set("stroke", middle_stroke)
                        .set("stroke-width", 5),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 9)
                        .set("fill", inner_color),
                );
            }
            CylheimChartPageViewerNoteType::ClickDragChild => {
                let (outer_color, inner_color) = if page_viewer.scan_line_direction == 1 {
                    ("rgb(175,198,206)", "rgb(153,255,229)")
                } else {
                    ("rgb(175,190,206)", "rgb(153,255,255)")
                };
                let outer_stroke = "rgb(23,24,34)";
                let inner_stroke = if page_viewer.scan_line_direction == 1 {
                    "rgb(29,133,122)"
                } else {
                    "rgb(29,118,143)"
                };
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 12)
                        .set("fill", outer_color)
                        .set("stroke", outer_stroke)
                        .set("stroke-width", 2),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 8)
                        .set("fill", inner_color)
                        .set("stroke", inner_stroke)
                        .set("stroke-width", 2),
                );
            }
            CylheimChartPageViewerNoteType::Hold => {
                if note.hold_body_handler.is_some() {
                    continue;
                }
                let (outer_color, inner_color) = if page_viewer.scan_line_direction == 1 {
                    ("rgb(255,255,255)", "rgb(198,105,161)")
                } else {
                    ("rgb(255,255,255)", "rgb(198,105,123)")
                };
                // group = group.add(
                //     Line::new()
                //         .set("x1", x)
                //         .set("x2", x)
                //         .set("y1", y)
                //         .set("y2", end_y)
                //         .set("stroke", "rgb(255,255,255)")
                //         .set("stroke-dasharray", "4,4")
                //         .set("stroke-width", 23),
                // );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 30)
                        .set("fill", outer_color)
                        .set("stroke", "rgb(23,24,34)")
                        .set("stroke-width", 6),
                );
                group = group.add(
                    Rectangle::new()
                        .set("x", x - 33.0)
                        .set("y", y - 2.0)
                        .set("width", 66)
                        .set("height", 4)
                        .set("fill", outer_color),
                );
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 19)
                        .set("fill", outer_color)
                        .set("stroke", inner_color)
                        .set("stroke-width", 6),
                );
            }
            CylheimChartPageViewerNoteType::LongHold => {
                if note.hold_body_handler.is_some() {
                    continue;
                }
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", 30)
                        .set("fill", "rgb(255,255,255)")
                        .set("stroke", "rgb(23,24,34)")
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
            }
            _ => (),
        }
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
        document = document.add(group);
    }
    document = document.add(arrow);

    Ok(document)
}

pub(crate) fn draw_all_svg_pages(
    chart_viewer: &CylheimChartViewer,
    columns: usize,
    show_ghost_note: bool,
) -> Result<Document, CylToolError> {
    let page_width = 512;
    let page_height = 384;
    let gap = 20;
    let outer_gap = 24;
    let rows = (chart_viewer.pages.len() + columns - 1) / columns;

    let total_width = (page_width + gap) * columns - gap + 2 * outer_gap;
    let total_height = (page_height + gap) * rows - gap + 2 * outer_gap;

    let mut document = Document::new()
        .set("viewBox", (0, 0, total_width, total_height))
        .set("width", total_width)
        .set("height", total_height);

    // 设置整体背景为白色
    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "rgb(54,54,54)"), // 使用浅灰色背景
    );

    for (index, page_viewer) in chart_viewer.pages.iter().enumerate() {
        let col = index % columns;
        let row = index / columns;

        let page_svg = draw_svg_page(page_viewer.clone(), show_ghost_note)?;

        let mut translated_group = Group::new().set(
            "transform",
            format!(
                "translate({}, {})",
                col * (page_width + gap) + outer_gap,
                row * (page_height + gap) + outer_gap
            ),
        );

        for child in page_svg.get_children() {
            translated_group = translated_group.add(child.clone());
        }
        document = document.add(translated_group);
    }

    Ok(document)
}

#[allow(dead_code)]
fn note_y_from_tick(tick: u32, start: u32, end: u32, direction: i32) -> f64 {
    let y = (tick - start) as f64 / (end - start) as f64;
    let result = if direction < 0 { y } else { 1.0 - y };
    if result < 0.0 {
        return 0.0;
    } else if result > 1.0 {
        return 1.0;
    } else {
        return result;
    }
}
#[allow(dead_code)]

pub(crate) fn tempo_to_scanline(tempo: u32, page_size: u32, position_function_arg: f64) -> f64 {
    (60_000_000.0 / tempo as f64) * position_function_arg * 960.0 as f64 / page_size as f64
}
use anyhow::{anyhow, Result};
use resvg::usvg;
use tiny_skia::Pixmap;

pub fn svg_to_png(svg_data: &[u8], output_size: Option<(u32, u32)>) -> Result<Vec<u8>> {
    // 解析 SVG 并创建渲染树

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    // 设置选项并包含字体数据库
    let opt = usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        ..Default::default()
    };

    let tree = usvg::Tree::from_data(svg_data, &opt).map_err(|e| anyhow!("SVG 解析失败: {}", e))?;

    // 确定输出尺寸
    let (width, height) = match output_size {
        Some((w, h)) if w > 0 && h > 0 => (w, h),
        _ => {
            let size = tree.size();
            (size.width().ceil() as u32, size.height().ceil() as u32)
        }
    };

    // 创建像素图
    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| anyhow!("无法创建 {}x{} 的像素图", width, height))?;

    // 计算缩放变换
    let scale = if let Some((w, h)) = output_size {
        let sx = w as f32 / tree.size().width() as f32;
        let sy = h as f32 / tree.size().height() as f32;
        tiny_skia::Transform::from_scale(sx, sy)
    } else {
        tiny_skia::Transform::identity()
    };

    // 渲染 SVG 到像素图
    resvg::render(&tree, scale, &mut pixmap.as_mut());

    // 编码为 PNG
    pixmap
        .encode_png()
        .map_err(|e| anyhow!("PNG 编码失败: {}", e))
}
#[allow(dead_code)]

pub fn convert_chart_to_svg(
    chart: CylheimChart,

    columns: usize,
    show_ghost_note: bool,
    labels: Option<HashMap<u32, HashMap<String, String>>>,
) -> Result<Document, CylToolError> {
    let chart_viewer = build_cylheim_chart_viewer(chart, labels)?;
    draw_all_svg_pages(&chart_viewer, columns, show_ghost_note)
}
#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
    use std::{fs, time::Instant};
    const TEST_RESOURCE_ROOT: &str = "./tests/resources/";
    const TEST_OUTPUT_ROOT: &str = "./tests/output/";
    fn get_resource_path(filename: &str) -> String {
        TEST_RESOURCE_ROOT.to_owned() + filename
    }
    fn get_output_path(filename: &str) -> String {
        TEST_OUTPUT_ROOT.to_owned() + filename
    }
    #[test]
    #[allow(unused)]
    fn test() {
        let path = get_resource_path("test_cyl_chart.json");
        let path_out = get_output_path("viewer_test_out.json");
        let svg_out = get_output_path("viewer_test_out.svg");
        let f = fs::read_to_string(path).unwrap();
        let cylchart: CylheimChart = serde_json::from_str(&f).unwrap();
        let start = Instant::now(); // 记录开始时间
        let chart_viewer = build_cylheim_chart_viewer(cylchart, None).unwrap();
        let duration = start.elapsed(); // 计算经过的时间
        println!("build_cylheim_chart_viewer time elapsed: {:?}", duration);
        let svg = draw_all_svg_pages(&chart_viewer, 4, true).unwrap();
        fs::write(svg_out, svg.to_string()).unwrap();
        fs::write(path_out, serde_json::to_string(&chart_viewer).unwrap());
        let png = svg_to_png(&svg.to_string().as_bytes(), None).unwrap();
        fs::write(get_output_path("viewer_test_out.png"), png).unwrap();
        println!("all time elapsed: {:?}", duration);
    }
}
