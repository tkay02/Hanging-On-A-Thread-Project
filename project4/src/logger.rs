use std::io::{BufWriter,Write,Error};
use std::fs::{File,OpenOptions};
use std::process;


pub struct Logger {
    file_writer: Option<BufWriter<File>>
}

impl Logger {

    pub fn new(file_name:String, write_to_file:bool) -> Result<Logger, Error> {
        let writer:Option<BufWriter<File>>;
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

    pub fn write(&mut self, message:String) {
        if let Some(writer) = self.file_writer.as_mut() {
            let result = writer.write(message.as_bytes());
            if result.is_err() {
                println!("An error has occurred when writing to a file");
                process::exit(1);
            }
        } else {
            println!("{}", message);
        }
    }

}