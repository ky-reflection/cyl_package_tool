use std::{cmp::min, collections::HashMap};

use super::{
    cyl_chart::{CylheimChart, CylheimChartPagePositionFunction, CylheimChartTempo},
    utils::CylToolError,
};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use svg::Document;
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
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, Copy, MutGetters, Default)]
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
impl From<CylheimChartPageViewerNoteType> for u32 {
    fn from(val: CylheimChartPageViewerNoteType) -> Self {
        match val {
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
#[derive(Default)]
pub enum CylheimChartPageViewerScanlineEventType {
    Accelerate,
    Decelerate,
    #[default]
    Stable,
}
fn validate_chart_for_viewer(chart: &CylheimChart) -> Result<(), CylToolError> {
    if chart.page_list().is_empty() {
        return Err(CylToolError::from("chart has no pages"));
    }
    if chart.tempo_list().is_empty() {
        return Err(CylToolError::from("chart has no tempos"));
    }
    if chart.tempo_list().iter().any(|tempo| *tempo.value() == 0) {
        return Err(CylToolError::from("tempo value must be greater than 0"));
    }
    if chart
        .page_list()
        .iter()
        .any(|page| *page.end_tick() <= *page.start_tick())
    {
        return Err(CylToolError::from(
            "page end_tick must be greater than start_tick",
        ));
    }
    Ok(())
}
#[path = "chart_viewer_svg.rs"]
mod chart_viewer_svg;
pub(crate) use chart_viewer_svg::draw_all_svg_pages;
#[allow(dead_code)]
pub(crate) fn build_cylheim_chart_viewer(
    chart: CylheimChart,
    labels: Option<HashMap<u32, HashMap<String, String>>>,
) -> Result<CylheimChartViewer, CylToolError> {
    validate_chart_for_viewer(&chart)?;
    let mut chart = chart;
    chart.sort_internal_data();
    chart.sort_and_reassign_note_ids();

    let page_count = chart.page_list().len();
    let mut note_viewer_list: Vec<CylheimChartPageViewerNotes> =
        Vec::with_capacity(chart.note_list().len());
    for note in chart.note_list() {
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
    for note in chart.note_list() {
        if *note.next_id() >= 0
            && (*note.note_type() == CylheimChartPageViewerNoteType::Drag.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::DragChild.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::ClickDrag.get_id()
                || *note.note_type() == CylheimChartPageViewerNoteType::ClickDragChild.get_id())
        {
            let next_note_index = *note.next_id() as usize;
            let current_note_index = *note.id() as usize;
            let next_note_pos = *note_viewer_list[next_note_index].position();
            let current_note_pos = *note_viewer_list[current_note_index].position();

            note_viewer_list[next_note_index].set_drag_parent_pos(Some(next_note_pos));
            note_viewer_list[current_note_index].set_drag_child_pos(Some(current_note_pos));
        }
    }

    let mut notes_by_page: Vec<Vec<CylheimChartPageViewerNotes>> =
        vec![Vec::new(); page_count];
    for note_viewer in note_viewer_list {
        notes_by_page[note_viewer.page_index as usize].push(note_viewer);
    }

    let tempo_list = chart.tempo_list();
    let pages: Vec<CylheimChartPageViewer> = chart
        .page_list()
        .iter()
        .enumerate()
        .map(|(page_index, page)| {
            let start_tick = *page.start_tick();
            let end_tick = *page.end_tick();
            let tempo_start = tempo_list.partition_point(|tempo| *tempo.tick() < start_tick);
            let tempo_end = tempo_list.partition_point(|tempo| *tempo.tick() < end_tick);
            CylheimChartPageViewer {
                time_base: *chart.time_base(),
                page_index: page_index as u32,
                start_tick,
                end_tick,
                scan_line_direction: *page.scan_line_direction(),
                ghost_scan_line_direction: 0,
                tempo_list: tempo_list[tempo_start..tempo_end].to_vec(),
                note_list: std::mem::take(&mut notes_by_page[page_index]),
                ghost_note_list: Vec::new(),
                position_function: page.position_function().clone(),
                ghost_position_function: None,
                scanline: CylheimChartPageViewerScanlineHandler::default(),
            }
        })
        .collect();

    let mut chart_viewer = CylheimChartViewer { chart, pages };
    for note in chart_viewer.chart.note_list() {
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
    let page_count = chart_viewer.pages.len();
    for index in 0..page_count.saturating_sub(1) {
        chart_viewer.pages[index].ghost_scan_line_direction =
            chart_viewer.pages[index + 1].scan_line_direction;
        chart_viewer.pages[index].ghost_position_function =
            chart_viewer.pages[index + 1].position_function.clone();
    }
    let first_page_size = chart_viewer.pages[0].end_tick - chart_viewer.pages[0].start_tick;
    let first_page_pf_arg = chart_viewer.pages[0]
        .position_function
        .as_ref()
        .map_or(1.0, |x| x.position_function_arguments()[0]);
    let mut current_tempo_value = *chart_viewer.chart.tempo_list()[0].value();
    let mut current_scanline =
        tempo_to_scanline(current_tempo_value, first_page_size, first_page_pf_arg);
    for page in chart_viewer.pages.iter_mut() {
        let page_size = page.end_tick - page.start_tick;
        let page_position_function_arg = page
            .position_function
            .as_ref()
            .map_or(1.0, |x| x.position_function_arguments()[0]);
        if page.tempo_list.is_empty() {
            page.scanline.is_stable = true;
            let page_scanline = tempo_to_scanline(
                current_tempo_value,
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
                        event_type,
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
                        event_type,
                    });
                current_tempo_value = *tempo.value();
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


#[allow(dead_code)]
fn note_y_from_tick(tick: u32, start: u32, end: u32, direction: i32) -> f64 {
    let y = (tick - start) as f64 / (end - start) as f64;
    let result = if direction < 0 { y } else { 1.0 - y };
    result.clamp(0.0, 1.0)
}
#[allow(dead_code)]
pub(crate) fn tempo_to_scanline(tempo: u32, page_size: u32, position_function_arg: f64) -> f64 {
    (60_000_000.0 / tempo as f64) * position_function_arg * 960.0_f64 / page_size as f64
}
use anyhow::{anyhow, Result};
use resvg::usvg;
use tiny_skia::Pixmap;

pub fn svg_to_png(svg_data: &[u8], output_size: Option<(u32, u32)>) -> Result<Vec<u8>> {
    // 瑙ｆ瀽 SVG 骞跺垱寤烘覆鏌撴爲

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    // 璁剧疆閫夐」骞跺寘鍚瓧浣撴暟鎹簱
    let opt = usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        ..Default::default()
    };

    let tree = usvg::Tree::from_data(svg_data, &opt).map_err(|e| anyhow!("SVG 瑙ｆ瀽澶辫触: {}", e))?;

    // 纭畾杈撳嚭灏哄
    let (width, height) = match output_size {
        Some((w, h)) if w > 0 && h > 0 => (w, h),
        _ => {
            let size = tree.size();
            (size.width().ceil() as u32, size.height().ceil() as u32)
        }
    };

    // 鍒涘缓鍍忕礌鍥?
    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| anyhow!("鏃犳硶鍒涘缓 {}x{} 鐨勫儚绱犲浘", width, height))?;

    // 璁＄畻缂╂斁鍙樻崲
    let scale = if let Some((w, h)) = output_size {
        let sx = w as f32 / tree.size().width();
        let sy = h as f32 / tree.size().height();
        tiny_skia::Transform::from_scale(sx, sy)
    } else {
        tiny_skia::Transform::identity()
    };

    // 娓叉煋 SVG 鍒板儚绱犲浘
    resvg::render(&tree, scale, &mut pixmap.as_mut());

    // 缂栫爜涓?PNG
    pixmap
        .encode_png()
        .map_err(|e| anyhow!("PNG 缂栫爜澶辫触: {}", e))
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
    fn ensure_output_dir() {
        fs::create_dir_all(TEST_OUTPUT_ROOT).unwrap();
    }
    #[test]
    fn test_drag_next_id_remapped() {
        let path = get_resource_path("viewer_drag_chain.json");
        let chart: CylheimChart = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let viewer = build_cylheim_chart_viewer(chart, None).unwrap();
        let notes = &viewer.pages()[0].note_list();
        assert_eq!(notes.len(), 3);
        let drag = notes.iter().find(|n| n.note_type() == &3).unwrap();
        assert!(drag.drag_child_pos().is_some(), "drag head should link to child");
        let child_count = notes
            .iter()
            .filter(|n| *n.note_type() == 4 && n.drag_parent_pos().is_some())
            .count();
        assert_eq!(child_count, 2, "both drag children should have parent pos");
    }
    #[test]
    #[allow(unused)]
    fn test() {
        ensure_output_dir();
        let path = get_resource_path("test_cyl_chart.json");
        let path_out = get_output_path("viewer_test_out.json");
        let svg_out = get_output_path("viewer_test_out.svg");
        let f = fs::read_to_string(path).unwrap();
        let cylchart: CylheimChart = serde_json::from_str(&f).unwrap();
        let start = Instant::now(); // 璁板綍寮€濮嬫椂闂?
        let chart_viewer = build_cylheim_chart_viewer(cylchart, None).unwrap();
        let duration = start.elapsed(); // 璁＄畻缁忚繃鐨勬椂闂?
        println!("build_cylheim_chart_viewer time elapsed: {:?}", duration);
        let svg = draw_all_svg_pages(&chart_viewer, 4, true).unwrap();
        fs::write(svg_out, svg.to_string()).unwrap();
        fs::write(path_out, serde_json::to_string(&chart_viewer).unwrap());
        let png = svg_to_png(svg.to_string().as_bytes(), None).unwrap();
        fs::write(get_output_path("viewer_test_out.png"), png).unwrap();
        println!("all time elapsed: {:?}", duration);
    }
}
