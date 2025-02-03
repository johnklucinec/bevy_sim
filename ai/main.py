import cv2 as cv
import torch 
from ultralytics import YOLO
from windowcapture import WindowCapture
import os
from time import time

os.chdir(os.path.dirname(os.path.abspath(__file__)))

# Initialize YOLOv8 with nano model
model = YOLO('yolov8n.pt')  
# Can change to 'yolov8s.pt' for slightly better accuracy

# Enable GPU if available
if torch.cuda.is_available():
    model.to('cuda')
	print(f"GPU detected: {torch.cuda.get_device_name(0)}")
    print(f"CUDA version: {torch.version.cuda}")
else:
	print("No GPU available - using CPU")

wincap = WindowCapture('Camera View')

loop_time = time()
while True:
    screenshot = wincap.get_screenshot()
    frame_rgb = cv.cvtColor(screenshot, cv.COLOR_BGR2RGB)

    # Inference with confidence threshold
    results = model(frame_rgb, 
                   stream=True,
                   conf=0.5,  # Confidence threshold
                   verbose=False)

    for r in results:
        boxes = r.boxes.xyxy.cpu().numpy()
        classes = r.boxes.cls.cpu().numpy()
        confidences = r.boxes.conf.cpu().numpy()

        for box, cls, conf in zip(boxes, classes, confidences):
            x1, y1, x2, y2 = map(int, box)
            label = f"{model.names[int(cls)]} {conf:.2f}"
            
            cv.rectangle(screenshot, (x1, y1), (x2, y2), (0, 255, 0), 2)
            cv.putText(screenshot, label, (x1, y1-10),
                      cv.FONT_HERSHEY_SIMPLEX, 0.5, (0,255,0), 2)

    # FPS counter
    fps = 1 / (time() - loop_time)
    cv.putText(screenshot, f'FPS: {fps:.1f}', (10, 30),
              cv.FONT_HERSHEY_SIMPLEX, 0.7, (0,255,0), 2)
    loop_time = time()

    cv.imshow('Computer Vision', screenshot)
    
    if cv.waitKey(1) == ord('q'):
        cv.destroyAllWindows()
        break

print('Done.')
