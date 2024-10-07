use configparser::ini::Ini;

#[derive(Debug)]
pub struct Commands {
    pub topleft: Option<String>,
    pub topright: Option<String>,
    pub bottomright: Option<String>,
    pub bottomleft: Option<String>,
    pub left: Option<String>,
    pub top: Option<String>,
    pub right: Option<String>,
    pub bottom: Option<String>,
}

pub fn load_commands() -> Commands {
    let mut cfg = Ini::new();
    let mut path = dirs::config_dir().unwrap();
    path.push("GXDE/gxde-corner-zone/config.conf");
    if let Err(err) = cfg.load(path) {
        panic!("{}", err);
    }

    Commands {
        topleft: cfg.get("commands", "topleft"),
        topright: cfg.get("commands", "topright"),
        bottomright: cfg.get("commands", "bottomright"),
        bottomleft: cfg.get("commands", "bottomleft"),
        left: cfg.get("commands", "left"),
        top: cfg.get("commands", "top"),
        right: cfg.get("commands", "right"),
        bottom: cfg.get("commands", "bottom"),
    }
}
