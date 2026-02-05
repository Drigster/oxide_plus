use crate::utils::APP_DIR_NAME;
use image::DynamicImage;
use image::imageops::FilterType;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use url::Url;

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
    let re = Regex::new(r"[^a-zA-Z0-9\-_]+").unwrap();

    let url = Url::parse(&image_uri)?;

    let domain = url.domain().ok_or("Failed to get domain")?;
    let domain = re.replace_all(domain, "_");

    let path = url.path();
    let path = if path.ends_with(".png") {
        let temp = path.strip_suffix(".png").unwrap();
        let temp = re.replace_all(temp, "_");
        format!("{}.png", temp)
    } else if path.ends_with(".jpg") {
        let temp = path.strip_suffix(".jpg").unwrap();
        let temp = re.replace_all(temp, "_");
        format!("{}.jpg", temp)
    } else {
        path.to_string()
    };

    let filename = format!("{}{}", domain, path);

    // Check local assets folder first
    let local_path = PathBuf::from("src/assets/images").join(&*filename);

    if local_path.exists() {
        println!("Found local image: {}", local_path.display());
        return Ok(local_path);
    }

    // If not found locally, fall back to downloading (for any missing images)
    download_and_cache_image(image_uri.clone(), &filename).await
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
                return Ok(cache_path);
            }
        }
    }

    // Download the image
    let cache_path_clone = cache_path.clone();
    let uri_clone = image_uri.clone();
    blocking::unblock(
        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let bytes: Vec<u8> = ureq::get(&uri_clone).call()?.body_mut().read_to_vec()?;

            if bytes.is_empty() {
                return Err("Downloaded file is empty".into());
            }

            let image = image::load_from_memory(&bytes)?;

            image.save(cache_path_clone)?;

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
        base_dirs.join(APP_DIR_NAME).join("images")
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
