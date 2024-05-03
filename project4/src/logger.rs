//! # Hanging on by a Thread: logger.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! This module provides conditional logging to either a file or standard output based on
//! command line arguments.
//! The `Logger` structure within this module is what allows for messages to be logged either to a
//! specified file or to the console.
//!
//! ## Dependencies
//! This module relies on several components from the Rust standard library:
//! - `std::io::{BufWriter, Write, Error}` for handling buffered writing operations,
//!   which improve performance when writing to files by reducing the number of write operations.
//! - `std::fs::{File, OpenOptions}` for file management, allowing the logger to create,
//!   open, and modify files as needed.
//! - `std::process` for handling critical failures during logging, such as exiting the
//!   application when a file write fails, ensuring that file writing errors are not silently ignored.
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::io::{BufWriter,Write,Error};
use std::fs::{File,OpenOptions};
use std::process;

/// Represents a simple logger with the capability to write messages to a file
/// or standard output.
///
/// # Fields
/// - `file_writer`: An optional `BufWriter<File>` used for writing messages to a file
///   when enabled. If `None`, messages will be printed to standard output.
pub struct Logger {
    file_writer: Option<BufWriter<File>>
}

impl Logger {
    /// Creates a new `Logger` instance, optionally writing to a file.
    ///
    /// # Parameters
    /// - `file_name`: The path to the file where logs should be written.
    /// - `write_to_file`: A boolean flag that, if true, initializes file writing.
    ///
    /// # Returns
    /// - A `Result<Logger, Error>` which is `Ok` containing the `Logger` if file operations succeed,
    ///   or an `Err` with an `Error` if there is a problem opening or creating the file.
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

    /// Writes a message to the configured output destination.
    ///
    /// If a file writer is set up, the message will be written to the file. If writing fails,
    /// the process will exit with an error status. If no file writer is present, the message
    /// will be printed to standard output.
    ///
    /// # Parameters
    /// - `message`: The string message to log.
    ///
    /// # Panics
    /// - The function will exit the process if it fails to write to the file.
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