import cv2 as cv
import numpy as np
from time import time
import os
from windowcapture import WindowCapture
from line_detector import LineDetector
from yolo_detector import YOLODetector
from command_handler import CommandHandler
import threading

os.chdir(os.path.dirname(os.path.abspath(__file__)))

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
            
            # Capture keyboard input for car commands
            key = cv.waitKey(1) & 0xFF
            if key == ord('q'):
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
