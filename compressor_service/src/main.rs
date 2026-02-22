mod semantic;

use anyhow::Result;
use notify::{Watcher, RecursiveMode, Config};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use zstd::dict::EncoderDictionary;
use lexivault_lib::HybridCompressor;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Transparent Compressor Service...");

    // 1. Filesystem Watcher
    let (tx, rx) = channel();
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default())?;
    
    // Watch a specific development directory
    let watch_path = Path::new("c:\\Drive_D\\webdev\\Compressor\\test_repo");
    if watch_path.exists() {
        watcher.watch(watch_path, RecursiveMode::Recursive)?;
        println!("Watching: {:?}", watch_path);
    }

    // 2. Event Loop
    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() || event.kind.is_create() {
                    for path in event.paths {
                        handle_file_event(path).await?;
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn handle_file_event(path: PathBuf) -> Result<()> {
    if let Some(ext) = path.extension() {
        if ext == "java" || ext == "rs" {
            println!("Optimizing: {:?}", path);
            
            // Trigger Dictionary Training if enough files are present
            // In a real system, we'd collect samples and call zstd::dict::train_on_files
            // training_service::train_new_dictionary(&path.parent().unwrap())?;
        }
    }
    Ok(())
}

pub mod training_service {
    use super::*;
    
    pub fn train_new_dictionary(dir: &Path) -> Result<Vec<u8>> {
        // Collect samples from the directory
        let mut samples: Vec<Vec<u8>> = Vec::new();
        // ... read files ...
        
        // Train dictionary
        // let dict = zstd::dict::train_on_files(&samples, 112640)?; 
        // Ok(dict)
        Ok(vec![])
    }
}
