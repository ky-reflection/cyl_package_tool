use std::collections::{HashMap, HashSet};

use super::{
    cytus1_chart::{compare_links, Cytus1Chart, Cytus1ChartLink, Cytus1ChartNote},
    utils::CylToolError,
};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone, MutGetters)]
#[allow(unused)]
pub struct CylheimChart {
    #[getset(get = "pub", set = "pub")]
    format_version: u32,
    #[getset(get = "pub", set = "pub")]
    time_base: u32,
    #[getset(get = "pub", set = "pub")]
    start_offset_time: f64,
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end_offset_time: Option<f64>,
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub")]
    is_start_without_ui: Option<bool>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    page_list: Vec<CylheimChartPage>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    tempo_list: Vec<CylheimChartTempo>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    event_order_list: Vec<CylheimChartTickEventList>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    note_list: Vec<CylheimChartNote>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartPage {
    #[getset(get = "pub", set = "pub")]
    start_tick: u32,
    #[getset(get = "pub", set = "pub")]
    end_tick: u32,
    #[getset(get = "pub", set = "pub")]
    scan_line_direction: i32,
    #[getset(get = "pub", set = "pub")]
    #[getset(get = "pub", set = "pub")]
    #[serde(rename = "PositionFunction", skip_serializing_if = "Option::is_none")]
    position_function: Option<CylheimChartPagePositionFunction>,
}
impl CylheimChartPage {
    pub fn get_page_size(&self) -> i32 {
        self.end_tick as i32 - self.start_tick as i32
    }
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartPagePositionFunction {
    #[serde(rename = "Type")]
    #[getset(get = "pub", set = "pub")]
    position_function_type: u32,
    #[serde(rename = "Arguments")]
    #[getset(get = "pub", set = "pub")]
    position_function_arguments: [f64; 2],
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartTempo {
    #[getset(get = "pub", set = "pub")]
    tick: u32,
    #[getset(get = "pub", set = "pub")]
    value: u32,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartTickEventList {
    #[getset(get = "pub", set = "pub")]
    tick: u32,
    #[getset(get = "pub", set = "pub")]
    event_list: Vec<CylheimChartEvent>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartEvent {
    #[serde(rename = "type")]
    #[getset(get = "pub", set = "pub")]
    event_type: u32,
    #[serde(rename = "args")]
    #[getset(get = "pub", set = "pub")]
    event_args: String,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
pub(crate) struct CylheimChartNote {
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
    x: f64,
    #[getset(get = "pub", set = "pub")]
    has_sibling: bool,
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
#[allow(dead_code)]
enum InsertNoteOption {}
impl CylheimChart {
    #[allow(dead_code)]
    fn to_cytus1_chart_directly(
        &self,
        page_shift: Option<f64>,
    ) -> Result<Cytus1Chart, CylToolError> {
        let check_note_type = |note_type: u32| match note_type {
            0 | 1 | 3 | 4 => true,
            _ => false,
        };
        let get_tick_time_second = |tempo: u32, time_base: u32, tick: u32| {
            tick as f64 / time_base as f64 * tempo as f64 / 1000.0 / 1000.0
        };
        for note in &self.note_list {
            if !check_note_type(note.note_type) {
                return Err(CylToolError::from(format!(
                    "Note {} have an invalid type {}.",
                    note.id, note.note_type
                )));
            }
            if note.x < 0.0 || note.x > 1.0 {
                return Err(CylToolError::from(format!(
                    "Note {} have an invalid x {}.",
                    note.id, note.x
                )));
            }
        }
        let time_base = self.time_base;
        let base_page_size = self.page_list.get(0).unwrap().get_page_size();
        let base_tempo = self.tempo_list.get(0).unwrap();
        for (page_index, page) in self.page_list.iter().enumerate() {
            if page.get_page_size() != base_page_size {
                return Err(CylToolError::from(format!(
                    "Page {} have an invalid pagesize {}.",
                    page_index,
                    page.get_page_size()
                )));
            }
        }
        for tempo in &self.tempo_list {
            if tempo.value != base_tempo.value {
                return Err(CylToolError::from(format!(
                    "Tick {} have an invalid tempo {}.",
                    tempo.tick, tempo.value
                )));
            }
        }
        let mut target_chart = Cytus1Chart::default();
        let note_map: HashMap<u32, CylheimChartNote> = self
            .note_list
            .clone()
            .into_iter()
            .map(|note| (note.id, note))
            .collect();
        let _page_map: HashMap<u32, CylheimChartPage> = self
            .page_list
            .clone()
            .into_iter()
            .enumerate()
            .map(|(page_index, page)| (page_index as u32, page))
            .collect();
        let c1_bpm = 2.0 * 60_000_000.0 / base_tempo.value as f64;
        let c1_page_size = 240.0 / c1_bpm;
        let c1_page_shift = page_shift.unwrap_or(0.0);
        let mut c1_note_vec: Vec<Cytus1ChartNote> = note_map
            .iter()
            .map(|(note_id, note)| {
                Cytus1ChartNote::new(
                    *note_id,
                    get_tick_time_second(base_tempo.value, time_base, note.tick),
                    note.x,
                    match note.note_type {
                        1 => get_tick_time_second(base_tempo.value, time_base, note.hold_tick),
                        _ => 0.0,
                    },
                )
            })
            .collect();
        let mut c1_link_vec: Vec<Cytus1ChartLink> = Vec::new();
        let mut visited_note: HashSet<u32> = HashSet::new();
        for (note_id, note) in &note_map {
            if visited_note.contains(note_id) {
                continue;
            }
            match note.note_type {
                3 => {}
                _ => continue,
            }
            let mut current_id = note.id as i32;
            let mut link = Cytus1ChartLink::default();
            while current_id != -1 {
                link.link_mut().push(current_id as u32);
                visited_note.insert(current_id as u32);
                if let Some(next_note) = note_map.get(&(current_id as u32)) {
                    current_id = next_note.next_id;
                } else {
                    break;
                }
            }
            c1_link_vec.push(link);
        }
        c1_note_vec.sort_by_key(|x| *x.id());
        c1_link_vec.sort_by(compare_links);
        target_chart.set_bpm(c1_bpm);
        target_chart.set_page_shift(c1_page_shift);
        target_chart.set_page_size(c1_page_size);
        target_chart.set_notes(c1_note_vec);
        target_chart.set_links(c1_link_vec);
        Ok(target_chart)
    }
    #[allow(dead_code)]
    pub fn to_cytus1_chart_with_pageshift(
        &self,
        use_flag: bool,
    ) -> Result<Cytus1Chart, CylToolError> {
        let get_tick_time_second = |tempo: u32, time_base: u32, tick: u32| {
            tick as f64 / time_base as f64 * tempo as f64 / 1000.0 / 1000.0
        };
        let mut current_chart = self.clone();
        let mut cytus1_flag_check = !use_flag;
        if use_flag {
            if let Some(cytus1_flag) = current_chart.event_order_list.get(0) {
                if current_chart.event_order_list.len() == 1
                    && cytus1_flag.tick == 0
                    && cytus1_flag.event_list.len() == 1
                {
                    if let Some(cytus1_flag_event) = cytus1_flag.event_list.get(0) {
                        if cytus1_flag_event.event_args.contains("#DEFINE CYTUS1") {
                            cytus1_flag_check = true;
                        }
                    }
                }
            }
        }
        if !cytus1_flag_check {
            return Err(CylToolError::from(format!("Cannot find cytus1_flag.")));
        }
        let mut cytus1_tempo_check = false;
        match current_chart.tempo_list.len() {
            2 => {
                if let Some(zero_tempo) = current_chart.tempo_list.get(0) {
                    if let Some(true_tempo) = current_chart.tempo_list.get(1) {
                        if let Some(first_page) = current_chart.page_list.get(0) {
                            if zero_tempo.tick == 0
                                && zero_tempo.value == 0
                                && true_tempo.tick - zero_tempo.tick > 0
                                && true_tempo.tick - zero_tempo.tick
                                    < 2 * first_page.get_page_size() as u32
                                && first_page.scan_line_direction == 1
                            {
                                cytus1_tempo_check = true;
                            }
                        }
                    }
                }
            }
            1 => {
                if let Some(true_tempo) = current_chart.tempo_list.get(0) {
                    cytus1_tempo_check = true_tempo.tick == 0 && true_tempo.value > 0
                }
            }
            _ => (),
        };
        if !cytus1_tempo_check {
            return Err(CylToolError::from(format!("Invalid tempo.")));
        }
        let time_base = current_chart.time_base;
        let true_tempo = current_chart
            .tempo_list
            .get(current_chart.tempo_list.len() - 1)
            .unwrap()
            .clone();
        let page_shift = get_tick_time_second(true_tempo.value, time_base, true_tempo.tick);
        current_chart.event_order_list_mut().clear();
        current_chart.tempo_list_mut().clear();
        current_chart.tempo_list_mut().push(CylheimChartTempo {
            tick: 0,
            value: true_tempo.value,
        });
        for note in current_chart.note_list_mut() {
            note.set_tick(note.tick - true_tempo.tick);
        }
        let target = current_chart.to_cytus1_chart_directly(Some(page_shift));
        target
    }
}
#[cfg(test)]
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
    fn test_cytus2to1() {
        let path1 = get_resource_path("test_valid_c2to1_chart.json");
        let path2 = get_resource_path("test_cyl_chart.json");
        let path3 = get_output_path("test_c1.txt");
        let f1 = fs::read_to_string(path1).unwrap();
        let f2 = fs::read_to_string(path2).unwrap();
        let chart1: CylheimChart = serde_json::from_str(&f1).unwrap();
        let chart2: CylheimChart = serde_json::from_str(&f2).unwrap();
        let chart_test1 = chart1.to_cytus1_chart_directly(None).unwrap();
        // let chart_test2 = chart2.to_cytus1_chart(None).unwrap();
        println!("{}", chart_test1);
        fs::write(path3, chart_test1.to_string()).unwrap();
    }
    #[test]
    fn test_cytus2to1_with_flag() {
        let path = get_resource_path("cylchart_with_cytus1_flag.json");
        let path_out = get_output_path("cylchart_with_cytus1_flag_converted.txt");
        let f = fs::read_to_string(path).unwrap();
        let cylchart: CylheimChart = serde_json::from_str(&f).unwrap();
        let cytus1chart = cylchart.to_cytus1_chart_with_pageshift(true).unwrap();
        println!("{}", cytus1chart);
        fs::write(path_out, cytus1chart.to_string()).unwrap();
    }
}
