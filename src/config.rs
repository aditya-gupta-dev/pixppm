pub struct Config {
    pub fps: usize,
    pub width: usize,
    pub height: usize,
    pub workers: usize,
    pub duration: usize,
    pub out_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fps: 24,
            width: 1280,
            height: 720,
            duration: 60,
            out_dir: "out".into(),
            workers: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
}

pub fn get_config() -> Config {
    Config::default()
}
