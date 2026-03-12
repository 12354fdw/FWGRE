use bevy::prelude::*;
use bevy::image::TextureAtlasBuilder;

use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct TextureRegistry {
    pub tiles: HashMap<String, usize>,
}

#[derive(Resource)]
pub struct TexturesLoading {
    handles: Vec<(String, Handle<Image>)>
}

#[derive(Resource)]
pub struct Atlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>
}

// all textures at 128x128

impl TextureRegistry {
    pub fn get(&self, name: &str) -> usize {
        *self.tiles.get(name).expect("Texture not found in registry!")
    }
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut handles = Vec::new();

    let tiles = ["grass", "water"];

    for tile in tiles {
        let handle: Handle<Image> = asset_server.load(format!("textures/{tile}.png"));
        handles.push((tile.to_string(), handle));
    }

    commands.insert_resource(TexturesLoading { handles });
}

pub fn build_atlas(
    mut commands: Commands,
    loading: Res<TexturesLoading>,
    mut textures: ResMut<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    atlas: Option<Res<Atlas>>,
) {
    if atlas.is_some() {
        return;
    }

    let mut builder = TextureAtlasBuilder::default();
    let mut all_loaded = true;

    for (_, handle) in &loading.handles {
        if let Some(texture) = textures.get(handle) {
            builder.add_texture(Some(handle.id()), texture);
        } else {
            all_loaded = false;
        }
    }

    if !all_loaded || loading.handles.is_empty() {
        return;
    }

    let (atlas_layout, atlas_sources, atlas_texture) = builder.build().unwrap();

    let atlas_layout_handle = atlases.add(atlas_layout);
    let atlas_texture_handle = textures.add(atlas_texture);

    commands.insert_resource(Atlas {
        texture: atlas_texture_handle,
        layout: atlas_layout_handle,
    });

    let mut registry = TextureRegistry::default();

    for (name, handle) in &loading.handles {
        if let Some(index) = atlas_sources.texture_index(handle.id()) {
            registry.tiles.insert(name.clone(), index);
        }
    }

    commands.insert_resource(registry);
}