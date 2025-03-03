import cv2 as cv
import numpy as np
import time
import os
import csv
from windowcapture import WindowCapture
from line_detector import LineDetector
from yolo_detector import YOLODetector
from command_handler import CommandHandler  # Add this line to import CommandHandler

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
	fps = 1 / (time() - last_time)
	cv.putText(frame, f'FPS: {fps:.1f}', (10, 30),
			  cv.FONT_HERSHEY_SIMPLEX, 0.7, (0,255,0), 2)
	return time()

def normal_display(wincap, yolo_detector, line_detector):
    command_handler = CommandHandler()
    loop_time = time()
    
    try:
        while True:
            screenshot = wincap.get_screenshot()
            
            # Process frame with both detectors
            yolo_frame = yolo_detector.process_frame(screenshot.copy())
            line_frame = line_detector.process_frame(screenshot.copy())
            
            # Simple overlay using bitwise OR
            final_frame = cv.bitwise_or(yolo_frame, line_frame)
            
            # Display FPS and show result
            loop_time = display_fps(final_frame, loop_time)
            cv.imshow('Computer Vision', final_frame)
            
            # Check for commands
            while not command_handler.command_queue.empty():
                command = command_handler.command_queue.get()
                # add specific frame processing based on commands here
                
            if cv.waitKey(1) == ord('q'):
                cv.destroyAllWindows()
                break
                
    finally:
        command_handler.stop()

def debug_display(): # NEED TO ADD COMMAND HANDLER
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
normal_display(wincap, yolo_detector, line_detector)
# debug_display()





# def handle_reset_car(self, value: str | None = None) -> str:
#         """Reset the car to a default safe position, e.g., start line."""
#         # might have to change position, but added this to make it easy to reset, maybe can
#         # change it so it will use this when it reaches a certain distance or time.
#         self.car_position = (0.0, 0.0)
#         self.current_speed = 0.0
#         return "Car position and speed have been reset."
    
    
# def handle_out_of_bounds(self, value: str | None = None) -> str:

#         # handle car going out of bounds.
#         # - Stop the car
#         # - Reset car position
#         # - Decrement a reward metric
        
#         self.current_speed = 0.0
#         self.car_position = (0.0, 0.0)
#         return "Car went out of bounds! Resetting position and stopping."