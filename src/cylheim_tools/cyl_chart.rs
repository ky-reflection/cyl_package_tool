use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
pub struct CylheimChart {
    format_version: u32,
    time_base: u32,
    start_offset_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_offset_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_start_without_ui: Option<bool>,
    page_list: Vec<CylheimChartPage>,
    tempo_list: Vec<CylheimChartTempo>,
    event_order_list: Vec<CylheimChartTickEventList>,
    note_list: Vec<CylheimChartNote>,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartPage {
    start_tick: u32,
    end_tick: u32,
    scan_line_direction: i32,
    #[serde(rename = "PositionFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    position_function: Option<CylheimChartPagePositionFunction>,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartPagePositionFunction {
    #[serde(rename = "Type")]
    position_function_type: u32,
    #[serde(rename = "Arguments")]
    position_function_arguments: [f64; 2],
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartTempo {
    tick: u32,
    value: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartTickEventList {
    tick: u32,
    event_list: Vec<CylheimChartEvent>,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartEvent {
    #[serde(rename = "type")]
    event_type: u32,
    #[serde(rename = "args")]
    event_args: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct CylheimChartNote {
    page_index: u32,
    #[serde(rename = "type")]
    note_type: u32,
    id: u32,
    tick: u32,
    x: f64,
    has_sibling: bool,
    hold_tick: u32,
    next_id: i32,
    is_forward: bool,
    #[serde(rename = "NoteDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    falling_note_direction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approach_rate: Option<f64>,
}
