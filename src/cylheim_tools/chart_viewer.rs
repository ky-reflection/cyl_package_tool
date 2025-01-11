use super::{
    cyl_chart::{
        CylheimChart, CylheimChartNote, CylheimChartPage, CylheimChartPagePositionFunction,
        CylheimChartTempo,
    },
    utils::CylToolError,
};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    long_hold_handler: Option<CylheimChartPageViewerLongHoldHandler>,
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
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub(crate) struct CylheimChartPageViewerLongHoldHandler {
    #[getset(get = "pub", set = "pub")]
    with_head: bool,
    #[getset(get = "pub", set = "pub")]
    start_y: f64,
    #[getset(get = "pub", set = "pub")]
    end_y: f64,
}
impl CylheimChartPageViewerLongHoldHandler {
    #[allow(dead_code)]
    fn check_start_end(&self) -> bool {
        self.start_y <= self.end_y
    }
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
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
    CustomNoteType(i32),
}
impl CylheimChartPageViewerNoteType {
    #[allow(unused)]

    pub fn get_id(&self) -> i32 {
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
    pub fn from_id(id: i32) -> Option<Self> {
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
            _ => None, // 无效ID返回 None
        }
    }
}

pub(crate) fn build_cylheim_page_viewer(
    chart: CylheimChart,
) -> Result<HashMap<u32, CylheimChartPageViewer>, CylToolError> {
    todo!()
}
#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
    use std::fs;
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
        panic!();
        // let a = CylheimChartPageViewer {
        //     todo!()
        // };
    }
}
