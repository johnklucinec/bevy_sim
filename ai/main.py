import cv2 as cv
from time import time
import os
from windowcapture import WindowCapture
from line_detector import LineDetector
from yolo_detector import YOLODetector
from command_handler import CommandHandler
from pid_controller import PIDController

os.chdir(os.path.dirname(os.path.abspath(__file__)))

# Initialize components
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

    pid = PIDController(kp = 0.5, ki = 0.0, kd = 0.05, setpoint = 250.0)

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

if __name__ == '__main__':

    # Uncomment the function you want to use
    normal_display(wincap, yolo_detector, line_detector)
    # debug_display()

