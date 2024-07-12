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
