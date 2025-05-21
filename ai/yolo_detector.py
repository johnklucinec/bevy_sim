# Author: John Klucinec (@johnklucinec)
import cv2 as cv
import torch
from ultralytics import YOLO

class YOLODetector:
    def __init__(self, model_path='safety_cone.pt', conf_threshold=0.7):
        self.model = YOLO(model_path)
        self.conf_threshold = conf_threshold
        self.device = 'cuda' if torch.cuda.is_available() else 'cpu'
        
        if self.device == 'cuda':
            self.model.to('cuda')
            print(f"Using GPU: {torch.cuda.get_device_name(0)}", flush=True)
        else:
            print("Using CPU", flush=True)

    def process_frame(self, frame):
        cone_center = None
        results = self.model(
            frame, 
            stream=True,
            conf=self.conf_threshold,
            verbose=False
        )
        
        for r in results:
            boxes = r.boxes.xyxy.cpu().numpy()
            classes = r.boxes.cls.cpu().numpy()
            confidences = r.boxes.conf.cpu().numpy()
            
            for box, cls, conf in zip(boxes, classes, confidences):
                x1, y1, x2, y2 = map(int, box)
                label = f"{self.model.names[int(cls)]} {conf:.2f}"
                
                cv.rectangle(frame, (x1, y1), (x2, y2), (0, 255, 0), 2)
                cv.putText(frame, label, (x1, y1-10),
                          cv.FONT_HERSHEY_SIMPLEX, 0.5, (0,255,0), 2)
                cone_center = (x1 + (x2 - x1) // 2, y2)
                cv.circle(frame, cone_center, 3, (0, 0, 255), -1)
                

        return frame, cone_center
