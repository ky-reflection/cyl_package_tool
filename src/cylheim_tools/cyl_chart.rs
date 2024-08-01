use std::{
    any::Any,
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

use super::{
    cytus1_chart::{compare_links, Cytus1Chart, Cytus1ChartLink, Cytus1ChartNote},
    utils::CylToolError,
};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
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
    #[getset(get = "pub", set = "pub")]
    page_list: Vec<CylheimChartPage>,
    #[getset(get = "pub", set = "pub")]
    tempo_list: Vec<CylheimChartTempo>,
    #[getset(get = "pub", set = "pub")]
    event_order_list: Vec<CylheimChartTickEventList>,
    #[getset(get = "pub", set = "pub")]
    note_list: Vec<CylheimChartNote>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
struct CylheimChartPage {
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
struct CylheimChartPagePositionFunction {
    #[serde(rename = "Type")]
    #[getset(get = "pub", set = "pub")]
    position_function_type: u32,
    #[serde(rename = "Arguments")]
    #[getset(get = "pub", set = "pub")]
    position_function_arguments: [f64; 2],
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
struct CylheimChartTempo {
    #[getset(get = "pub", set = "pub")]
    tick: u32,
    #[getset(get = "pub", set = "pub")]
    value: u32,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
struct CylheimChartTickEventList {
    #[getset(get = "pub", set = "pub")]
    tick: u32,
    #[getset(get = "pub", set = "pub")]
    event_list: Vec<CylheimChartEvent>,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
struct CylheimChartEvent {
    #[serde(rename = "type")]
    #[getset(get = "pub", set = "pub")]
    event_type: u32,
    #[serde(rename = "args")]
    #[getset(get = "pub", set = "pub")]
    event_args: String,
}
#[derive(Serialize, Deserialize, Debug, Getters, Setters, Clone)]
#[allow(unused)]
struct CylheimChartNote {
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
enum InsertNoteOption {}
impl CylheimChart {
    // fn to_cytus1_chart_check(&self) -> Result<(), CylToolError> {
    //     let check_note_type = |note_type: u32| match note_type {
    //         0 | 1 | 3 | 4 => true,
    //         _ => false,
    //     };
    //     for note in &self.note_list {
    //         if !check_note_type(note.note_type) {
    //             return Err(CylToolError::from(format!(
    //                 "Note {} have an invalid type {}.",
    //                 note.id, note.note_type
    //             )));
    //         }
    //     }

    //     Ok(todo!())
    // }
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
        let page_map: HashMap<u32, CylheimChartPage> = self
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
}
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn test_cytus2to1() {
        let path1 = "./tests/resources/test_valid_c2to1_chart.json";
        let path2 = "./tests/resources/test_cyl_chart.json";
        let path3 = "./tests/resources/test_c1.txt";
        let f1 = fs::read_to_string(path1).unwrap();
        let f2 = fs::read_to_string(path2).unwrap();
        let chart1: CylheimChart = serde_json::from_str(&f1).unwrap();
        let chart2: CylheimChart = serde_json::from_str(&f2).unwrap();
        let chart_test1 = chart1.to_cytus1_chart_directly(None).unwrap();
        // let chart_test2 = chart2.to_cytus1_chart(None).unwrap();
        println!("{}", chart_test1);
        fs::write(path3, chart_test1.to_string()).unwrap();
    }
}
