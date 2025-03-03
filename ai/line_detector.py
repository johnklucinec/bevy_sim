import cv2 as cv
import numpy as np
import warnings

class LineDetector:
	def __init__(self):
		self.roi_params = {
			'bottom_offset': 60,    
			'top_offset': 260,      
			'center_x_offset': 0    
		}
		self.canny_thresholds = (50, 100)
		self.gaussian_kernel = (5, 5)
		self.hough_params = {
			'rho': 2,            
			'theta': np.pi/180,
			'threshold': 30,
			'min_line_length': 40,
			'max_line_gap': 1
		}

	def canny(self, image):
		gray = cv.cvtColor(image, cv.COLOR_RGB2GRAY)
		blur = cv.GaussianBlur(gray, self.gaussian_kernel, 1)
		return cv.Canny(blur, *self.canny_thresholds)

	def region_of_interest(self, image):
		height, width = image.shape[:2]
		bottom = height - self.roi_params['bottom_offset']
		top = height - self.roi_params['top_offset']
		
		poly_coords = np.array([[
			(0, bottom),
			(width, bottom),
			(width, top),
			(0, top)
		]], dtype=np.int32)
		
		mask = np.zeros_like(image)
		mask = cv.fillPoly(mask, poly_coords, 255)
		return cv.bitwise_and(image, mask)

	def average_slope_intercept(self, image, lines):
		if lines is None:
			return []
		
		height, width = image.shape[:2]
		center_x = width // 2
		best_line = None
		min_center_distance = float('inf')

		for line in lines:
			x1, y1, x2, y2 = line.reshape(4)
			
			# Handle vertical lines explicitly
			if x1 == x2:
				slope = float('inf')
				intercept = x1
			else:
				with warnings.catch_warnings():
					warnings.simplefilter('ignore', np.exceptions.RankWarning)
					slope, intercept = np.polyfit((x1, x2), (y1, y2), 1)
			
			# Calculate where the line intersects the bottom of the image
			y_bottom = height - self.roi_params['bottom_offset'] / 2
			if slope == float('inf'):
				x_bottom = x1
			elif slope == 0:
				x_bottom = center_x  # Horizontal line, use center
			else:
				x_bottom = int((y_bottom - intercept) / slope)
			
			# Find line closest to center at the bottom
			distance = abs(x_bottom - center_x)
			
			if distance < min_center_distance:
				min_center_distance = distance
				best_line = (slope, intercept)

		if best_line is None:
			return []
			
		averaged_line = self.make_coordinates(image, best_line)
		return [averaged_line]

	def make_coordinates(self, image, line_params):
		slope, intercept = line_params
		height, width = image.shape[:2]
		y1 = height - self.roi_params['bottom_offset']
		y2 = int(height * (1 - 0.55))
		
		if slope == float('inf'):
			return [int(intercept), y1, int(intercept), y2]
		elif slope == 0:
			return [0, int(intercept), width, int(intercept)]
		else:
			x1 = int((y1 - intercept) / slope)
			x2 = int((y2 - intercept) / slope)	
		
		x1 = np.clip(x1, 0, width - 1)
		x2 = np.clip(x2, 0, width - 1)
		
		return [x1, y1, x2, y2]



	def display_lines(self, image, lines):
		line_image = np.zeros_like(image)
		if lines is not None:
			for line in lines:
				x1, y1, x2, y2 = line.reshape(4)
				cv.line(line_image, (x1, y1), (x2, y2), (255, 0, 0), 2)
		return line_image

	def process_frame(self, frame):
		canny_image = self.canny(frame)
		cropped_image = self.region_of_interest(canny_image)
		
		lines = cv.HoughLinesP(
			cropped_image,
			self.hough_params['rho'],
			self.hough_params['theta'],
			self.hough_params['threshold'],
			np.array([]),
			minLineLength=self.hough_params['min_line_length'],
			maxLineGap=self.hough_params['max_line_gap']
		)
		
		averaged_lines = self.average_slope_intercept(frame, lines)
		line_image = np.zeros_like(frame)
		
		if averaged_lines:
			line = averaged_lines[0]
			x1, y1, x2, y2 = line
			cv.line(line_image, (x1, y1), (x2, y2), (255, 0, 0), 3)
		
		#return self.display_lines(frame, lines)
		return line_image

