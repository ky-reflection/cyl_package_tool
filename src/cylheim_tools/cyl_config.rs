use super::cytoid_level::CylheimLevelMetaConfig;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CylheimProjectConfig {
    #[serde(rename = "Version", default = "default_cyl_version")]
    version: isize,
    #[serde(rename = "ChartInfos")]
    chart_infos: Vec<ChartInfo>,
    #[serde(rename = "LastOpenedChart")]
    last_open_chart: Option<String>,
    #[serde(rename = "LastEditedTime")]
    last_edit_time: f64,
    #[serde(rename = "LevelMetaConfig")]
    level_meta_config: Option<CylheimLevelMetaConfig>,
}
fn default_cyl_version() -> isize {
    0
}
#[derive(Serialize, Deserialize, Debug)]
struct ChartInfo {
    #[serde(rename = "DiffName")]
    diff_name: String,
    #[serde(rename = "Diff")]
    diff_value: String,
    #[serde(rename = "FileName")]
    chart_path: String,
    #[serde(rename = "Media")]
    song_path: String,
    #[serde(rename = "Video")]
    video_path: String,
    #[serde(rename = "Bg")]
    background_path: String,
    #[serde(rename = "Icon")]
    icon_path: String,
    #[serde(rename = "SongName")]
    song_name: String,
    #[serde(rename = "ThemeColor")]
    theme_color: String,
    #[serde(rename = "DiffTextColor")]
    diff_text_color: String,
    #[serde(rename = "DiffBgColor")]
    diff_background_color: String,
    #[serde(rename = "StoryboardPath")]
    storyboard_path: Option<String>,
    #[serde(rename = "Bookmarks")]
    bookmark: Vec<CylheimBookmark>,
    #[serde(rename = "GenerateEventConfig", default)]
    generate_event_config: GenerateEventConfig,
}
#[derive(Serialize, Deserialize, Debug)]
struct CylheimBookmark {
    #[serde(rename = "Tick")]
    tick: usize,
    #[serde(rename = "Tag")]
    tag: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GenerateEventConfig {
    #[serde(rename = "IgnoreFrom")]
    ignore_from: f64,
    #[serde(rename = "IgnoreTo")]
    ignore_to: f64,
    #[serde(rename = "IgnoreSameTypeInterval")]
    ignore_same_type_interval: usize,
    #[serde(rename = "UseLastEventAsRef")]
    use_last_event_as_ref: bool,
    #[serde(rename = "IgnorePosFunc")]
    ignore_pos_func: bool,
    #[serde(rename = "BaseTicks")]
    base_ticks: Option<Vec<usize>>,
}
impl Default for GenerateEventConfig {
    fn default() -> Self {
        Self {
            ignore_from: 0.8,
            ignore_to: 1.25,
            ignore_same_type_interval: 480,
            use_last_event_as_ref: false,
            ignore_pos_func: false,
            base_ticks: None,
        }
    }
}

impl CylheimProjectConfig {
    #[allow(unused)]
    fn sort_bookmarks_by_tick(&mut self) {
        for chart_info in &mut self.chart_infos {
            chart_info.bookmark.sort_by_key(|bookmark| bookmark.tick);
        }
    }
}
