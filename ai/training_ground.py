import time
import random
import csv

from main import WindowCapture, LineDetector, YOLODetector, extract_state

class TrainingGround:
    def __init__(self):
        
        #reference to the window capture object
        self.wincap = WindowCapture()
        self.line_detector = LineDetector()
        self.yolo_detector = YOLODetector()
        
        
        self.actions = [
            #Actions/Commands can go in here
        ]
        
        #Pick an action based on state of vectore
        def pick_action(self):
           
            return random.choice(self.actions)
        
        #Applying the action and calling command handler to change speed, steering, etc
        def apply_action(self, action):
            return
        
        #make it run for 30 seconds then reset, basically main loop tying the other
        # functions together
        def training_loop(self, iterations=20):
            return






if __name__ == "__main__":
  
    tg = TrainingGround()
    tg.run_training_loop(iterations=10)