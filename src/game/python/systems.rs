use super::components::PythonComms;
use crate::game::python::components;
use bevy::prelude::*;
use crossbeam_channel::Sender;
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    thread,
};

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
pub fn send_commands(mut comms: ResMut<PythonComms>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        writeln!(comms.stdin, "DETECT").unwrap();

        // You can use this to print the command to the console
        // comms.tx.send("DETECT".to_string()).unwrap();
    }

    if input.just_pressed(KeyCode::KeyR) {
        writeln!(comms.stdin, "RESET").unwrap();
    }
}

pub fn handle_responses(comms: Res<PythonComms>, mut events: EventWriter<components::PythonEvent>) {
    // Process all available messages without blocking
    for msg in comms.rx.try_iter() {
        println!("Python output: {}", msg);

        if msg.contains("Starting") {
            println!("Bevy output: Recieved the word 'Starting'");
        }
        events.send(components::PythonEvent(msg));
    }
}
