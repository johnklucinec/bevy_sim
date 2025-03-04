use super::components::{CommandQueue, PythonComms};
use crate::game::python::components;

use crate::CarInput;
use bevy::prelude::*;
use crossbeam_channel::Sender;
use std::{
    io::{BufRead, BufReader, Write}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, thread
};

// Calls the python script and stores the child process, stdin, and stdout
// This allows us to read and send commands
pub fn spawn_python_child() -> (Child, ChildStdin, ChildStdout) {
    let mut cmd = Command::new("python")
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

// This reads everything sent from the python terminal
pub fn handle_responses(
    comms: Res<PythonComms>, 
    mut events: EventWriter<components::PythonEvent>,
    mut car_input: ResMut<CarInput>
) {
    // Process all available messages without blocking
    for msg in comms.rx.try_iter() {
        println!("Python output received: '{}'", msg);

        // Check for car control commands
        let parts: Vec<&str> = msg.trim().split_whitespace().collect();
        // debugging print
        println!("Command parts: {:?}", parts);
        
        match parts.as_slice() {
            ["STEER", value_str] => {
                // Pass the entire command to be parsed in CarInput
                car_input.text_command = Some(format!("steer {}", value_str));
                println!("Bevy output: STEER {} command processed successfully", value_str);
            },
            ["SPEED", value_str] => {
                // Pass the entire command to be parsed in CarInput
                car_input.text_command = Some(format!("speed {}", value_str));
                println!("Bevy output: SPEED {} command processed successfully", value_str);
            },
            _ => {
                println!("Bevy output: Unrecognized command format: {:?}", parts);
            }
        }
        // Send the message as an event for any other systems that might need it
        events.send(components::PythonEvent(msg));
    }
}
