use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use std::process::{Child, ChildStdin};

#[derive(Event)]
pub struct PythonEvent(pub String);

#[derive(Resource)]
pub struct PythonComms {
    pub child: Child,
    pub stdin: ChildStdin,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}
