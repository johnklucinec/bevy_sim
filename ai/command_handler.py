import sys
import re
from threading import Thread, Lock
from queue import Queue
from enum import Enum

class CommandType(Enum):
    """Valid command types and their string representations"""
    DETECT = "DETECT"
    RESET = "RESET"
    SPEED = "SPEED"

class CommandHandler:
    """Main class for handling command processing and responses"""
    
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
        """Initialize the command-to-handler function mapping"""
        self.command_map = {
            CommandType.DETECT: self.handle_detect,
            CommandType.RESET: self.handle_reset,
            CommandType.SPEED: self.handle_speed
        }

	# --------------------------------------------------
    # Command-specific Handlers
    # --------------------------------------------------
    
    def handle_detect(self, value: str | None = None) -> str:
        """Handle detection initialization"""
        return "Starting detection"

    def handle_reset(self, value: str | None = None) -> str:
        """Handle system reset"""
        return "Resetting detectors"

    def handle_speed(self, value: str | None = None) -> str:
        """Handle speed configuration/query"""
        if not value:
            return "Speed value missing"
        return f"The current speed is: {value}"

    # --------------------------------------------------
    # Response Management
    # --------------------------------------------------
    
    def send_response(self, response: str):
        """Handle output of system responses"""
        print(response, flush=True)
        self.response_queue.put(response)

    def stop(self):
        """Clean shutdown of command processing"""
        self.running = False

    # --------------------------------------------------
    # Everything for the Main Loop
    # --------------------------------------------------

    def _listen_for_commands(self):
        """Main loop for command input listening"""
        while self.running:
            try:
                command = sys.stdin.readline().strip()
                if command:
                    self.command_queue.put(command)
                    self._handle_command(command)
            except Exception as e:
                self.send_response(f"Error reading command: {e}")

    def _handle_command(self, command: str):
        """Process incoming commands and dispatch to appropriate handler"""
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
        """Extract command type and value using regex pattern matching"""
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
        """Dispatch command to the registered handler function"""
        handler = self.command_map.get(cmd_type)
        return handler(value) if handler else "Unknown command type"