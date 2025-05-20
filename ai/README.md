# Bevy Simulation AI

## Installation

To get started with the Bevy Simulation AI, follow these steps:

### 1. Install Python Dependencies

```sh
pip install opencv-python pyautogui Pillow pywin32 torch ultralytics
```


### 2. Install CUDA for GPU Acceleration

> ⚠️ **IMPORTANT**: You **MUST** use CUDA 11.8.0 specifically for compatibility!

Download CUDA 11.8.0 from the official NVIDIA archive:
[CUDA 11.8.0 Download](https://developer.nvidia.com/cuda-11-8-0-download-archive)

After installing CUDA, reinstall PyTorch with CUDA support:

```sh
pip uninstall torch torchvision torchaudio
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
```


## Troubleshooting

**Window Update Issues**: If the secondary window doesn't update in your Bevy project, try setting the Vulkan/OpenGL present method to "native" in your graphics settings (NVIDIA control panel).
