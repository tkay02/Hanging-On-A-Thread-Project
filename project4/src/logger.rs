use std::io::{BufWriter,Write,Error};
use std::fs::{File, OpenOptions};
use std::process;


pub struct Logger {
    is_writable: bool,
    file_writer: BufWriter<File>
}

impl Logger {

    pub fn new(file_name:String, write_to_file:bool) -> Result<Logger, Error> {
        let output = OpenOptions::new()
            .create(true)
            .write(true)
            //.truncate(true)
            .open(file_name)?;
        let file_writer = BufWriter::new(output);
        Ok( Logger {is_writable: write_to_file, file_writer} )
    }

    pub fn write(&mut self, message:String) {
        if self.is_writable {
            let result = self.file_writer.write(message.as_bytes());
            if result.is_err() {
                println!("An error has occurred when writing to a file");
                process::exit(1);
            }
        } else {
            println!("{}", message);
        }
    }

}