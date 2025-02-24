import cv2 as cv
import torch
from ultralytics import YOLO

class YOLODetector:
    def __init__(self, model_path='yolov8n.pt', conf_threshold=0.5):
        self.model = YOLO(model_path)
        self.conf_threshold = conf_threshold
        self.device = 'cuda' if torch.cuda.is_available() else 'cpu'
        
        if self.device == 'cuda':
            self.model.to('cuda')
            print(f"Using GPU: {torch.cuda.get_device_name(0)}")
        else:
            print("Using CPU")

    def process_frame(self, frame):
        results = self.model(
            frame, 
            stream=True,
            conf=self.conf_threshold,
            verbose=False
        )
        
        #store bounding box data in list
        bounding_box = []
        
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
                
                bounding_box.append((x1, y1, x2, y2, int(cls), float(conf)))
        return frame, bounding_box
