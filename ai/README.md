# Bevy Simulation AI

## Installation

To get started with the Bevy Simulation AI, you need to install the following dependencies:

```sh
pip install opencv-python
pip install pyautogui
pip install Pillow
pip install pywin32
pip install torch
pip install ultralytics
```

Install this if you want to have (NVIDIA) GPU accel on the Image Recognition:

https://developer.nvidia.com/cuda-11-8-0-download-archive

After that you need to reinstall pytorch with CUDA support
```sh
pip uninstall torch torchvision torchaudio
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
```

## Running the Capture

To run the capture, execute the following command:

```sh
python main.py
```

To stop the capture, press 'q'. Eventually this will be done automatically. 

