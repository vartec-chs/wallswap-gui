pub struct WallpaperDelivery {
    // Fields for wallpaper delivery module
    pub name: &'static str,
    pub url: &'static str,
    pub description: Option<&'static str>,
}
impl WallpaperDelivery {
    pub const fn init(
        name: &'static str,
        url: &'static str,
        description: Option<&'static str>,
    ) -> Self {
        Self {
            name: name,
            url: url,
            description: description,
        }
    }
}

pub const WALLPAPER_DELIVERY_MODULES: &[WallpaperDelivery] = &[WallpaperDelivery::init(
    "Wallpaperscraft",
    "https://wallpaperscraft.com",
    Some("High-quality wallpapers from Wallpaperscraft"),
)];

mod wallpaperscraft;
