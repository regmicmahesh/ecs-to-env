pub struct Config<'a> {
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(filename: &'a str) -> Self {
        Config { filename }
    }
}
