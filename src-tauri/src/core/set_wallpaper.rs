use crate::core::errors::{AppErrors, FileSystemError, OperationError, SystemError};
use std::path::Path;
use tokio::task;

/// Режимы установки обоев
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperMode {
    /// Центрировать изображение
    Center,
    /// Растянуть на весь экран
    Stretch,
    /// Замостить (повторить)
    Tile,
    /// Заполнить (сохраняя пропорции)
    Fill,
    /// Подогнать (сохраняя пропорции)
    Fit,
    /// Развернуть (обрезая при необходимости)
    Span,
}

impl Default for WallpaperMode {
    fn default() -> Self {
        WallpaperMode::Fill
    }
}

/// Основная структура для работы с обоями
pub struct WallpaperSetter;

impl WallpaperSetter {
    /// Создать новый экземпляр WallpaperSetter
    pub fn new() -> Self {
        Self
    }

    /// Установить обои асинхронно без блокировки основного потока
    pub async fn set_wallpaper<P: AsRef<Path>>(
        &self,
        wallpaper_path: P,
        mode: WallpaperMode,
    ) -> Result<(), AppErrors> {
        let path = wallpaper_path.as_ref().to_path_buf();

        // Проверяем существование файла
        if !path.exists() {
            return Err(AppErrors::FileSystem(
                FileSystemError::WallpaperFileNotFound(path.display().to_string()),
            ));
        }

        // Выполняем установку в отдельном потоке
        task::spawn_blocking(move || Self::set_wallpaper_sync(&path, mode))
            .await
            .map_err(|e| {
                AppErrors::Operation(OperationError::WallpaperSetError(format!(
                    "Ошибка выполнения задачи: {}",
                    e
                )))
            })?
    }

    /// Синхронная установка обоев для конкретной ОС
    fn set_wallpaper_sync(wallpaper_path: &Path, mode: WallpaperMode) -> Result<(), AppErrors> {
        #[cfg(target_os = "windows")]
        {
            Self::set_wallpaper_windows(wallpaper_path, mode)
        }

        #[cfg(target_os = "macos")]
        {
            Self::set_wallpaper_macos(wallpaper_path, mode)
        }

        #[cfg(target_os = "linux")]
        {
            Self::set_wallpaper_linux(wallpaper_path, mode)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(AppErrors::System(SystemError::UnsupportedOS))
        }
    }

