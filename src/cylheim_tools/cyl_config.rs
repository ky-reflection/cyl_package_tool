use super::cytoid_level::CylheimLevelMetaConfig;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CylheimProjectConfig {
    #[serde(rename = "Version", default = "default_cyl_version")]
    pub version: i32,
    #[serde(rename = "ChartInfos")]
    pub chart_infos: Vec<ChartInfo>,
    #[serde(rename = "LastOpenedChart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_open_chart: Option<String>,
    #[serde(rename = "LastEditedTime")]
    pub last_edit_time: f64,
    #[serde(rename = "LevelMetaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_meta_config: Option<CylheimLevelMetaConfig>,
}
fn default_cyl_version() -> i32 {
    0
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChartInfo {
    #[serde(rename = "DiffName")]
    pub diff_name: String,
    #[serde(rename = "Diff")]
    pub diff_value: String,
    #[serde(rename = "FileName")]
    pub chart_path: String,
    #[serde(rename = "Media")]
    pub song_path: String,
    #[serde(rename = "Video")]
    pub video_path: String,
    #[serde(rename = "Bg")]
    pub background_path: String,
    #[serde(rename = "Icon")]
    pub icon_path: String,
    #[serde(rename = "SongName")]
    pub song_name: String,
    #[serde(rename = "ThemeColor")]
    pub theme_color: String,
    #[serde(rename = "DiffTextColor")]
    pub diff_text_color: String,
    #[serde(rename = "DiffBgColor")]
    pub diff_background_color: String,
    #[serde(rename = "StoryboardPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storyboard_path: Option<String>,
    #[serde(rename = "Bookmarks")]
    pub bookmark: Vec<CylheimBookmark>,
    #[serde(rename = "GenerateEventConfig", default)]
    pub generate_event_config: GenerateEventConfig,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CylheimBookmark {
    #[serde(rename = "Tick")]
    tick: u32,
    #[serde(rename = "Tag")]
    tag: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateEventConfig {
    #[serde(rename = "IgnoreFrom")]
    ignore_from: f64,
    #[serde(rename = "IgnoreTo")]
    ignore_to: f64,
    #[serde(rename = "IgnoreSameTypeInterval")]
    ignore_same_type_interval: u32,
    #[serde(rename = "UseLastEventAsRef")]
    use_last_event_as_ref: bool,
    #[serde(rename = "IgnorePosFunc")]
    ignore_pos_func: bool,
    #[serde(rename = "BaseTicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ticks: Option<Vec<u32>>,
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
    #[allow(dead_code)]
    pub fn sort_bookmarks_by_tick(&mut self) {
        for chart_info in &mut self.chart_infos {
            chart_info.bookmark.sort_by_key(|bookmark| bookmark.tick);
        }
    }
}
