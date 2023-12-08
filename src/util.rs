use bevy::prelude::*;


pub fn clamp<T>(input: T, min: T, max: T) -> T
where T: PartialOrd<T> {
    assert!(min <= max, "The maximum of clamp should never be less than min!");
    
    if input <= min {
        return min;
    }
    
    if input >= max {
        return max;
    }
    
    input
}

pub fn clamp_vec2_by_length(input: Vec2, min: f32, max: f32) -> Vec2 {
    assert!(min <= max, "The maximum of clamp should never be less than min!");
    let input_length = input.length();
    let clamped_length = clamp(input_length, min, max);
    
    if input_length != clamped_length {
        return input.normalize() * clamped_length;
    }
    
    input
}