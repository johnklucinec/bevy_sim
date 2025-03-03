import sys
from threading import Thread, Lock
from queue import Queue


class CommandHandler:
    def __init__(self):
        self.command_queue = Queue()
        self.response_queue = Queue()
        self.running = True
        self.lock = Lock()

        # Start command listening thread
        self.command_thread = Thread(target=self._listen_for_commands)
        self.command_thread.daemon = True
        self.command_thread.start()

    def _listen_for_commands(self):
        while self.running:
            try:
                command = sys.stdin.readline().strip()
                if command:
                    self.command_queue.put(command)
                    self._handle_command(command)
            except Exception as e:
                print(f"Error reading command: {e}", flush=True)

    def _handle_command(self, command: str):
        # Get the first word of the command (without parameters)
        command_parts = command.split()
        command_name = command_parts[0] if command_parts else ""

        # Check if the command (without parameters) is in the list
        car_commands = [
            "GO",
            "STOP",
            "LEFT",
            "RIGHT",
            "GEAR",
            "THROTTLE",
            "TURN"
            ]

        if command == "DETECT":
            response = "Starting detection"
        elif command == "RESET":
            response = "Resetting detectors"
        elif command_name.upper() in car_commands:
            # Send the entire command including parameters
            response = command
        else:
            response = f"Unknown command: {command}"

        self.send_response(response)

    def send_response(self, response: str):
        print(response, flush=True)
        self.response_queue.put(response)

    def stop(self):
        self.running = False

    def execute_commands(self, *commands):
        for command in commands:
            self._handle_command(command)
