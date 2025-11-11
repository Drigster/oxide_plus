use image::DynamicImage;
use image::imageops::FilterType;
use std::fs;
use std::path::PathBuf;

/// Gets the local image path from the assets/images directory.
///
/// Extracts the filename from the URL and looks for it in the local assets/images folder.
/// Falls back to downloading if the local file doesn't exist.
///
/// # Arguments
/// * `image_uri` - The URL of the image (e.g., "https://cdn.rusthelp.com/images/public/scrap.png")
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the local or cached image file
/// * `Err` - If the image cannot be found or downloaded
pub async fn get_cached_image(image_uri: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Extract the filename from the URL
    let filename = image_uri
        .split('/')
        .last()
        .ok_or("Invalid URL: no filename found")?
        .to_string();

    // Check local assets folder first
    let local_path = PathBuf::from("src/assets/images").join(&filename);

    if local_path.exists() {
        return Ok(local_path);
    }

    println!(
        "Local image not found: {:?}, will need to download",
        local_path
    );

    // If not found locally, fall back to downloading (for any missing images)
    download_and_cache_image(image_uri, &filename).await
}

/// Downloads an image and caches it (fallback for missing local images)
async fn download_and_cache_image(
    image_uri: String,
    filename: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cache_dir = get_cache_dir()?;
    let cache_path = cache_dir.join(filename);

    // Check if already in cache
    if cache_path.exists() {
        if let Ok(metadata) = fs::metadata(&cache_path) {
            if metadata.len() > 0 {
                println!("Cache hit: {:?}", cache_path);
                return Ok(cache_path);
            }
        }
    }

    println!("Downloading: {}", image_uri);

    // Download the image
    let cache_path_clone = cache_path.clone();
    let uri_clone = image_uri.clone();
    blocking::unblock(
        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let bytes: Vec<u8> = ureq::get(&uri_clone).call()?.body_mut().read_to_vec()?;

            if bytes.is_empty() {
                return Err("Downloaded file is empty".into());
            }

            let downscaled_bytes = downscale_image(bytes, 96, 96);

            // Write to temporary file first
            let temp_path = cache_path_clone.with_extension("png");

            {
                downscaled_bytes.save_with_format(&temp_path, image::ImageFormat::Png)?;
            }

            // Atomically rename
            fs::rename(&temp_path, &cache_path_clone)?;

            println!("Downloaded and cached: {:?}", cache_path_clone);

            Ok(())
        },
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(cache_path)
}

/// Gets or creates the cache directory for downloaded images
fn get_cache_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    use std::fs;

    let cache_dir = if let Some(base_dirs) = dirs::cache_dir() {
        base_dirs.join("rustplus_freya").join("images")
    } else {
        PathBuf::from(".cache").join("images")
    };

    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

fn downscale_image(bytes: Vec<u8>, width: u32, height: u32) -> DynamicImage {
    let image = image::load_from_memory(&bytes).unwrap();

    image.resize(width, height, FilterType::Lanczos3)
}