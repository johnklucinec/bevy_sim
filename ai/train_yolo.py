from ultralytics import YOLO
import os

# Load model
model = YOLO('yolov8n.pt')

# Define the path to data.yaml file, relative to this script's location.
script_dir = os.path.dirname(os.path.abspath(__file__))
data_yaml_path = os.path.join(script_dir, 'datasets', 'data.yaml')

# Ensure the path is absolute before passing to YOLO
if not os.path.isabs(data_yaml_path):
    data_yaml_path = os.path.abspath(data_yaml_path)

# Train the model
if __name__ == '__main__':
    try:
        results = model.train(
            data=data_yaml_path, 
            epochs=300,  # Number of training epochs (300 reccomended starting point per Ultralytics)
            imgsz=640,
            batch=32,    # Batch size (adjust based on your GPU memory, 32 uses about 6GB of VRAM)
            name='yolov8n_safety_cones_custom' # Name for the training run, results saved in 'runs/detect/yolov8n_safety_cones_custom'
        )
        print("Training completed. Results saved to: ", results.save_dir)
        print("Best model saved at: ", results.best)
    except Exception as e:
        print(f"An error occurred during training: {e}")
        print("Please ensure your dataset is correctly formatted and paths in data.yaml are valid.")
        print("Consider starting with a smaller number of epochs or batch size if you encounter memory issues.")


