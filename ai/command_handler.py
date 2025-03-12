# Author: John Klucinec (@johnklucinec)
# Modified by: Brant and Ramiro

import sys
import re
from threading import Thread, Lock
from queue import Queue
from enum import Enum
import cv2 as cv
from windowcapture import WindowCapture
from line_detector import LineDetector

wincap = WindowCapture('Camera View')
line_detector = LineDetector()


def debug_display(): # NEED TO ADD COMMAND HANDLER
	while True:
		screenshot = wincap.get_screenshot()
		
		# Process frame with both detectors
		line_frame, _ = line_detector.process_frame(screenshot.copy())
		
		cv.imshow('Computer Vision', line_frame)
		
		if cv.waitKey(1) == ord('q'):
			cv.destroyAllWindows()
			break



class CommandType(Enum):
    """
    Valid command types and their string representations
    """

    DETECT = "DETECT"
    RESET = "RESET"
    SPEED = "SPEED"
    STEER = "STEER"


class CommandHandler:
    """
    Main class for handling command processing and responses
    """

    def __init__(self):
        # Thread-safe communication channels
        self.command_queue = Queue()
        self.response_queue = Queue()
        self.running = True
        self.lock = Lock()  # Currently unused

        # Initialize command mapping
        self._setup_command_map()

        # Start command processing thread
        self.command_thread = Thread(target=self._listen_for_commands)
        self.command_thread.daemon = True
        self.command_thread.start()

    def _setup_command_map(self):
        """
        Initialize the command-to-handler function mapping
        """

        self.command_map = {
            CommandType.DETECT: self.handle_detect,
            CommandType.RESET: self.handle_reset,
            CommandType.SPEED: self.handle_speed,
            CommandType.STEER: self.handle_steer,
        }

    # --------------------------------------------------
    # Command-specific Handlers
    # --------------------------------------------------

    def handle_detect(self, value: str | None = None) -> str:
        """
        Handle detection initialization
        """

        return "DETECT: We detected a traffic cone."
        #return "DETECT: 10.1";

    def handle_reset(self, value: str | None = None) -> str:
        """
        Handle system reset
        """

        return "Resetting detectors"

    def handle_speed(self, value: str | None = None) -> str:
        """
        Handle speed command
        """

        if not value:
            return "Speed value missing"
        try:
            speed = float(value)
            return f"SPEED: {speed}"
        except ValueError:
            return f"Invalid speed value: {value}"

    def handle_steer(self, value: str | None = None) -> str:
        """
        Handle steer commands
        """
        if not value:
            return CommandType.STEER.value + ": Value missing"
        try:
            angle = float(value)
            return CommandType.STEER.value + f": {angle}"
        except ValueError:
            return CommandType.STEER.value + f": Invalid STEER value: {value}"

        return f"The current speed is: {value}"
    
    def handle_debug(self, value: str | None = None) -> str:
        """
        Start debug display in new thread
        """
        debug_thread = Thread(target=debug_display)
        debug_thread.daemon = True
        debug_thread.start()
        return "Starting debug display"

    # --------------------------------------------------
    # Response Management
    # --------------------------------------------------

    def send_response(self, response: str):
        """
        Handle output of system responses
        """

        print(response, flush=True)
        self.response_queue.put(response)

    def stop(self):
        """
        Clean shutdown of command processing
        """

        self.running = False

    # --------------------------------------------------
    # Everything for the Main Loop
    # --------------------------------------------------

    def _listen_for_commands(self):
        """
        Main loop for command input listening
        """

        while self.running:
            try:
                command = sys.stdin.readline().strip()
                if command:
                    self.command_queue.put(command)
                    self._handle_command(command)
            except Exception as e:
                self.send_response(f"Error reading command: {e}")

    def _handle_command(self, command: str):
        """
        Process incoming commands and dispatch to appropriate handler
        """

        try:
            # Parse command into components
            cmd_type, value = self._parse_command(command)
            if not cmd_type:
                self.send_response("Invalid command format")
                return

            # Execute corresponding handler
            response = self._execute_handler(cmd_type, value)
            self.send_response(response)

        except Exception as e:
            self.send_response(f"Error processing command: {e}")

    def _parse_command(self, command: str) -> tuple[CommandType | None, str | None]:
        """
        Extract command type and value using regex pattern matching
        """

        pattern = r"^([A-Z]+):?\s*(.*)$"  # Command structure: TYPE[: VALUE]
        match = re.match(pattern, command)

        # Return early if there is an invalid command
        if not match:
            return None, None

        cmd_type_str, value = match.groups()

        try:
            return CommandType(cmd_type_str), value
        except ValueError:
            return None, value

    def _execute_handler(self, cmd_type: CommandType, value: str | None) -> str:
        """
        Dispatch command to the registered handler function
        """

        handler = self.command_map.get(cmd_type)
        return handler(value) if handler else "Unknown command type"
