# Python Integration for Bevy

<table>
  <tr>
    <td>
      <img src="https://avatars.githubusercontent.com/u/72411904?v=4" alt="John Klucinec" width="100">
    </td>
    <td>
      <strong>Author:</strong>
      <a href="https://github.com/johnklucinec">John Klucinec</a>
    </td>
  </tr>
</table>

---

## Directory Structure

- 📁 **python**
    - 📄 **commands.rs** - *Command definitions and keyboard input handling*
    - 📄 **components.rs** - *Data structures and resources for Python communication*
    - 📄 **mod.rs** - *Plugin definition and system registration*
    - 📄 **systems.rs** - *Python process management and I/O handling*

---

## Overview

This module provides seamless integration between Bevy and Python scripts using asynchronous IPC (Inter-Process Communication) channels. It allows your Bevy application to spawn Python processes, send commands, and receive responses in a non-blocking manner.

## Command Types

The module supports several command types for communicating with Python:


| Command | Description |
| :-- | :-- |
| `Detect` | Example Command (Not Implemented) |
| `Reset` |  Example Command (Not Implemented) |
| `Steer` | Sends steering commands to Python |
| `Speed` | Sends speed-related commands to Python |
| `Pidreset` | Resets PID controllers in Python |

## Components

| Component | Description |
| :-- | :-- |
| `CommandEvent` | Event triggered when receiving responses from Python |
| `PythonComms` | Resource holding the Python process and communication channels |
| `CommandQueue` | Queue for pending commands to be sent to Python |
| `CommandMessage` | Structure representing a command with its type and payload |

## Usage in a Bevy Project

### Setup

1. Add the plugin to your Bevy app:
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PythonPlugin)
        // Initialize Python communication in your startup system
        .add_systems(Startup, initialize_python)
        .run();
}

fn initialize_python(mut commands: Commands) {
    // Spawn Python process
    let (child, stdin, stdout) = spawn_python_child();

    // Create communication channels
    let (tx, rx) = crossbeam_channel::unbounded();

    // Setup I/O threads
    setup_io_threads(tx.clone(), stdout);

    // Insert resource
    commands.insert_resource(PythonComms {
        child,
        stdin,
        tx,
        rx,
    });
}
```


### Sending Commands

```rust
// Send a command manually
fn my_game_system(mut command_queue: ResMut<CommandQueue>) {
    // Queue a detection command
    command_queue.enqueue(CommandMessage::new(CommandType::Detect, "traffic cone"));
}
```


### Handling Responses

```rust
fn process_python_responses(mut events: EventReader<CommandEvent>) {
    for event in events.iter() {
        match event.command_type {
            CommandType::Detect => {
                println!("Detection result: {}", event.string_value);
                // Handle detection result
            },
            CommandType::Steer => {
                if let Some(angle) = event.value {
                    // Apply steering angle
                    println!("Steering angle: {}", angle);
                }
            },
            // Handle other command types
            _ => {}
        }
    }
}
```


## Key Benefits

**Asynchronous Communication**: The module uses non-blocking I/O through crossbeam channels, allowing Python processing to run concurrently without affecting the Bevy game loop.

**Pipe-based IPC**: Communication between Rust and Python uses standard input/output pipes, providing a simple yet effective way to exchange data between processes.

**Structured Command System**: The command system provides a type-safe way to send instructions to Python with proper serialization.

**Event-Based Architecture**: Python responses are converted to Bevy events, integrating seamlessly with Bevy's ECS architecture.

**Graceful Error Handling**: The system includes error handling for process spawning and communication failures.

## Advanced Features

- **On-demand Process Management**: The Python process is only started when needed and can be restarted if it crashes.
- **Buffered Communication**: Commands are queued and processed sequentially to maintain order.
- **Automatic Command Parsing**: Python responses are automatically parsed into strongly-typed Bevy events.

*Part of the Bevy Simulator project.*
