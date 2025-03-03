import cv2 as cv
import numpy as np
import warnings

class LineDetector:
	def __init__(self):
		self.roi_params = {
			'bottom_offset': 50,    
			'top_offset': 260,      
			'center_x_offset': 0    
		}
		self.canny_thresholds = (50, 150)
		self.gaussian_kernel = (5, 5)
		self.hough_params = {
			'rho': 2,            
			'theta': np.pi/180,
			'threshold': 100,
			'min_line_length': 40,
			'max_line_gap': 5
		}

	def canny(self, image):
		gray = cv.cvtColor(image, cv.COLOR_RGB2GRAY)
		blur = cv.GaussianBlur(gray, self.gaussian_kernel, 0)
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

		with warnings.catch_warnings():
			warnings.simplefilter('ignore', np.exceptions.RankWarning)
			for line in lines:
				x1, y1, x2, y2 = line.reshape(4)
				parameters = np.polyfit((x1, x2), (y1, y2), 1)
				slope, intercept = parameters
				
				# Calculate where the line intersects the bottom of the image
				y_bottom = height - self.roi_params['bottom_offset']
				x_bottom = int((y_bottom - intercept) / slope) if slope != 0 else x1
				
				# Find line closest to center at the bottom
				distance = abs(x_bottom - center_x)
				
				if distance < min_center_distance:
					min_center_distance = distance
					best_line = parameters

		if best_line is None:
			return []
			
		averaged_line = self.make_coordinates(image, best_line)
		return [averaged_line]

	def make_coordinates(self, image, line_params):
		slope, intercept = line_params
		height = image.shape[0]
		y1 = height - self.roi_params['bottom_offset']
		y2 = height - self.roi_params['top_offset']
		
		try:
			x1 = int((y1 - intercept)/slope)
			x2 = int((y2 - intercept)/slope)
		except ZeroDivisionError:
			x1, x2 = 0, image.shape[1]
		
		x1 = np.clip(x1, 0, image.shape[1])
		x2 = np.clip(x2, 0, image.shape[1])
		
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
		
		return line_image
