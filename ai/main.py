import cv2 as cv
from time import time
import os
from windowcapture import WindowCapture
from line_detector import LineDetector
from yolo_detector import YOLODetector
from command_handler import CommandHandler, CommandType
from pid_controller import PIDController

os.chdir(os.path.dirname(os.path.abspath(__file__)))

# Initialize the window capture object
wincap = WindowCapture('Camera View')

line_detector = LineDetector()
yolo_detector = YOLODetector()


def display_fps(frame, last_time):
    """
    Display the FPS on the frame.
    """

    fps = 1 / (time() - last_time)
    cv.putText(frame, f'FPS: {fps:.1f}', (10, 30),
               cv.FONT_HERSHEY_SIMPLEX, 0.7, (0, 255, 0), 2)
    return time()


def normal_display(wincap, yolo_detector, line_detector):
    """
    Main display loop.
    """
    command_handler = CommandHandler()

    # kp (Proportional gain)- Higher kp system will respond more aggressively to error
    # ki (Integral gain) – Addresses accumulated error over time
    # kd (Derivative gain) – Reacts to the rate of change of the error
    
    pid = PIDController(kp = 0.2, ki = 0.0, kd = 0.1, setpoint = 250.0)
    loop_time = time()
    last_time = time()
    try:
        while True:
            screenshot = wincap.get_screenshot()
            
            #Get time since last loop
            current_time = time()
            dt = current_time - last_time
            last_time = current_time
            

            # Process frame with both detectors
            yolo_frame = yolo_detector.process_frame(screenshot.copy())

            line_frame, center_x = line_detector.process_frame(screenshot.copy())
            
            # Simple overlay using bitwise OR
            final_frame = cv.bitwise_or(yolo_frame, line_frame)
            
            #if valid center_x compute PID
            if center_x is not None:
                raw_pid = pid.update(center_x, dt)
                scaling_factor = 0.02  # Experiment: try lowering from 0.05 or 0.03
                scaled_steering = raw_pid * scaling_factor
                #Send steering signal to command handler
                val = command_handler._execute_handler(CommandType.STEER, str(scaled_steering))
                print(val)
                cv.putText(final_frame, f'Steer: {scaled_steering:.2f}', (10, 60),
                          cv.FONT_HERSHEY_SIMPLEX, 0.7, (0,255,0), 2)

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


def debug_display():    # NEED TO ADD COMMAND HANDLER
    """
    Debug display loop.
    """

    while True:
        screenshot = wincap.get_screenshot()

        # Process frame with both detectors
        line_frame = line_detector.process_frame(screenshot.copy())

        cv.imshow('Computer Vision', line_frame)

        if cv.waitKey(1) == ord('q'):
            cv.destroyAllWindows()
            break


# Uncomment the function you want to use
normal_display(wincap, yolo_detector, line_detector)
# debug_display()

