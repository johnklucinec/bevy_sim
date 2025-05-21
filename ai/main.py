# Author: John Klucinec (@johnklucinec)
# Modified by: Brant

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
    
    pid = PIDController(kp = 0.25, ki = 0.015, kd = 0.2, setpoint = 250.0)
    steady_speed = 100
    loop_time = time()
    commands = []
    buffer_size = 15

    # Threshold for cone targeting
    cone_avoidance_region = (175,325,275) # left, right, top coordinates
    center_of_avoidance_region = (cone_avoidance_region[0] + cone_avoidance_region[1]) / 2
    cone_safety_cushion = 25

    try:
        speed_val = command_handler._execute_handler(CommandType.SPEED, str(steady_speed))
        print(speed_val)
        
        while True:
            screenshot = wincap.get_screenshot()
            
            yolo_frame, cone_center = yolo_detector.process_frame(screenshot.copy())
            line_frame, center_x = line_detector.process_frame(screenshot.copy())
            
            final_frame = cv.bitwise_or(yolo_frame, line_frame)
            cv.rectangle(final_frame, (cone_avoidance_region[0], cone_avoidance_region[2]), (cone_avoidance_region[1], 500), (0, 255, 0), 2)
            
            pid_x_offset = None
            steering_source = "None"
            scaled_steering = None # Initialize to prevent UnboundLocalError

            # Priority 1: Relevant Cone
            if cone_center is not None:
                cone_pos_x, cone_pos_y = cone_center
                # Check if cone is within the avoidance region
                if cone_pos_y > cone_avoidance_region[2] and cone_avoidance_region[0] <= cone_pos_x <= cone_avoidance_region[1]:
                    # Calculate avoidance offset based on cone position
                    distance_from_center = cone_pos_x - center_of_avoidance_region
                    
                    # Calculate avoidance direction (negative = steer left, positive = steer right)
                    if abs(distance_from_center) < 30:  # If cone is near center
                        # Calculate which side has more space to maneuver
                        left_space = cone_pos_x - cone_avoidance_region[0]
                        right_space = cone_avoidance_region[1] - cone_pos_x
                        
                        # Calculate car's position relative to center
                        car_offset = center_x - center_of_avoidance_region if center_x is not None else 0
                        
                        # If car is already significantly offset to one side, continue in that direction
                        if abs(car_offset) > 20:
                            avoidance_direction = -1 if car_offset < 0 else 1
                        # Otherwise, choose the side with more space
                        else:
                            avoidance_direction = -1 if left_space > right_space else 1
                        
                        # Stronger avoidance when directly in front, gradually decrease as cone moves to sides
                        avoidance_strength = 250 * (1 - min(1, abs(distance_from_center) / 50))
                        avoidance_offset = avoidance_direction * max(100, avoidance_strength)  # Minimum 150 offset
                    else:
                        # For cones not in dead center, use proportional avoidance
                        avoidance_strength = 2
                        avoidance_offset = -distance_from_center * avoidance_strength
                    
                    # Add safety cushion in the direction we're steering
                    avoidance_offset += cone_safety_cushion * (1 if avoidance_offset > 0 else -1)
                    
                    # Clamp the offset to reasonable values
                    pid_x_offset = max(-300, min(300, avoidance_offset))
                    
                    # Draw debug info
                    cv.putText(final_frame, f'Cone Avoidance: {pid_x_offset:.0f}', 
                                (10, 150), cv.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 255), 2)

            # Priority 2: Line                
            if center_x is not None:
                if pid_x_offset is not None:
                    center_x += pid_x_offset    
                steering_source = f"Line ({center_x})"
                raw_pid = pid.update(center_x)
                scaling_factor = 0.02  # STEERING_SCALING_FACTOR
                scaled_steering = raw_pid * scaling_factor
                
                commands.append(scaled_steering)

                if len(commands) >= buffer_size: 
                    avg_steering_command = sum(commands) / len(commands)
                    val = command_handler._execute_handler(CommandType.STEER, str(avg_steering_command))
                    print(val)
                    commands = [] # Clear buffer after sending
                
                # Display steering info on frame
                cv.putText(final_frame, f'Target: {steering_source}', (10, 60),
                           cv.FONT_HERSHEY_SIMPLEX, 0.7, (0, 255, 0), 2)
                if scaled_steering is not None: # Ensure scaled_steering is valid before drawing
                    cv.putText(final_frame, f'SteerVal: {scaled_steering:.2f}', (10, 90),
                               cv.FONT_HERSHEY_SIMPLEX, 0.7, (0, 255, 0), 2)

            loop_time = display_fps(final_frame, loop_time)
            cv.imshow('Final View', final_frame)

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
