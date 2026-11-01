pub struct Config {
    pub width: usize,
    pub height: usize,
    pub buf_size: usize,
    pub pixel_size: usize,
    pub filename: String,
    pub out_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 640,
            height: 480,
            filename: "img.ppm".into(),
            pixel_size: 20,
            buf_size: 8 * 1024,
            out_dir: "out".into(),
        }
    }
}

pub fn get_config() -> Config {
    Config::default()
}
