use super::Config;
use serde_json::Value;
use std::fs;
use std::io::prelude::*;

use std::io::{BufWriter, Error};

use super::model::EnvVariable;

pub struct App<'a> {
    pub config: &'a Config<'a>,
    pub contents: Vec<EnvVariable>,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config<'a>) -> App<'a> {
        App {
            config,
            contents: Vec::new(),
        }
    }

    fn load(&mut self, reader: &mut dyn std::io::Read) -> Result<(), Error> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        self.contents = serde_json::from_str(&contents).unwrap();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut file = fs::File::open(&self.config.filename)?;
        self.load(&mut file).unwrap();
        self.write(&mut std::io::stdout())?;

        Ok(())
    }

    fn write(&self, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        let mut writer = BufWriter::new(writer);

        for env_variable in &self.contents {
            let line = env_variable.to_kv();
            writer.write(line.as_bytes())?;
            writer.write(b"\n")?;
        }
        writer.flush()?;

        Ok(())
    }
}
