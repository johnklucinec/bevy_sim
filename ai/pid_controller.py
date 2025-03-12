class PIDController:

    def __init__(self, kp=1.0, ki=0.0, kd=0.0, setpoint=250.0):
            
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
    
    #calculates the pid output using measured value and returns control signal
    # measured_value = current center_x value
    # dt = time step since last update
    def update(self, measure_value, dt=1.0):
        
        #error
        error = self.setpoint - measure_value
        
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
        
        return control_signal
        
        
        
        
        

                
            