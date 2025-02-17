use piston_window::{TextureSettings, Texture, PistonWindow, G2dTexture, Flip};
use std::fs;
use rand::Rng;

pub struct FruitSprites {
    sprites: Vec<G2dTexture>,
}

impl FruitSprites {
    pub fn new(window: &mut PistonWindow) -> Result<Self, String> {
        let mut sprites = Vec::new();
        let texture_settings = TextureSettings::new();
        
        // Read the fruits directory
        let paths = match fs::read_dir("assets/fruits") {
            Ok(paths) => paths,
            Err(e) => return Err(format!("Failed to read fruits directory: {}", e)),
        };

        // Load each fruit image
        for path in paths {
            if let Ok(entry) = path {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("png") {
                    // Skip files that are 0 bytes
                    if let Ok(metadata) = fs::metadata(&path) {
                        if metadata.len() == 0 {
                            continue;
                        }
                    }

                    // Try to load the texture
                    match Texture::from_path(
                        &mut window.create_texture_context(),
                        &path,
                        Flip::None,
                        &texture_settings
                    ) {
                        Ok(texture) => sprites.push(texture),
                        Err(e) => println!("Failed to load texture {:?}: {}", path, e),
                    }
                }
            }
        }

        if sprites.is_empty() {
            Err("No valid fruit sprites were loaded".to_string())
        } else {
            Ok(FruitSprites { sprites })
        }
    }

    pub fn random_index(&self) -> usize {
        rand::thread_rng().gen_range(0..self.sprites.len())
    }

    pub fn get_texture(&self, index: usize) -> &G2dTexture {
        &self.sprites[index]
    }
}

