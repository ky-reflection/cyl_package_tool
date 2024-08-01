use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn get_color(&self) -> &str {
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
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
pub enum CylheimDiffOption {
    Easy,
    Hard,
    Chaos,
    Glitch,
    Crash,
    Dream,
    Custom(CylheimDiffMeta),
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[allow(unused)]
pub struct CylheimDiffMeta {
    pub diff_name: String,
    pub diff_background_color: String,
    pub diff_text_color: String,
}
impl CylheimDiffOption {
    #[allow(unused)]
    pub fn get_meta(&self) -> CylheimDiffMeta {
        match self {
            CylheimDiffOption::Easy => CylheimDiffMeta {
                diff_name: "EASY".to_string(),
                diff_background_color: "#FF003366".to_string(),
                diff_text_color: "#B333CCFF".to_string(),
            },
            CylheimDiffOption::Hard => CylheimDiffMeta {
                diff_name: "HARD".to_string(),
                diff_background_color: "#FF330000".to_string(),
                diff_text_color: "#B3FF3333".to_string(),
            },
            CylheimDiffOption::Chaos => CylheimDiffMeta {
                diff_name: "CHAOS".to_string(),
                diff_background_color: "#FF330033".to_string(),
                diff_text_color: "#B3FF33FF".to_string(),
            },
            CylheimDiffOption::Glitch => CylheimDiffMeta {
                diff_name: "GLITCH".to_string(),
                diff_background_color: "#FF002E1D".to_string(),
                diff_text_color: "#B300A96B".to_string(),
            },
            CylheimDiffOption::Crash => CylheimDiffMeta {
                diff_name: "CRASH".to_string(),
                diff_background_color: "#FF452E13".to_string(),
                diff_text_color: "#B3FFC000".to_string(),
            },
            CylheimDiffOption::Dream => CylheimDiffMeta {
                diff_name: "DREAM".to_string(),
                diff_background_color: "#FF3E3E3E".to_string(),
                diff_text_color: "#B3FFFFFF".to_string(),
            },
            CylheimDiffOption::Custom(custom_meta) => custom_meta.clone(),
        }
    }
}
#[allow(unused)]
pub enum CylheimThemeIcon {
    Paff,
    NekoTheElder,
    Robo,
    Ivy,
    CrystalPunk,
    Vanessa,
    VanessaS,
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
impl CylheimThemeIcon {
    #[allow(unused)]
    pub fn get_icon_path(&self) -> &str {
        match self {
            CylheimThemeIcon::Paff => "cyl://cyicons/paff001",
            CylheimThemeIcon::NekoTheElder => "cyl://cyicons/neko001",
            CylheimThemeIcon::Robo => "cyl://cyicons/robo001",
            CylheimThemeIcon::Ivy => "cyl://cyicons/ivy001",
            CylheimThemeIcon::CrystalPunk => "cyl://cyicons/cherry002",
            CylheimThemeIcon::Vanessa => "cyl://cyicons/vanessa001",
            CylheimThemeIcon::VanessaS => "cyl://cyicons/vanessa001s",
            CylheimThemeIcon::Bobo => "cyl://cyicons/bobo001",
            CylheimThemeIcon::Graffj => "cyl://cyicons/graffj001",
            CylheimThemeIcon::Amiya => "cyl://cyicons/amiya001",
            CylheimThemeIcon::Kaf => "cyl://cyicons/kaf001",
            CylheimThemeIcon::Alice => "cyl://cyicons/alice001",
            CylheimThemeIcon::Hans => "cyl://cyicons/hans001",
            CylheimThemeIcon::KizunaAi => "cyl://cyicons/ai001",
            CylheimThemeIcon::Miku => "cyl://cyicons/miku001",
            CylheimThemeIcon::Ilka => "cyl://cyicons/ilka001",
            CylheimThemeIcon::Xenon => "cyl://cyicons/xenon001",
            CylheimThemeIcon::Conner => "cyl://cyicons/conner001",
            CylheimThemeIcon::Cherry => "cyl://cyicons/cherry001",
            CylheimThemeIcon::Joe => "cyl://cyicons/joe001",
            CylheimThemeIcon::Sagar => "cyl://cyicons/sagar001",
            CylheimThemeIcon::Rin => "cyl://cyicons/rin001",
            CylheimThemeIcon::Aroma => "cyl://cyicons/paff002",
            CylheimThemeIcon::Nora => "cyl://cyicons/robo002",
            CylheimThemeIcon::NekoTheYounger => "cyl://cyicons/neko002",
            CylheimThemeIcon::CustomColor(custom_path) => custom_path.as_str(),
        }
    }
}
