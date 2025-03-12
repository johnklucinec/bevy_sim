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
    loop_time = time()

    # kp (Proportional gain)- Higher kp system will respond more aggressively to error
    # ki (Integral gain) – Addresses accumulated error over time
    # kd (Derivative gain) – Reacts to the rate of change of the error
    
    pid = PIDController(kp = 3.0, ki = 0.0, kd = 0.1, setpoint = 250.0)
    loop_time = time()

    try:
        while True:
            screenshot = wincap.get_screenshot()

            # Process frame with both detectors
            yolo_frame = yolo_detector.process_frame(screenshot.copy())

            line_frame, center_x = line_detector.process_frame(screenshot.copy())
            
            # Simple overlay using bitwise OR
            final_frame = cv.bitwise_or(yolo_frame, line_frame)
            
            #if valid center_x compute PID
            if center_x is not None:
                steering_signal = pid.update(center_x)
                print(f"Steering signal: {steering_signal}")
                #Send steering signal to command handler
                command_handler._execute_handler(CommandType.STEER, str(steering_signal))
                #Slowl speed for testing
                command_handler._execute_handler(CommandType.SPEED, "1.0")
                
                cv.putText(final_frame, f'Steer: {steering_signal:.2f}', (10, 60),
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

