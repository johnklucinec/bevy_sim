/// Author: John Klucinec (@johnklucinec)
use super::components::{CommandQueue, PythonComms};
use crate::game::python::{commands::CommandType, components::CommandEvent};

use bevy::prelude::*;
use crossbeam_channel::Sender;
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    thread,
};

// Calls the python script and stores the child process, stdin, and stdout
// This allows us to read and send commands
pub fn spawn_python_child() -> (Child, ChildStdin, ChildStdout) {
    let mut cmd = Command::new("python")
        .arg("-u")
        .arg("./ai/main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Python");

    let stdin = cmd.stdin.take().unwrap();
    let stdout = cmd.stdout.take().unwrap();

    (cmd, stdin, stdout)
}

// This function sets up a thread to read from the Python script's stdout and send messages to the main thread
// This is done in a non-blocking way
pub fn setup_io_threads(tx: Sender<String>, stdout: ChildStdout) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        while let Ok(n) = reader.read_line(&mut line) {
            if n == 0 {
                break;
            } // EOF
            let msg = line.trim().to_string();
            tx.send(msg).expect("Failed to send message");
            line.clear();
        }
    });
}

pub fn process_command_queue(mut comms: ResMut<PythonComms>, mut commands: ResMut<CommandQueue>) {
    // Process all queued commands
    while let Some(cmd) = commands.dequeue() {
        // Format the command as a string and send it to Python
        let cmd_str = cmd.to_string();

        // Send to Python via stdin
        if let Err(e) = writeln!(comms.stdin, "{}", cmd_str) {
            eprintln!("Failed to write to Python stdin: {}", e);
        }

        // Flush to ensure command is sent immediately
        if let Err(e) = comms.stdin.flush() {
            eprintln!("Failed to flush Python stdin: {}", e);
        }
    }
}

pub fn handle_responses(comms: Res<PythonComms>, mut command_events: EventWriter<CommandEvent>) {
    // Process all available messages without blocking
    for msg in comms.rx.try_iter() {
        println!("Python output received: '{}'", msg);

        // Extract the first word (potential command)
        if let Some(first_word) = msg.split_whitespace().next() {
            // Remove any colon if present (e.g., "DETECT:")
            let command_str = first_word.trim_end_matches(':');

            // Try to parse the command type
            if let Ok(cmd_type) = command_str.parse::<CommandType>() {
                // Get everything after the command as the value/message
                let value_str = msg[command_str.len()..].trim_start_matches(':').trim();

                // Try to parse as float if it looks like a number
                let value = value_str.parse::<f32>().ok();

                // Create and send the structured command event
                command_events.send(CommandEvent {
                    command_type: cmd_type,
                    value,
                    string_value: value_str.to_string(),
                });

                // For Debugging
                // println!(
                //     "Bevy output: {:?} command processed with value: {:?}",
                //     cmd_type, value
                // );
            }
            // If not a recognized command, do nothing (no error message)
        }
    }
}
