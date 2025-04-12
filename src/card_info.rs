use crate::cards::CardInfo;

macro_rules! card_info {
    ($x:expr, $y:expr, $z:expr) => {
        CardInfo {
            name: $x,
            ack: $y,
            cost: $z,
        }
    };
}

pub fn get_card_info(num_code: &'static str) -> CardInfo {
    match num_code {
        "S001-A-001" => card_info!("APPLe".to_string(), 1200, 1),
        "NAAI-A-001" => card_info!("Vertin 维尔汀".to_string(), 0, 0),
        _ => card_info!("Unknown".to_string(), 0, 0),
    }
}
