use super::components::{CommandQueue, PythonComms};
use crate::game::python::components;

use crate::CarInput;
use bevy::prelude::*;
use crossbeam_channel::Sender;
use std::{
    io::{BufRead, BufReader, Write}, os::windows::process, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, thread
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

// Systems now conditionally run based on PythonComms existence

// // Used to send commands to the python script
// // Currently it bases things off keypresses, but that was for proof of conecept
// pub fn send_commands(comms: ResMut<PythonComms>, input: Res<ButtonInput<KeyCode>>) {
//     get_commands(comms, input);
// }

// pub fn send_command(mut comms: ResMut<PythonComms>, command: &CommandMessage) {
//     let command = CommandMessage::new(CommandType::Reset, String::new());
//     writeln!(comms.stdin, "{}", command).unwrap();
// }

pub fn process_command_queue(mut comms: ResMut<PythonComms>, mut commands: ResMut<CommandQueue>) {
    while let Some(command) = commands.dequeue() {
        writeln!(comms.stdin, "{}", command).unwrap();
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
        println!("Python output: {}", msg);

        // Check for car control commands
        match msg.to_lowercase().trim() {
            "go" | "stop" | "left" | "right" | "gear" => {
                car_input.text_command = Some(msg.to_lowercase());
                println!("Bevy output: {} command processed", msg);
            },
            _ => {
                
            }
        }
        // I think we want to use this to read messages at some point.
        events.send(components::PythonEvent(msg));
    }
}
