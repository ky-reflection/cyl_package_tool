use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CylheimLevelMetaConfig {
    schema_version: u32,
    version: u32,
    id: String,
    title: String,
    title_localized: String,
    artist: String,
    artist_localized: String,
    artist_source: String,
    illustrator: String,
    illustrator_localized: String,
    illustrator_source: String,
    charter: String,
    storyboarder: String,
    music: LevelFilePathConfig,
    music_preview: LevelFilePathConfig,
    background: LevelFilePathConfig,
    charts: Vec<CylheimLevelChartConfig>,
    #[serde(rename = "SavePath")]
    save_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LevelFilePathConfig {
    #[serde(rename = "path")]
    path: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct CylheimLevelChartConfig {
    #[serde(rename = "type")]
    chart_type: String,
    name: String,
    difficulty: f64,
    path: String,
    #[serde(rename = "GenerateNewFeaturesToStoryboard")]
    generate_new_features_to_storyboard: bool,
    #[serde(rename = "VideoPath")]
    video_path: String,
}
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CytoidLevelMetaConfig {
    schema_version: u32,
    version: u32,
    id: String,
    title: String,
    title_localized: Option<String>,
    artist: String,
    artist_localized: Option<String>,
    artist_source: Option<String>,
    illustrator: String,
    illustrator_localized: Option<String>,
    illustrator_source: Option<String>,
    charter: String,
    storyboarder: Option<String>,
    music: LevelFilePathConfig,
    music_preview: LevelFilePathConfig,
    background: LevelFilePathConfig,
    charts: Vec<CytoidLevelChartConfig>,
}
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
struct CytoidLevelChartConfig {
    #[serde(rename = "type")]
    chart_type: String,
    name: String,
    difficulty: f64,
    path: String,
    music_override: Option<LevelFilePathConfig>,
    storyboard: Option<LevelFilePathConfig>,
}
