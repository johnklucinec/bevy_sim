import cv2 as cv
import numpy as np
import time
import os
import csv
from windowcapture import WindowCapture
from line_detector import LineDetector
from yolo_detector import YOLODetector

os.chdir(os.path.dirname(os.path.abspath(__file__)))


def parse_lines(line_data, frame_shape):
    
    height, width, _ = frame_shape
    
    #default Values
    left_mid_x = -1
    left_slope = 0
    right_mid_x = -1
    right_slope = 0
    
    if line_data is None or len(line_data) == 0:
        return left_mid_x, left_slope, right_mid_x, right_slope
    
    for(x1, y1, x2, y2) in line_data:
        dx = (x2 - x1) if (x2 - x1) != 0 else 1e-6
        slope = (y2 - y1) / dx
        mid_x = (x1 + x2) / 2.0
        
        #negative slope -> left line, positive slope -> right line
        if slope < 0:
            left_mid_x = mid_x
            left_slope = slope
        else:
            right_mid_x = mid_x
            right_slope = slope
            
    return left_mid_x, left_slope, right_mid_x, right_slope

def parse_bound_boxes(bboxes, frame_shape):
    
    #returns obj count and avg center x
    
    height, width, _ = frame_shape
    
    count = len(bboxes)
    if count == 0:
        return count, -1 #-1 means no avg center
    
    centers = []
    
    for(x1, y1, x2, y2, _, _) in bboxes:
        center_x = (x1 + x2) / 2.0
        centers.append((center_x))
        
    avg_center_x = np.mean(centers)
    return count, avg_center_x


#Create a state vector with line parsing and bounding box parsing, returns list of numbers
def extract_state(frame, line_data, bboxes):
    
    left_mid_x, left_slope, right_mid_x, right_slope = parse_lines(line_data, frame.shape)
    obj_count, obj_avg_x = parse_bound_boxes(bboxes, frame.shape)
    
    state_vector = [
        left_mid_x,
        left_slope,
        right_mid_x,
        right_slope,
        obj_count,
        obj_avg_x
    ]
    
    return state_vector


# Initialize components
wincap = WindowCapture('Camera View')
line_detector = LineDetector()
yolo_detector = YOLODetector()
    

def display_fps(frame, last_time):
    fps = 1 / (time.time() - last_time)
    cv.putText(frame, f'FPS: {fps:.1f}', (10, 30),
              cv.FONT_HERSHEY_SIMPLEX, 0.7, (0,255,0), 2)
    return time.time()

def normal_display():
    loop_time = time.time()
    
    #Oepn csv file to log state data
    with open('state_data_log.csv', 'w', newline= '') as f:
        writer = csv.writer(f)
        #header
        
        writer.writerow([
            'timestamp',
            'left_mid_x', 'left_slope',
            'right_mid_x', 'right_slope',
            'obj_count', 'obj_avg_x'
        ])
        
        
        while True:
            screenshot = wincap.get_screenshot()
            
            # Process frame with both detectors
            yolo_frame, yolo_boxes = yolo_detector.process_frame(screenshot.copy())
            line_frame, line_data = line_detector.process_frame(screenshot.copy())
            
            # Simple overlay using bitwise OR
            final_frame = cv.bitwise_or(yolo_frame, line_frame)
            
            # Extract state vector
            state_vector = extract_state(screenshot, line_data, yolo_boxes)
            
            #Log into csv file
            current_time = time.time()
            writer.writerow([current_time] + state_vector)
            
            print("State Vector:", state_vector)
            
            # Display FPS and show result
            loop_time = display_fps(final_frame, loop_time)
                
            cv.imshow('Computer Vision', final_frame)
            
            #slow down frames
            time.sleep(0.2)
            
            if cv.waitKey(1) == ord('q'):
                cv.destroyAllWindows()
                break
            

def debug_display():
    while True:
        screenshot = wincap.get_screenshot()
        
        # Process frame with both detectors
        line_frame = line_detector.process_frame(screenshot.copy())
        
        cv.imshow('Computer Vision', line_frame)
        
        if cv.waitKey(1) == ord('q'):
            cv.destroyAllWindows()
            break

### MAIN ###
# Uncomment the function you want to use
normal_display()
# debug_display()
