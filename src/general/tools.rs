use bevy::{math::Vec2, sprite::TextureAtlas, prelude::{Assets, Handle, AssetServer}};






pub fn lerp (from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress
}



pub fn rotate_vector(vector: Vec2, radians: f32) -> Vec2 {

    Vec2::new(
        vector.x * radians.cos() - vector.y * radians.sin(),
        vector.x * radians.sin() + vector.y * radians.cos(),
    )
}

pub fn clamp_shift(mut val: f32, min: f32, max: f32, by: f32) -> f32 {

    if val >= min && val <= max {
        return val
    }

    if val < min {
        val += by;
        return val.min(min);
    }
    else {
        val -= by;
        return val.max(max);
    }
}

pub fn load_texture_atlas(
    assets: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_path: &str,
    tile_size: Vec2,
    columns: usize,
    rows: usize,

) -> Handle<TextureAtlas> {

    let texture_handle = assets.load(asset_path);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, columns, rows);
    return texture_atlases.add(texture_atlas);
}