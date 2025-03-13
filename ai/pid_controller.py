class PIDController:

    def __init__(self, kp=1.0, ki=0.0, kd=0.0, setpoint=250.0, buffer_size=5):
            
            # kp = Proportional gain
            # ki = Integral gain
            # kd = Derivative gain
            # setpoint = target value for 'center_x'
            
            self.kp = kp
            self.ki = ki
            self.kd = kd
            self.setpoint = setpoint
            
            # PID state
            self._intergral = 0.0
            self._prev_error = 0.0
            
            #buffer for output
            self.output_buffer = []
            self.buffer_size = buffer_size
            
    
    def reset(self):
        self._integral = 0.0
        self._prev_error = 0.0
        self.output_buffer = []
    
    #calculates the pid output using measured value and returns control signal
    # measured_value = current center_x value
    # dt = time step since last update
    def update(self, measure_value, dt=0.016):
        
        #error
        error = measure_value - self.setpoint
        
        #proportional term
        p_out = self.kp * error
        
        #integral term
        self._intergral += error * dt
        i_out = self.ki * self._intergral
        
        #derivative term
        derivative = (error - self._prev_error) / dt
        d_out = self.kd * derivative
        
        control_signal = p_out + i_out + d_out
        
        self._prev_error = error
        
        self.output_buffer.append(control_signal)
        if len(self.output_buffer) > self.buffer_size:
            self.output_buffer.pop(0)
        
        avg_pid = sum(self.output_buffer) / len(self.output_buffer)
               
        return avg_pid
        
        
        
        
        

                
            