    /// Установка обоев в Windows
    #[cfg(target_os = "windows")]
    fn set_wallpaper_windows(wallpaper_path: &Path, _mode: WallpaperMode) -> Result<(), AppErrors> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
        };

        let path_str = wallpaper_path.to_str().ok_or_else(|| {
            AppErrors::Operation(OperationError::WallpaperSetError(
                "Неверный путь к файлу".to_string(),
            ))
        })?;

        // Конвертируем путь в UTF-16
        let wide_path: Vec<u16> = OsStr::new(path_str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            // Устанавливаем обои
            let result = SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(wide_path.as_ptr() as *mut _),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            );

            if result.is_ok() {
                Ok(())
            } else {
                Err(AppErrors::Operation(OperationError::WallpaperSetError(
                    "Не удалось установить обои в Windows".to_string(),
                )))
            }
        }
    }

    /// Установка обоев в macOS
    #[cfg(target_os = "macos")]
    fn set_wallpaper_macos(wallpaper_path: &Path, _mode: WallpaperMode) -> Result<(), AppErrors> {
        use std::process::Command;

        let path_str = wallpaper_path.to_str().ok_or_else(|| {
            AppErrors::Operation(OperationError::WallpaperSetError(
                "Неверный путь к файлу".to_string(),
            ))
        })?;

        // Используем osascript для установки обоев
        let script = format!(
            r#"tell application "Finder" to set desktop picture to POSIX file "{}""#,
            path_str
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| {
                AppErrors::Operation(OperationError::WallpaperSetError(format!(
                    "Ошибка выполнения osascript: {}",
                    e
                )))
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(AppErrors::Operation(OperationError::WallpaperSetError(
                format!("Не удалось установить обои в macOS: {}", error_msg),
            )))
        }
    }

    /// Установка обоев в Linux
    #[cfg(target_os = "linux")]
    fn set_wallpaper_linux(wallpaper_path: &Path, _mode: WallpaperMode) -> Result<(), AppErrors> {
        use std::process::Command;

        let path_str = wallpaper_path.to_str().ok_or_else(|| {
            AppErrors::Operation(OperationError::WallpaperSetError(
                "Неверный путь к файлу".to_string(),
            ))
        })?;

        // Пробуем различные методы в зависимости от окружения рабочего стола
        let desktop_methods = [
            // GNOME/Unity
            (
                "gsettings",
                vec![
                    "set",
                    "org.gnome.desktop.background",
                    "picture-uri",
                    &format!("file://{}", path_str),
                ],
            ),
            // KDE
            (
                "qdbus",
                vec![
                    "org.kde.plasmashell",
                    "/PlasmaShell",
                    "org.kde.PlasmaShell.evaluateScript",
                    &format!(
                        "var allDesktops = desktops();for (i=0;i<allDesktops.length;i++) {{d = allDesktops[i];d.wallpaperPlugin = \"org.kde.image\";d.currentConfigGroup = Array(\"Wallpaper\", \"org.kde.image\", \"General\");d.writeConfig(\"Image\", \"file://{}\");}}",
                        path_str
                    ),
                ],
            ),
            // XFCE
            (
                "xfconf-query",
                vec![
                    "-c",
                    "xfce4-desktop",
                    "-p",
                    "/backdrop/screen0/monitor0/workspace0/last-image",
                    "-s",
                    path_str,
                ],
            ),
            // Универсальный метод через feh
            ("feh", vec!["--bg-scale", path_str]),
            // Nitrogen
            ("nitrogen", vec!["--set-scaled", path_str]),
        ];

        for (command, args) in &desktop_methods {
            if let Ok(output) = Command::new(command).args(args).output() {
                if output.status.success() {
                    return Ok(());
                }
            }
        }

        Err(AppErrors::Operation(OperationError::WallpaperSetError(
            "Не удалось установить обои в Linux. Убедитесь, что установлены gsettings, feh или nitrogen".to_string()
        )))
    }

    /// Получить текущие обои (только для Windows и macOS)
    pub async fn get_current_wallpaper(&self) -> Result<Option<String>, AppErrors> {
        task::spawn_blocking(|| Self::get_current_wallpaper_sync())
            .await
            .map_err(|e| {
                AppErrors::Operation(OperationError::WallpaperSetError(format!(
                    "Ошибка выполнения задачи: {}",
                    e
                )))
            })?
    }

    #[cfg(target_os = "windows")]
    fn get_current_wallpaper_sync() -> Result<Option<String>, AppErrors> {
        use windows::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPI_GETDESKWALLPAPER, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
        };

        let mut buffer = vec![0u16; 260]; // MAX_PATH

        unsafe {
            let result = SystemParametersInfoW(
                SPI_GETDESKWALLPAPER,
                buffer.len() as u32,
                Some(buffer.as_mut_ptr() as *mut _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );

            if result.is_ok() {
                let end = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
                let path = String::from_utf16(&buffer[..end]).map_err(|e| {
                    AppErrors::Operation(OperationError::WallpaperSetError(format!(
                        "Ошибка декодирования пути: {}",
                        e
                    )))
                })?;

                if path.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(path))
                }
            } else {
                Err(AppErrors::Operation(OperationError::WallpaperSetError(
                    "Не удалось получить текущие обои".to_string(),
                )))
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn get_current_wallpaper_sync() -> Result<Option<String>, AppErrors> {
        use std::process::Command;

        let output = Command::new("osascript")
            .arg("-e")
            .arg("tell application \"Finder\" to get POSIX path of (desktop picture as alias)")
            .output()
            .map_err(|e| {
                AppErrors::Operation(OperationError::WallpaperSetError(format!(
                    "Ошибка выполнения osascript: {}",
                    e
                )))
            })?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if path.is_empty() {
                Ok(None)
            } else {
                Ok(Some(path))
            }
        } else {
            Err(AppErrors::Operation(OperationError::WallpaperSetError(
                "Не удалось получить текущие обои".to_string(),
            )))
        }
    }

    #[cfg(target_os = "linux")]
    fn get_current_wallpaper_sync() -> Result<Option<String>, AppErrors> {
        // Для Linux получение текущих обоев сложнее из-за разнообразия DE
        // Возвращаем None, так как это не критично для основного функционала
        Ok(None)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    fn get_current_wallpaper_sync() -> Result<Option<String>, AppErrors> {
        Err(AppErrors::System(SystemError::UnsupportedOS))
    }
}

impl Default for WallpaperSetter {
    fn default() -> Self {
        Self::new()
    }
}

/// Удобная функция для быстрой установки обоев с режимом по умолчанию
pub async fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), AppErrors> {
    let setter = WallpaperSetter::new();
    setter
        .set_wallpaper(wallpaper_path, WallpaperMode::default())
        .await
}

/// Удобная функция для быстрой установки обоев с указанным режимом
pub async fn set_wallpaper_with_mode<P: AsRef<Path>>(
    wallpaper_path: P,
    mode: WallpaperMode,
) -> Result<(), AppErrors> {
    let setter = WallpaperSetter::new();
    setter.set_wallpaper(wallpaper_path, mode).await
}

/// Удобная функция для получения текущих обоев
pub async fn get_current_wallpaper() -> Result<Option<String>, AppErrors> {
    let setter = WallpaperSetter::new();
    setter.get_current_wallpaper().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wallpaper_setter_creation() {
        let _setter = WallpaperSetter::new();
        assert!(true); // Просто проверяем, что создание работает
    }

    #[tokio::test]
    async fn test_nonexistent_file() {
        let setter = WallpaperSetter::new();
        let result = setter
            .set_wallpaper("nonexistent.jpg", WallpaperMode::Fill)
            .await;
        assert!(result.is_err());
    }
}
