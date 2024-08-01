use super::utils::CylToolError;
use super::CylheimChart;
use getset::{Getters, MutGetters, Setters};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::result::Result;
use std::{fmt, str::FromStr};
#[derive(Serialize, Deserialize, Debug, Clone, Getters, Setters)]

pub struct Cytus1Chart {
    #[getset(get = "pub", set = "pub")]
    version: u32,
    #[getset(get = "pub", set = "pub")]
    bpm: f64,
    #[getset(get = "pub", set = "pub")]
    page_shift: f64,
    #[getset(get = "pub", set = "pub")]
    page_size: f64,
    #[getset(get = "pub", set = "pub")]
    notes: Vec<Cytus1ChartNote>,
    #[getset(get = "pub", set = "pub")]
    links: Vec<Cytus1ChartLink>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Getters, Setters)]

pub struct Cytus1ChartNote {
    #[getset(get = "pub", set = "pub")]
    id: u32,
    #[getset(get = "pub", set = "pub")]
    time: f64,
    #[getset(get = "pub", set = "pub")]
    x: f64,
    #[getset(get = "pub", set = "pub")]
    hold_length: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Getters, Setters, MutGetters)]

pub struct Cytus1ChartLink {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    link: Vec<u32>,
}

impl Default for Cytus1Chart {
    fn default() -> Self {
        Self {
            version: 2,
            bpm: 0.0,
            page_shift: 0.0,
            page_size: 0.0,
            notes: Vec::new(),
            links: Vec::new(),
        }
    }
}
impl Cytus1ChartNote {
    pub fn new(id: u32, time: f64, x: f64, hold_length: f64) -> Self {
        Self {
            id,
            time,
            x,
            hold_length,
        }
    }
}
impl Default for Cytus1ChartLink {
    fn default() -> Self {
        Self { link: Vec::new() }
    }
}
impl fmt::Display for Cytus1Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "VERSION {}", self.version)?;
        writeln!(f, "BPM {:.6}", self.bpm)?;
        writeln!(f, "PAGE_SHIFT {:.6}", self.page_shift)?;
        writeln!(f, "PAGE_SIZE {:.6}", self.page_size)?;
        for note in &self.notes {
            writeln!(f, "{}", note)?;
        }
        for link in &self.links {
            writeln!(f, "{}", link)?;
        }
        Ok(())
    }
}

impl fmt::Display for Cytus1ChartLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LINK")?;
        for item in &self.link {
            write!(f, " {}", item)?;
        }
        Ok(())
    }
}
impl fmt::Display for Cytus1ChartNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Note\t{}\t{:.6}\t{:.6}\t{:.6}",
            self.id, self.time, self.x, self.hold_length
        )
    }
}
impl FromStr for Cytus1Chart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_re = Regex::new(r"VERSION (\d+)").map_err(|e| e.to_string())?;
        let bpm_re = Regex::new(r"BPM ([\d\.]+)").map_err(|e| e.to_string())?;
        let page_shift_re = Regex::new(r"PAGE_SHIFT ([\d\.]+)").map_err(|e| e.to_string())?;
        let page_size_re = Regex::new(r"PAGE_SIZE ([\d\.]+)").map_err(|e| e.to_string())?;
        let note_re = Regex::new(r"NOTE\s+(\d+)\s+([\d\.]+)\s+([\d\.]+)\s+([\d\.]+)")
            .map_err(|e| e.to_string())?;
        let link_re = Regex::new(r"LINK((?:\s+\d+)+)").map_err(|e| e.to_string())?;

        let version = version_re
            .captures(s)
            .and_then(|cap| cap.get(1))
            .ok_or("Failed to parse VERSION")?
            .as_str()
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let bpm = bpm_re
            .captures(s)
            .and_then(|cap| cap.get(1))
            .ok_or("Failed to parse BPM")?
            .as_str()
            .parse::<f64>()
            .map_err(|e| e.to_string())?;

        let page_shift = page_shift_re
            .captures(s)
            .and_then(|cap| cap.get(1))
            .ok_or("Failed to parse PAGE_SHIFT")?
            .as_str()
            .parse::<f64>()
            .map_err(|e| e.to_string())?;

        let page_size = page_size_re
            .captures(s)
            .and_then(|cap| cap.get(1))
            .ok_or("Failed to parse PAGE_SIZE")?
            .as_str()
            .parse::<f64>()
            .map_err(|e| e.to_string())?;

        let notes = note_re
            .captures_iter(s)
            .map(|cap| Cytus1ChartNote {
                id: cap.get(1).unwrap().as_str().parse().unwrap(),
                time: cap.get(2).unwrap().as_str().parse().unwrap(),
                x: cap.get(3).unwrap().as_str().parse().unwrap(),
                hold_length: cap.get(4).unwrap().as_str().parse().unwrap(),
            })
            .collect();

        let links = link_re
            .captures_iter(s)
            .map(|cap| Cytus1ChartLink {
                link: cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            })
            .collect();

        Ok(Cytus1Chart {
            version,
            bpm,
            page_shift,
            page_size,
            notes,
            links,
        })
    }
}

impl Cytus1Chart {
    #[allow(dead_code)]
    fn into_c2chart(&self) -> Result<CylheimChart, CylToolError> {
        Err(CylToolError::from("error"))
    }
}
pub fn compare_links(a: &Cytus1ChartLink, b: &Cytus1ChartLink) -> Ordering {
    let len: usize = a.link.len().min(b.link.len());
    for i in 0..len {
        match a.link[i].cmp(&b.link[i]) {
            Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }
    // 如果所有比较的 `id` 都相等，比较 `ids` 向量的长度
    a.link.len().cmp(&b.link.len())
}
#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, vec};
    #[test]
    fn test_cytus1_chart_to_string() {
        let note = Cytus1ChartNote {
            id: 2,
            time: 2.234234,
            x: 0.5,
            hold_length: 1.333,
        };
        let link = Cytus1ChartLink {
            link: vec![1, 2, 3, 4, 5],
        };
        let chart = Cytus1Chart {
            version: 2,
            bpm: 200.0,
            page_shift: 0.0,
            page_size: 240.0 / 200.0,
            notes: vec![
                note.clone(),
                note.clone(),
                note.clone(),
                note.clone(),
                note.clone(),
            ],
            links: vec![link.clone(), link.clone()],
        };
        println!("{}", chart);
    }
    #[test]
    fn test_cytus1_chart_from_str() {
        let path = "./tests/resources/test_cytus1_chart.txt";
        let f = fs::read_to_string(path).unwrap();
        let config: Cytus1Chart = f.parse().expect("Failed to parse config");
        println!("{:#?}", config);
    }
}
