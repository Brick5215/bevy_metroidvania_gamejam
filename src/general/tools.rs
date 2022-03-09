use bevy::math::Vec2;






pub fn lerp (from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress
}



pub fn rotate_vector(vector: Vec2, radians: f32) -> Vec2 {

    Vec2::new(
        vector.x * radians.cos() - vector.y * radians.sin(),
        vector.x * radians.sin() + vector.y * radians.cos(),
    )
}