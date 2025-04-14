//A spline is a piecewise polynomial function that can be used to create smooth curves in a 3d space
// using this here to make it look better overall when generating the enviornment
use bevy::prelude::*;
use super::road::Segment;   




#[derive(Resource, Clone)]
pub struct Spline {

    points: Vec<Vec3>,
}

impl Spline {
    //build a spline from the segments
    pub fn from_segments(segments: &[Segment]) -> Self {
        
        let mut pts = Vec::with_capacity(segments.len() + 1);
        if let Some(first) = segments.first() {
            pts.push(first.start);
        }
        for seg in segments {
            pts.push(seg.end);
        }
        Self { points: pts }
    }

    //Return shortest distance from p to the poly‑line
    pub fn distance_to(&self, p: Vec3) -> f32 {
        self.points
            .windows(2)
            .map(|w| distance_point_to_segment(p, w[0], w[1]))
            .fold(f32::MAX, f32::min)
    }

    // road is flat at y = 0.0
    pub const HEIGHT: f32 = 0.0;
}

// Helper – distance from a point to a line segment in 3‑D
fn distance_point_to_segment(p: Vec3, a: Vec3, b: Vec3) -> f32 {
    let ab = b - a;
    let t  = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    (a + t * ab - p).length()
}