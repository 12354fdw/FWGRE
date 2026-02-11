use std::collections::HashMap;
use macroquad::prelude::*;
use tracing::info;

pub struct Textures {
    textures: HashMap<String, Vec<Texture2D>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub async fn load(&mut self, id: &str, path: &str) {
        info!("Loading '{}' from '{}'", id, path);

        let tex = load_texture(path).await
            .expect("Failed to load texture");

        tex.set_filter(FilterMode::Nearest);

        self.textures
            .entry(id.to_string())
            .or_insert_with(Vec::new)
            .push(tex);
    }

    pub fn get(&self, id: &str) -> &Texture2D {
        let list = self.textures.get(id)
            .expect("Texture id not found");

        let index = macroquad::rand::gen_range(0, list.len());
        &list[index]
    }

    pub fn get_variant(&self, id: &str, variant: usize) -> &Texture2D {
        &self.textures[id][variant]
    }

    pub fn variant_count(&self, id: &str) -> usize {
        self.textures[id].len()
    }
}
