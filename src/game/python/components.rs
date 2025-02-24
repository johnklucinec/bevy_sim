use super::commands::CommandType;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use std::fmt;
use std::{
    collections::VecDeque,
    process::{Child, ChildStdin},
};

#[derive(Event)]
#[allow(dead_code)]
pub struct PythonEvent(pub String);

#[derive(Resource)]
#[allow(dead_code)]
pub struct PythonComms {
    pub child: Child,
    pub stdin: ChildStdin,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Default, Resource)]
pub struct CommandQueue {
    queue: VecDeque<CommandMessage>,
}

#[allow(dead_code)]
impl CommandQueue {
    pub fn enqueue(&mut self, command: CommandMessage) {
        self.queue.push_back(command);
    }

    pub fn dequeue(&mut self) -> Option<CommandMessage> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub struct CommandMessage {
    command_type: CommandType,
    payload: String,
}

impl fmt::Display for CommandMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.command_type.to_string(), self.payload)
    }
}

impl CommandMessage {
    pub fn new(command_type: CommandType, payload: impl Into<String>) -> Self {
        Self {
            command_type,
            payload: payload.into(),
        }
    }
}
