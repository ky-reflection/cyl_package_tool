// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
pub enum CylheimThemeColor {
    Paff,
    NekoTheElder,
    Robo,
    Ivy,
    CrystalPunk,
    Vanessa,
    Bobo,
    Graffj,
    Amiya,
    Kaf,
    Alice,
    Hans,
    KizunaAi,
    Miku,
    Ilka,
    Xenon,
    Conner,
    Cherry,
    Joe,
    Sagar,
    Rin,
    Aroma,
    Nora,
    NekoTheYounger,
    CustomColor(String),
}
impl CylheimThemeColor {
    #[allow(unused)]
    fn get_color(&self) -> &str {
        match self {
            CylheimThemeColor::Paff => "#FF5BC098",
            CylheimThemeColor::NekoTheElder => "#FFC481A2",
            CylheimThemeColor::Robo => "#FF84B6BF",
            CylheimThemeColor::Ivy => "#FFBA1E1E",
            CylheimThemeColor::CrystalPunk => "#FFA1505F",
            CylheimThemeColor::Vanessa => "#FF9EE0E0",
            CylheimThemeColor::Bobo => "#FF83AC36",
            CylheimThemeColor::Graffj => "#FFC79C4E",
            CylheimThemeColor::Amiya => "#FF5BC8C8",
            CylheimThemeColor::Kaf => "#FF454BDD",
            CylheimThemeColor::Alice => "#FF66494F",
            CylheimThemeColor::Hans => "#FF434343",
            CylheimThemeColor::KizunaAi => "#FFFF9EBF",
            CylheimThemeColor::Miku => "#FF03BACE",
            CylheimThemeColor::Ilka => "#FFFFFFFF",
            CylheimThemeColor::Xenon => "#FF923434",
            CylheimThemeColor::Conner => "#FFCD8145",
            CylheimThemeColor::Cherry => "#FFA1505F",
            CylheimThemeColor::Joe => "#FF644671",
            CylheimThemeColor::Sagar => "#FFB78548",
            CylheimThemeColor::Rin => "#FF83AC36",
            CylheimThemeColor::Aroma => "#FF5BC098",
            CylheimThemeColor::Nora => "#FF84B6BF",
            CylheimThemeColor::NekoTheYounger => "#FFC481A2",
            CylheimThemeColor::CustomColor(custom_color) => custom_color.as_str(),
        }
    }
}
#[allow(unused)]
pub enum CylheimDiffOption {
    EASY,
    HARD,
    CHAOS,
    GLITCH,
    CRASH,
    DREAM,
    Custom(CylheimDiffMeta),
}
#[derive(Clone)]
#[allow(unused)]
pub struct CylheimDiffMeta {
    diff_name: String,
    diff_text_color: String,
    diff_background_color: String,
}
impl CylheimDiffOption {
    #[allow(unused)]
    fn get_meta(&self) -> CylheimDiffMeta {
        match self {
            CylheimDiffOption::EASY => CylheimDiffMeta {
                diff_name: "EASY".to_string(),
                diff_text_color: "#FF003366".to_string(),
                diff_background_color: "#B333CCFF".to_string(),
            },
            CylheimDiffOption::HARD => CylheimDiffMeta {
                diff_name: "HARD".to_string(),
                diff_text_color: "#FF330000".to_string(),
                diff_background_color: "#B3FF3333".to_string(),
            },
            CylheimDiffOption::CHAOS => CylheimDiffMeta {
                diff_name: "CHAOS".to_string(),
                diff_text_color: "#FF330033".to_string(),
                diff_background_color: "#B3FF33FF".to_string(),
            },
            CylheimDiffOption::GLITCH => CylheimDiffMeta {
                diff_name: "GLITCH".to_string(),
                diff_text_color: "#FF002E1D".to_string(),
                diff_background_color: "#B300A96B".to_string(),
            },
            CylheimDiffOption::CRASH => CylheimDiffMeta {
                diff_name: "CRASH".to_string(),
                diff_text_color: "#FF452E13".to_string(),
                diff_background_color: "#B3FFC000".to_string(),
            },
            CylheimDiffOption::DREAM => CylheimDiffMeta {
                diff_name: "DREAM".to_string(),
                diff_text_color: "#FF3E3E3E".to_string(),
                diff_background_color: "#B3FFFFFF".to_string(),
            },
            CylheimDiffOption::Custom(custom_meta) => custom_meta.clone(),
        }
    }
}
