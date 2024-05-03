use std::io::{BufWriter,Write,Error};
use std::fs::{File,OpenOptions};


pub struct Logger {
    file_writer: Option<BufWriter<File>>
}

impl Logger {

    pub fn new(file_name:String, write_to_file:bool) -> Result<Logger, Error> {
        let mut writer:Option<BufWriter<File>>;
        if write_to_file {
            let output = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file_name)?;
            writer = Some(BufWriter::new(output))
        } else {
            writer = None;
        }
        Ok(Logger { file_writer: writer })
    }

    pub fn write(&self, message:String) {
        if self.file_writer.is_none() {
            println!("{}", message);
        } else {
            let mut writer = self.file_writer.unwrap();
            writer.write(message.as_bytes());
        }
    }

}