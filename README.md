# Bevy Simulator

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Bevy](https://img.shields.io/badge/Bevy-232326?style=for-the-badge&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![OpenCV](https://img.shields.io/badge/OpenCV-5C3EE8?style=for-the-badge&logo=opencv&logoColor=white)
![YOLOv8](https://img.shields.io/badge/YOLOv8-00FFFF?style=for-the-badge&logo=yolo&logoColor=white)
![Status](https://img.shields.io/badge/Status-In_Development-yellow?style=for-the-badge)


A self-driving car simulation project that combines game development tools with computer vision and AI techniques. Built with Bevy Engine and enhanced with Python-based image recognition.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributors](#contributors)
- [License](#license)


## Overview

Bevy Simulator is an engineering simulation project that demonstrates self-driving capabilities using game development tools. The project creates a virtual environment where a vehicle can be controlled manually or autonomously through computer vision techniques.

The simulation uses the Bevy game engine to create the physics and environment, while Python scripts handle image recognition through OpenCV and YOLOv8. The system detects road edges and uses PID controllers to keep the vehicle on track.

## Features

- **Interactive Simulation**: Drive a vehicle manually using keyboard controls
- **Environmental Generation**: Procedurally generated roads and obstacles
- **Computer Vision Integration**: Real-time image processing with OpenCV
- **Self-Driving Capabilities**: PID-based autonomous driving using edge detection
- **Dual Window System**: Main game window and secondary vision processing window
- **Interprocess Communication**: Bidirectional data exchange between Rust and Python


## Technologies

### Game Engine \& Physics

- **Bevy Engine**: Modern data-driven game engine written in Rust
- **Rust**: Systems programming language that guarantees memory safety


### Computer Vision \& AI

- **Python**: Scripting language for computer vision implementation
- **OpenCV**: Open source computer vision and machine learning library
- **YOLOv8**: Real-time object detection system


### Communication

- **Interprocess Communication**: Custom protocol for Rust-Python communication


## Getting Started

### Prerequisites

*Coming soon*

### Installation

*Coming soon*

## Usage

### Manual Control

- **Arrow Keys**: Drive the vehicle (up/down for acceleration/braking, left/right for steering)
- **G Key**: Change gears

### Autonomous Mode

- **Tab Key**: Toggle the secondary window for image recognition visualization (automatically starts autonomous driving)


## Project Structure

- 📁 **bevy_sim**
  - 📁 [ai 🔗](./ai)
  - 📁 assets
  - 📁 src
    - 📁 game
      - 📁 biome
      - 📁 car
      - 📁 camera
      - 📁 python
      - 📁 ui
    - 📁 [main_menu 🔗](src/main_menu)
  - 📄 main.rs
  - 📄 system.rs

> 📌 **Note:** Folders with 🔗 links contain their own README files with more detailed documentation


## Contributors

| [<img src="https://avatars.githubusercontent.com/u/72411904?v=4" width="100" alt="John Klucinec">](https://github.com/johnklucinec) | [<img src="https://avatars.githubusercontent.com/u/94635676?v=4" width="100" alt="Brant Cass">](https://github.com/brantcass) | [<img src="https://avatars.githubusercontent.com/u/101136913?s=64&v=4" width="100" alt="Ramiro Covarrubias">](https://github.com/Roxamir) | [<img src="https://avatars.githubusercontent.com/u/100557101?s=64&v=4" width="100" alt="Alex Collins">](https://github.com/acolli33) |
|------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| **[John Klucinec](https://github.com/johnklucinec)**<br>Game States • UI • IPC • Image Recognition                                  | **[Brant Cass](https://github.com/brantcass)**<br>World Generation • PID Controls                                           | **[Ramiro Covarrubias](https://github.com/Roxamir)**<br>Car Physics • PID Controls                                                      | **[Alex Collins](https://github.com/acolli33)**<br>Initial Planning • Documentation                                                  |

## License

*Coming soon*

---

*This project was created as part of the CS 462 course, Spring 2025.*