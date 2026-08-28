#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod album_view;

const HEADER_HEIGHT: f32 = 80.0;
const HEADER_LOGO_WIDTH: f32 = 117.0 * 1.3 * 1.3;
const HEADER_ICON_SIZE: f32 = 60.0;

use album_view::AlbumView;
use chrono::{Datelike, Local};
use eframe::egui;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_RANGE, RANGE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
struct Album {
    id: String,
    #[serde(rename = "albumName")]
    album_name: String,
    #[serde(rename = "assetCount", default)]
    asset_count: usize,
    #[serde(default)]
    shared: bool,
    #[serde(rename = "albumThumbnailAssetId", default)]
    album_thumbnail_asset_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Asset {
    id: String,
    #[serde(rename = "originalFileName", default)]
    original_file_name: String,
    #[serde(rename = "deviceAssetId", default)]
    device_asset_id: String,
    #[serde(rename = "type", default)]
    asset_type: String,
    #[serde(default)]
    checksum: String,
    #[serde(rename = "localDateTime", default)]
    local_date_time: String,
    #[serde(rename = "fileSizeInByte", default)]
    file_size_in_byte: i64,
}

#[derive(Clone)]
struct SelectableAlbum {
    album: Album,
    selected: bool,
    thumbnail: Option<egui::TextureHandle>,
}

#[derive(Clone)]
struct YearBucket {
    year: String,
    count: usize,
    total_size: i64,
    selected: bool,
}

#[derive(Clone)]
struct DownloadJob {
    asset: Asset,
    folder_name: String,
    group_name: String,
    album_position: Option<(usize, usize)>,
}

#[derive(Clone)]
struct ConflictItem {
    job: DownloadJob,
    selected: bool,
    local_size: u64,
    remote_size: Option<u64>,
}

#[derive(PartialEq, Clone, Copy)]
enum MediaMode {
    All,
    Photos,
    Videos,
}

#[derive(PartialEq, Clone, Copy)]
enum ExistingMode {
    Ask,
    Overwrite,
}

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Albums,
    NoAlbum,
    AllByYear,
}

enum ThumbnailEvent {
    Loaded { album_id: String, bytes: Vec<u8> },
    Failed,
}

enum DownloadEvent {
    Preparing(String),
    AlbumProgress {
        current: usize,
        total: usize,
        album_name: String,
    },
    Prepared(usize),
    Progress {
        current: usize,
        total: usize,
        file_name: String,
        downloaded: usize,
        skipped: usize,
        failed: usize,
        active: usize,
    },
    Finished {
        downloaded: usize,
        skipped: usize,
        failed: usize,
        conflicts: Vec<ConflictItem>,
        cancelled: bool,
    },
    Error(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SavedSettings {
    server: String,
    #[serde(default)]
    api_key: String,
    #[serde(default)]
    api_key_dpapi: String,
}

#[cfg(windows)]
fn protect_api_key(api_key: &str) -> Result<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use std::ptr::{null, null_mut};
    use winapi::shared::minwindef::DWORD;
    use winapi::um::dpapi::CryptProtectData;
    use winapi::um::winbase::LocalFree;
    use winapi::um::wincrypt::DATA_BLOB;
    const CRYPTPROTECT_UI_FORBIDDEN: DWORD = 0x1;

    let mut input = api_key.as_bytes().to_vec();
    let mut input_blob = DATA_BLOB {
        cbData: input.len() as DWORD,
        pbData: input.as_mut_ptr(),
    };
    let mut output_blob = DATA_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };

    let ok = unsafe {
        CryptProtectData(
            &mut input_blob,
            null(),
            null_mut(),
            null_mut(),
            null_mut(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output_blob,
        )
    };
    if ok == 0 {
        return Err("Windows DPAPI could not encrypt the API key.".to_string());
    }

    let encrypted = unsafe {
        std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize).to_vec()
    };
    unsafe {
        LocalFree(output_blob.pbData as *mut _);
    }
    Ok(STANDARD.encode(encrypted))
}

#[cfg(not(windows))]
fn protect_api_key(_api_key: &str) -> Result<String, String> {
    Err("DPAPI encryption is only available on Windows.".to_string())
}

#[cfg(windows)]
fn unprotect_api_key(encoded: &str) -> Result<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use std::ptr::null_mut;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::dpapi::CryptUnprotectData;
    use winapi::um::winbase::LocalFree;
    use winapi::um::wincrypt::DATA_BLOB;
    const CRYPTPROTECT_UI_FORBIDDEN: DWORD = 0x1;

    let mut encrypted = STANDARD
        .decode(encoded)
        .map_err(|e| format!("Invalid encrypted API key: {e}"))?;
    let mut input_blob = DATA_BLOB {
        cbData: encrypted.len() as DWORD,
        pbData: encrypted.as_mut_ptr(),
    };
    let mut output_blob = DATA_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };

    let ok = unsafe {
        CryptUnprotectData(
            &mut input_blob,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output_blob,
        )
    };
    if ok == 0 {
        return Err(
            "Windows DPAPI could not decrypt the API key for this Windows user.".to_string(),
        );
    }

    let decrypted = unsafe {
        std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize).to_vec()
    };
    unsafe {
        LocalFree(output_blob.pbData as *mut _);
    }
    String::from_utf8(decrypted).map_err(|e| format!("Decrypted API key is not valid UTF-8: {e}"))
}

#[cfg(not(windows))]
fn unprotect_api_key(_encoded: &str) -> Result<String, String> {
    Err("DPAPI decryption is only available on Windows.".to_string())
}

fn settings_path() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    Some(
        PathBuf::from(appdata)
            .join("Media_Backup_Manager")
            .join("settings.json"),
    )
}

fn legacy_settings_path() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    Some(
        PathBuf::from(appdata)
            .join("Immich_Backup_Manager")
            .join("settings.json"),
    )
}

fn load_saved_settings() -> SavedSettings {
    let Some(path) = settings_path() else {
        return SavedSettings::default();
    };

    let (data, migrated_from_legacy) = match fs::read_to_string(&path) {
        Ok(data) => (data, false),
        Err(_) => {
            let Some(old_path) = legacy_settings_path() else {
                return SavedSettings::default();
            };
            let Ok(data) = fs::read_to_string(&old_path) else {
                return SavedSettings::default();
            };
            (data, true)
        }
    };

    let mut settings: SavedSettings = serde_json::from_str(&data).unwrap_or_default();

    if !settings.api_key_dpapi.trim().is_empty() {
        settings.api_key = unprotect_api_key(&settings.api_key_dpapi).unwrap_or_default();
        if migrated_from_legacy && !settings.api_key.trim().is_empty() {
            let _ = save_settings(&settings.server, &settings.api_key);
        }
        return settings;
    }

    // Automatic one-time migration from older plaintext settings.
    if !settings.api_key.trim().is_empty() {
        let plaintext_key = settings.api_key.clone();
        if let Ok(encrypted) = protect_api_key(&plaintext_key) {
            settings.api_key_dpapi = encrypted;
            settings.api_key.clear();
            if let Ok(json) = serde_json::to_string_pretty(&settings) {
                let _ = fs::write(&path, json);
            }
            settings.api_key = plaintext_key;
        }
    }

    if migrated_from_legacy && !settings.api_key.trim().is_empty() {
        let _ = save_settings(&settings.server, &settings.api_key);
    }

    settings
}

fn save_settings(server: &str, api_key: &str) -> Result<(), String> {
    let path = settings_path().ok_or_else(|| "APPDATA-Ordner wurde nicht gefunden.".to_string())?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let settings = SavedSettings {
        server: server.to_string(),
        api_key: String::new(),
        api_key_dpapi: protect_api_key(api_key)?,
    };

    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn delete_saved_settings() -> Result<(), String> {
    let Some(path) = settings_path() else {
        return Ok(());
    };

    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

struct ImmichApp {
    server: String,
    api_key: String,
    show_key: bool,
    own_albums: bool,
    shared_albums: bool,
    deduplicate: bool,
    album_search: String,
    album_view: AlbumView,
    year_search: String,
    albums: Vec<SelectableAlbum>,
    thumbnail_receiver: Option<Receiver<ThumbnailEvent>>,
    header_logo_light_texture: Option<egui::TextureHandle>,
    header_logo_dark_texture: Option<egui::TextureHandle>,
    no_album_assets: Vec<Asset>,
    year_buckets: Vec<YearBucket>,

    all_assets: Vec<Asset>,
    all_year_buckets: Vec<YearBucket>,

    media_mode: MediaMode,
    existing_mode: ExistingMode,
    active_tab: ActiveTab,
    target_dir: String,
    status: String,
    progress: f32,

    parallel_downloads: usize,
    download_receiver: Option<Receiver<DownloadEvent>>,
    download_cancel: Option<Arc<AtomicBool>>,
    download_running: bool,
    download_popup: bool,
    download_protocol_popup: bool,
    protocol_selected_albums: usize,
    protocol_selected_years: usize,
    download_current: usize,
    download_total: usize,
    download_file: String,
    download_downloaded: usize,
    download_skipped: usize,
    download_failed: usize,
    download_active: usize,
    download_album_current: usize,
    download_album_total: usize,
    download_album_name: String,

    conflict_popup: bool,
    conflicts: Vec<ConflictItem>,
    conflict_index: usize,
    conflict_preview_key: String,
    conflict_local_texture: Option<egui::TextureHandle>,
    conflict_remote_texture: Option<egui::TextureHandle>,
    conflict_local_dims: Option<(u32, u32)>,
    conflict_remote_dims: Option<(u32, u32)>,
    info_popup: bool,
    settings_popup: bool,
    english: bool,
    dark_mode: bool,
}

impl Default for ImmichApp {
    fn default() -> Self {
        let saved = load_saved_settings();
        let server = if saved.server.trim().is_empty() {
            String::new()
        } else {
            saved.server
        };

        Self {
            server,
            api_key: saved.api_key,
            show_key: false,
            own_albums: true,
            shared_albums: true,
            deduplicate: true,
            album_search: String::new(),
            album_view: AlbumView::Covers,
            year_search: String::new(),
            albums: Vec::new(),
            thumbnail_receiver: None,
            header_logo_light_texture: None,
            header_logo_dark_texture: None,
            no_album_assets: Vec::new(),
            year_buckets: Vec::new(),

            all_assets: Vec::new(),
            all_year_buckets: Vec::new(),

            media_mode: MediaMode::All,
            existing_mode: ExistingMode::Ask,
            active_tab: ActiveTab::Albums,
            target_dir: String::new(),
            status: "Bereit.".to_owned(),
            progress: 0.0,
            parallel_downloads: 6,
            download_receiver: None,
            download_cancel: None,
            download_running: false,
            download_popup: false,
            download_protocol_popup: false,
            protocol_selected_albums: 0,
            protocol_selected_years: 0,
            download_current: 0,
            download_total: 0,
            download_file: String::new(),
            download_downloaded: 0,
            download_skipped: 0,
            download_failed: 0,
            download_active: 0,
            download_album_current: 0,
            download_album_total: 0,
            download_album_name: String::new(),
            conflict_popup: false,
            conflicts: Vec::new(),
            conflict_index: 0,
            conflict_preview_key: String::new(),
            conflict_local_texture: None,
            conflict_remote_texture: None,
            conflict_local_dims: None,
            conflict_remote_dims: None,
            info_popup: false,
            settings_popup: false,
            english: false,
            dark_mode: true,
        }
    }
}

impl ImmichApp {
    fn client(&self) -> Result<Client, String> {
        Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .map_err(|e| e.to_string())
    }

    fn base_url(&self) -> Result<String, String> {
        let s = self.server.trim().trim_end_matches('/').to_string();
        if s.is_empty() {
            return Err("Bitte Immich-Serveradresse eingeben.".to_string());
        }
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(s)
        } else {
            Ok(format!("http://{}", s))
        }
    }

    fn ensure_api_key(&self) -> Result<&str, String> {
        let key = self.api_key.trim();
        if key.is_empty() {
            Err("Bitte API-Key eingeben.".to_string())
        } else {
            Ok(key)
        }
    }

    fn apply_style(ctx: &egui::Context, dark_mode: bool) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 7.0);
        style.spacing.button_padding = egui::vec2(12.0, 7.0);
        style.spacing.scroll = egui::style::ScrollStyle::solid();
        style.spacing.scroll.bar_width = 11.0;
        style.spacing.scroll.handle_min_length = 56.0;
        style.spacing.scroll.bar_inner_margin = 5.0;
        style.spacing.scroll.bar_outer_margin = 2.0;
        style.visuals = if dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        if dark_mode {
            style.visuals.window_fill = egui::Color32::from_rgb(18, 24, 30);
            style.visuals.panel_fill = egui::Color32::from_rgb(15, 21, 27);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(12, 17, 22);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(24, 32, 40);
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(10, 159, 166);
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(24, 32, 40);
            style.visuals.widgets.inactive.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(51, 65, 76));
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(25, 74, 78);
            style.visuals.widgets.hovered.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(20, 203, 211));
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(12, 124, 131);
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(235, 241, 244));
        } else {
            style.visuals.window_fill = egui::Color32::from_rgb(248, 249, 251);
            style.visuals.panel_fill = egui::Color32::from_rgb(248, 249, 251);
            style.visuals.extreme_bg_color = egui::Color32::WHITE;
            style.visuals.faint_bg_color = egui::Color32::from_rgb(242, 244, 247);
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(10, 159, 166);
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(31, 36, 44));
        }
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(22.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(12.5, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        ctx.set_style(style);
    }

    fn panel_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            egui::Color32::from_rgb(18, 24, 30)
        } else {
            egui::Color32::WHITE
        }
    }

    fn header_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            egui::Color32::from_rgb(1, 4, 12)
        } else {
            egui::Color32::from_rgb(247, 248, 248)
        }
    }

    fn page_bg(&self) -> egui::Color32 {
        if self.dark_mode {
            egui::Color32::from_rgb(13, 19, 24)
        } else {
            egui::Color32::from_rgb(247, 248, 250)
        }
    }

    fn accent(&self) -> egui::Color32 {
        egui::Color32::from_rgb(10, 159, 166)
    }

    fn media_matches(asset: &Asset, mode: MediaMode) -> bool {
        match mode {
            MediaMode::All => true,
            MediaMode::Photos => asset.asset_type == "IMAGE",
            MediaMode::Videos => asset.asset_type == "VIDEO",
        }
    }

    fn load_albums(&mut self) -> Result<(), String> {
        let base = self.base_url()?;
        let key = self.ensure_api_key()?.to_string();
        let client = self.client()?;
        self.status = "Verbinde mit Immich ...".to_owned();

        let resp = client
            .get(format!("{}/api/albums", base))
            .header("x-api-key", &key)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?;

        let albums: Vec<Album> = resp.json().map_err(|e| e.to_string())?;
        let mut items: Vec<SelectableAlbum> = albums
            .into_iter()
            .filter(|a| (a.shared && self.shared_albums) || (!a.shared && self.own_albums))
            .map(|album| SelectableAlbum {
                album,
                selected: false,
                thumbnail: None,
            })
            .collect();

        items.sort_by(|a, b| {
            a.album
                .album_name
                .to_lowercase()
                .cmp(&b.album.album_name.to_lowercase())
        });
        let count = items.len();
        self.albums = items;
        self.status = format!("Verbindung erfolgreich. {} Album/Alben geladen.", count);
        self.start_thumbnail_loading(&base, &key, &client);
        Ok(())
    }

    fn start_thumbnail_loading(&mut self, base: &str, api_key: &str, client: &Client) {
        let jobs: Vec<(String, Option<String>)> = self
            .albums
            .iter()
            .map(|item| {
                (
                    item.album.id.clone(),
                    item.album.album_thumbnail_asset_id.clone(),
                )
            })
            .collect();

        if jobs.is_empty() {
            self.thumbnail_receiver = None;
            return;
        }

        let (tx, rx) = mpsc::channel::<ThumbnailEvent>();
        self.thumbnail_receiver = Some(rx);
        let queue = Arc::new(Mutex::new(VecDeque::from(jobs)));
        let worker_count = 8usize
            .min(queue.lock().map(|q| q.len()).unwrap_or(1))
            .max(1);

        for _ in 0..worker_count {
            let queue = Arc::clone(&queue);
            let tx = tx.clone();
            let client = client.clone();
            let base = base.to_string();
            let api_key = api_key.to_string();

            thread::spawn(move || loop {
                let job = queue.lock().ok().and_then(|mut q| q.pop_front());
                let Some((album_id, known_thumbnail_id)) = job else {
                    break;
                };

                let thumbnail_id =
                    if let Some(id) = known_thumbnail_id.filter(|id| !id.trim().is_empty()) {
                        Some(id)
                    } else {
                        client
                            .get(format!("{}/api/albums/{}", base, album_id))
                            .header("x-api-key", &api_key)
                            .send()
                            .and_then(|r| r.error_for_status())
                            .ok()
                            .and_then(|r| r.json::<serde_json::Value>().ok())
                            .and_then(|value| {
                                value
                                    .get("albumThumbnailAssetId")
                                    .and_then(|x| x.as_str())
                                    .map(str::to_string)
                                    .or_else(|| {
                                        value
                                            .get("assets")
                                            .and_then(|x| x.as_array())
                                            .and_then(|items| items.first())
                                            .and_then(|asset| asset.get("id"))
                                            .and_then(|x| x.as_str())
                                            .map(str::to_string)
                                    })
                            })
                    };

                let Some(thumbnail_id) = thumbnail_id else {
                    let _ = tx.send(ThumbnailEvent::Failed);
                    continue;
                };

                let result = client
                    .get(format!(
                        "{}/api/assets/{}/thumbnail?size=thumbnail",
                        base, thumbnail_id
                    ))
                    .header("x-api-key", &api_key)
                    .send()
                    .and_then(|r| r.error_for_status())
                    .and_then(|r| r.bytes());

                match result {
                    Ok(bytes) => {
                        let _ = tx.send(ThumbnailEvent::Loaded {
                            album_id,
                            bytes: bytes.to_vec(),
                        });
                    }
                    Err(_) => {
                        let _ = tx.send(ThumbnailEvent::Failed);
                    }
                }
            });
        }
    }

    fn poll_thumbnail_events(&mut self, ctx: &egui::Context) {
        let Some(receiver) = &self.thumbnail_receiver else {
            return;
        };
        let mut events = Vec::new();
        while let Ok(event) = receiver.try_recv() {
            events.push(event);
        }

        for event in events {
            match event {
                ThumbnailEvent::Loaded { album_id, bytes } => {
                    if let Ok(decoded) = image::load_from_memory(&bytes) {
                        let rgba = decoded.to_rgba8();
                        let size = [rgba.width() as usize, rgba.height() as usize];
                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                        let texture = ctx.load_texture(
                            format!("album_thumbnail_{}", album_id),
                            color_image,
                            egui::TextureOptions::LINEAR,
                        );
                        if let Some(item) = self.albums.iter_mut().find(|x| x.album.id == album_id)
                        {
                            item.thumbnail = Some(texture);
                        }
                    }
                }
                ThumbnailEvent::Failed => {}
            }
        }
    }

    fn load_no_album_years(&mut self) -> Result<(), String> {
        let base = self.base_url()?;
        let key = self.ensure_api_key()?.to_string();
        let client = self.client()?;
        self.no_album_assets.clear();
        self.year_buckets.clear();
        self.status = "Lade Fotos ohne Album nach Jahr ...".to_owned();

        let mut page = 1_i64;
        let mut seen_pages = HashSet::<i64>::new();

        loop {
            if !seen_pages.insert(page) {
                return Err(format!("Seitennavigation wiederholt Seite {}. Abbruch zum Schutz vor einer Endlosschleife.", page));
            }

            let body = json!({"isNotInAlbum": true, "page": page, "size": 1000});
            let resp = client
                .post(format!("{}/api/search/metadata", base))
                .header("x-api-key", &key)
                .json(&body)
                .send()
                .map_err(|e| e.to_string())?
                .error_for_status()
                .map_err(|e| e.to_string())?;

            let value: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
            let (items, next_page) = Self::search_items_and_next(&value);
            let assets: Vec<Asset> = serde_json::from_value(items).map_err(|e| e.to_string())?;
            let returned_count = assets.len();

            for asset in assets {
                if Self::media_matches(&asset, self.media_mode) {
                    self.no_album_assets.push(asset);
                }
            }

            if let Some(next) = Self::parse_next_page(next_page) {
                if next == page {
                    break;
                }
                page = next;
            } else {
                // Kein nextPage bedeutet laut API: letzte Seite.
                // returned_count wird nur für die Status-/Plausibilitätslogik behalten.
                let _ = returned_count;
                break;
            }
        }

        self.rebuild_year_buckets();
        self.status = format!(
            "{} Fotos/Videos ohne Album geladen, gruppiert in {} Jahresordner.",
            self.no_album_assets.len(),
            self.year_buckets.len()
        );
        Ok(())
    }

    fn load_all_assets_by_year(&mut self) -> Result<(), String> {
        let base = self.base_url()?;
        let key = self.ensure_api_key()?.to_string();
        let client = self.client()?;

        self.all_assets.clear();
        self.all_year_buckets.clear();
        self.status = "Lade alle Fotos nach Jahr ...".to_owned();

        let mut page = 1_i64;
        let mut seen_pages = HashSet::<i64>::new();

        loop {
            if !seen_pages.insert(page) {
                return Err(format!(
                    "Seitennavigation wiederholt Seite {}. Abbruch zum Schutz vor einer Endlosschleife.",
                    page
                ));
            }

            let body = json!({
                "page": page,
                "size": 1000
            });

            let resp = client
                .post(format!("{}/api/search/metadata", base))
                .header("x-api-key", &key)
                .json(&body)
                .send()
                .map_err(|e| e.to_string())?
                .error_for_status()
                .map_err(|e| e.to_string())?;

            let value: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
            let (items, next_page) = Self::search_items_and_next(&value);
            let assets: Vec<Asset> = serde_json::from_value(items).map_err(|e| e.to_string())?;

            for asset in assets {
                if Self::media_matches(&asset, self.media_mode) {
                    self.all_assets.push(asset);
                }
            }

            if let Some(next) = Self::parse_next_page(next_page) {
                if next == page {
                    break;
                }
                page = next;
            } else {
                break;
            }
        }

        self.rebuild_all_year_buckets();

        self.status = format!(
            "{} Fotos/Videos insgesamt geladen, gruppiert in {} Jahresordner.",
            self.all_assets.len(),
            self.all_year_buckets.len()
        );

        Ok(())
    }

    fn rebuild_all_year_buckets(&mut self) {
        let selected: HashSet<String> = self
            .all_year_buckets
            .iter()
            .filter(|x| x.selected)
            .map(|x| x.year.clone())
            .collect();

        let mut map: BTreeMap<String, (usize, i64)> = BTreeMap::new();

        for asset in &self.all_assets {
            let year = Self::asset_year(asset);
            let entry = map.entry(year).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += asset.file_size_in_byte.max(0);
        }

        self.all_year_buckets = map
            .into_iter()
            .rev()
            .map(|(year, (count, total_size))| YearBucket {
                selected: selected.contains(&year),
                year,
                count,
                total_size,
            })
            .collect();
    }

    fn search_items_and_next(
        value: &serde_json::Value,
    ) -> (serde_json::Value, Option<serde_json::Value>) {
        if let Some(assets) = value.get("assets") {
            (
                assets.get("items").cloned().unwrap_or_else(|| json!([])),
                assets.get("nextPage").cloned(),
            )
        } else {
            (
                value.get("items").cloned().unwrap_or_else(|| json!([])),
                value.get("nextPage").cloned(),
            )
        }
    }

    fn parse_next_page(value: Option<serde_json::Value>) -> Option<i64> {
        match value {
            Some(serde_json::Value::Number(n)) => n.as_i64(),
            Some(serde_json::Value::String(s)) => s.parse::<i64>().ok(),
            _ => None,
        }
    }

    fn rebuild_year_buckets(&mut self) {
        let selected: HashSet<String> = self
            .year_buckets
            .iter()
            .filter(|x| x.selected)
            .map(|x| x.year.clone())
            .collect();
        let mut map: BTreeMap<String, (usize, i64)> = BTreeMap::new();
        for asset in &self.no_album_assets {
            let year = Self::asset_year(asset);
            let entry = map.entry(year).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += asset.file_size_in_byte.max(0);
        }
        self.year_buckets = map
            .into_iter()
            .rev()
            .map(|(year, (count, total_size))| YearBucket {
                selected: selected.contains(&year),
                year,
                count,
                total_size,
            })
            .collect();
    }

    fn asset_file_name(asset: &Asset) -> String {
        if !asset.original_file_name.trim().is_empty() {
            return Path::new(&asset.original_file_name)
                .file_name()
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_else(|| asset.original_file_name.clone());
        }
        if !asset.device_asset_id.trim().is_empty() {
            return Path::new(&asset.device_asset_id)
                .file_name()
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_else(|| asset.device_asset_id.clone());
        }
        format!("{}.bin", asset.id)
    }

    fn asset_year(asset: &Asset) -> String {
        let dt = asset.local_date_time.trim();
        if dt.len() >= 4 && dt.chars().take(4).all(|c| c.is_ascii_digit()) {
            return dt[0..4].to_string();
        }
        let name = Self::asset_file_name(asset);
        let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
        for i in 0..digits.len().saturating_sub(3) {
            let part = &digits[i..i + 4];
            if let Ok(year) = part.parse::<i32>() {
                if (1900..=2100).contains(&year) {
                    return part.to_string();
                }
            }
        }
        "Unbekannt".to_string()
    }

    fn sanitize_file_name(name: &str) -> String {
        let bad = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        let mut out: String = name
            .chars()
            .map(|c| if bad.contains(&c) || c < ' ' { '_' } else { c })
            .collect();
        while out.ends_with('.') || out.ends_with(' ') {
            out.pop();
        }
        if out.is_empty() {
            "Unbenannt".to_string()
        } else {
            out
        }
    }

    fn duplicate_key(asset: &Asset) -> String {
        if !asset.checksum.trim().is_empty() {
            format!("checksum:{}", asset.checksum)
        } else {
            format!(
                "fallback:{}|{}|{}",
                Self::asset_file_name(asset).to_lowercase(),
                asset.local_date_time,
                asset.asset_type
            )
        }
    }

    fn deduplicate_jobs(jobs: Vec<DownloadJob>) -> Vec<DownloadJob> {
        let mut best: HashMap<String, DownloadJob> = HashMap::new();
        for job in jobs {
            let key = Self::duplicate_key(&job.asset);
            match best.get(&key) {
                Some(existing)
                    if existing.asset.file_size_in_byte >= job.asset.file_size_in_byte => {}
                _ => {
                    best.insert(key, job);
                }
            }
        }
        best.into_values().collect()
    }

    fn deduplicate_queue_jobs(jobs: Vec<DownloadJob>) -> Vec<DownloadJob> {
        // Technische Pflicht-Deduplizierung vor parallelen Downloads:
        // 1) dieselbe Immich-Asset-ID darf nur einmal pro Zielpfad in die Queue
        // 2) derselbe Zielpfad darf nur einmal gleichzeitig verarbeitet werden
        // Dadurch kann ein Worker eine Datei nicht herunterladen, während ein
        // anderer Worker dieselbe Datei kurz danach fälschlich als "vorhanden"
        // erkennt.
        let mut by_asset_and_path: HashMap<String, DownloadJob> = HashMap::new();

        for job in jobs {
            let file_name = Self::sanitize_file_name(&Self::asset_file_name(&job.asset));
            let target_key = format!(
                "{}\\{}",
                job.folder_name.to_lowercase(),
                file_name.to_lowercase()
            );
            let key = format!("{}|{}", job.asset.id, target_key);

            match by_asset_and_path.get(&key) {
                Some(existing)
                    if existing.asset.file_size_in_byte >= job.asset.file_size_in_byte => {}
                _ => {
                    by_asset_and_path.insert(key, job);
                }
            }
        }

        // Zweite Sicherung nur nach Zielpfad. Falls zwei verschiedene Asset-IDs
        // auf exakt denselben Dateinamen im selben Ordner zeigen, bleibt die
        // größere bekannte Version in der Queue.
        let mut by_target_path: HashMap<String, DownloadJob> = HashMap::new();
        for job in by_asset_and_path.into_values() {
            let file_name = Self::sanitize_file_name(&Self::asset_file_name(&job.asset));
            let target_key = format!(
                "{}\\{}",
                job.folder_name.to_lowercase(),
                file_name.to_lowercase()
            );

            match by_target_path.get(&target_key) {
                Some(existing)
                    if existing.asset.file_size_in_byte >= job.asset.file_size_in_byte => {}
                _ => {
                    by_target_path.insert(target_key, job);
                }
            }
        }

        by_target_path.into_values().collect()
    }

    fn format_bytes_i64(bytes: i64) -> String {
        if bytes <= 0 {
            "unbekannt".to_string()
        } else {
            Self::format_bytes_u64(bytes as u64)
        }
    }

    fn format_bytes_u64(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
        } else {
            format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
        }
    }

    fn remote_original_size(
        client: &Client,
        base: &str,
        api_key: &str,
        asset_id: &str,
    ) -> Option<u64> {
        let url = format!("{}/api/assets/{}/original", base, asset_id);

        // Zuerst HEAD versuchen: schnell und ohne Dateidaten.
        if let Ok(response) = client.head(&url).header("x-api-key", api_key).send() {
            if response.status().is_success() {
                if let Some(len) = response.content_length() {
                    if len > 0 {
                        return Some(len);
                    }
                }
            }
        }

        // Falls HEAD nicht unterstützt wird: nur das erste Byte anfordern.
        // Aus "Content-Range: bytes 0-0/GESAMT" lässt sich die echte
        // Größe bestimmen, ohne die komplette Datei herunterzuladen.
        if let Ok(response) = client
            .get(&url)
            .header("x-api-key", api_key)
            .header(RANGE, "bytes=0-0")
            .send()
        {
            if response.status().is_success()
                || response.status() == reqwest::StatusCode::PARTIAL_CONTENT
            {
                if let Some(value) = response.headers().get(CONTENT_RANGE) {
                    if let Ok(text) = value.to_str() {
                        if let Some(total) = text.rsplit('/').next() {
                            if total != "*" {
                                if let Ok(size) = total.parse::<u64>() {
                                    if size > 0 {
                                        return Some(size);
                                    }
                                }
                            }
                        }
                    }
                }

                // Manche Server ignorieren Range und liefern bei GET trotzdem
                // einen korrekten Content-Length-Header.
                if let Some(len) = response.content_length() {
                    if len > 1 {
                        return Some(len);
                    }
                }
            }
        }

        None
    }

    fn selected_album_count(&self) -> usize {
        self.albums.iter().filter(|x| x.selected).count()
    }

    fn selected_all_year_count(&self) -> usize {
        self.all_year_buckets.iter().filter(|x| x.selected).count()
    }

    fn selected_year_count(&self) -> usize {
        self.year_buckets.iter().filter(|x| x.selected).count()
    }

    fn start_background_download(&mut self) -> Result<(), String> {
        if self.download_running {
            return Err("Ein Download läuft bereits.".to_string());
        }

        let target_dir = self.target_dir.trim().to_string();
        if target_dir.is_empty() {
            return Err("Bitte zuerst ein Download-Verzeichnis auswählen.".to_string());
        }

        let base = self.base_url()?;
        let api_key = self.ensure_api_key()?.to_string();
        let selected_albums: Vec<Album> = if self.active_tab == ActiveTab::Albums {
            self.albums
                .iter()
                .filter(|x| x.selected)
                .map(|x| x.album.clone())
                .collect()
        } else {
            Vec::new()
        };
        let selected_years: Vec<String> = if self.active_tab == ActiveTab::NoAlbum {
            self.year_buckets
                .iter()
                .filter(|x| x.selected)
                .map(|x| x.year.clone())
                .collect()
        } else {
            Vec::new()
        };
        let selected_all_years: Vec<String> = if self.active_tab == ActiveTab::AllByYear {
            self.all_year_buckets
                .iter()
                .filter(|x| x.selected)
                .map(|x| x.year.clone())
                .collect()
        } else {
            Vec::new()
        };

        if selected_albums.is_empty() && selected_years.is_empty() && selected_all_years.is_empty()
        {
            return Err(if self.english {
                "Select at least one album or year folder."
            } else {
                "Bitte mindestens ein Album oder einen Jahresordner auswählen."
            }
            .to_string());
        }

        let no_album_assets = self.no_album_assets.clone();
        let all_assets = self.all_assets.clone();
        let media_mode = self.media_mode;
        let existing_mode = self.existing_mode;
        let deduplicate = self.deduplicate;
        let parallel_downloads = self.parallel_downloads.max(1);
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_for_thread = Arc::clone(&cancel_flag);

        let (tx, rx) = mpsc::channel::<DownloadEvent>();
        self.download_receiver = Some(rx);
        self.download_cancel = Some(cancel_flag);
        self.download_running = true;
        self.download_popup = true;
        self.download_current = 0;
        self.download_total = 0;
        self.download_file = "Download wird vorbereitet ...".to_string();
        self.download_downloaded = 0;
        self.download_skipped = 0;
        self.download_failed = 0;
        self.download_active = 0;
        self.download_album_current = 0;
        self.download_album_total = selected_albums.len();
        self.download_album_name.clear();
        self.progress = 0.0;
        self.status = "Download gestartet ...".to_string();

        thread::spawn(move || {
            let client = match Client::builder().timeout(Duration::from_secs(600)).build() {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(DownloadEvent::Error(e.to_string()));
                    return;
                }
            };

            if let Err(e) = fs::create_dir_all(&target_dir) {
                let _ = tx.send(DownloadEvent::Error(format!(
                    "Zielordner konnte nicht erstellt werden: {}",
                    e
                )));
                return;
            }

            let mut jobs: Vec<DownloadJob> = Vec::new();
            let album_total = selected_albums.len();

            for (album_index, album) in selected_albums.into_iter().enumerate() {
                if cancel_for_thread.load(Ordering::Relaxed) {
                    let _ = tx.send(DownloadEvent::Finished {
                        downloaded: 0,
                        skipped: 0,
                        failed: 0,
                        conflicts: Vec::new(),
                        cancelled: true,
                    });
                    return;
                }

                let _ = tx.send(DownloadEvent::AlbumProgress {
                    current: album_index + 1,
                    total: album_total,
                    album_name: album.album_name.clone(),
                });
                let _ = tx.send(DownloadEvent::Preparing(format!(
                    "Lese Album: {}",
                    album.album_name
                )));

                let mut page = 1_i64;
                loop {
                    if cancel_for_thread.load(Ordering::Relaxed) {
                        break;
                    }

                    let body = json!({"albumIds": [album.id.clone()], "page": page, "size": 1000});
                    let resp = client
                        .post(format!("{}/api/search/metadata", base))
                        .header("x-api-key", &api_key)
                        .json(&body)
                        .send()
                        .and_then(|r| r.error_for_status());

                    let resp = match resp {
                        Ok(v) => v,
                        Err(e) => {
                            let _ = tx.send(DownloadEvent::Error(format!(
                                "Album '{}' konnte nicht geladen werden: {}",
                                album.album_name, e
                            )));
                            return;
                        }
                    };

                    let value: serde_json::Value = match resp.json() {
                        Ok(v) => v,
                        Err(e) => {
                            let _ = tx.send(DownloadEvent::Error(format!(
                                "Album '{}' lieferte ungültige Daten: {}",
                                album.album_name, e
                            )));
                            return;
                        }
                    };

                    let (items, next_page) = Self::search_items_and_next(&value);
                    let assets: Vec<Asset> = match serde_json::from_value(items) {
                        Ok(v) => v,
                        Err(e) => {
                            let _ = tx.send(DownloadEvent::Error(format!(
                                "Album '{}' konnte nicht ausgewertet werden: {}",
                                album.album_name, e
                            )));
                            return;
                        }
                    };

                    for asset in assets {
                        if Self::media_matches(&asset, media_mode) {
                            jobs.push(DownloadJob {
                                asset,
                                folder_name: Self::sanitize_file_name(&album.album_name),
                                group_name: album.album_name.clone(),
                                album_position: Some((album_index + 1, album_total)),
                            });
                        }
                    }

                    if let Some(next) = Self::parse_next_page(next_page) {
                        page = next;
                    } else {
                        break;
                    }
                }
            }

            for year in selected_years {
                for asset in &no_album_assets {
                    if Self::asset_year(asset) == year && Self::media_matches(asset, media_mode) {
                        jobs.push(DownloadJob {
                            asset: asset.clone(),
                            folder_name: format!("Ohne Album\\{}", year),
                            group_name: format!("Ohne Album {}", year),
                            album_position: None,
                        });
                    }
                }
            }

            for year in selected_all_years {
                for asset in &all_assets {
                    if Self::asset_year(asset) == year && Self::media_matches(asset, media_mode) {
                        jobs.push(DownloadJob {
                            asset: asset.clone(),
                            folder_name: format!("Alle Fotos nach Jahr\\{}", year),
                            group_name: format!("Alle Fotos {}", year),
                            album_position: None,
                        });
                    }
                }
            }

            if deduplicate {
                jobs = Self::deduplicate_jobs(jobs);
            }

            // Unabhängig von der Benutzeroption muss die parallele Queue frei
            // von doppelten Asset-/Zielpfad-Einträgen sein.
            jobs = Self::deduplicate_queue_jobs(jobs);

            if jobs.is_empty() {
                let _ = tx.send(DownloadEvent::Error(
                    "Die Auswahl enthält keine passenden Dateien.".to_string(),
                ));
                return;
            }

            let total = jobs.len();
            let _ = tx.send(DownloadEvent::Prepared(total));

            let mut group_order: Vec<String> = Vec::new();
            let mut groups: HashMap<String, Vec<DownloadJob>> = HashMap::new();
            for job in jobs {
                if !groups.contains_key(&job.group_name) {
                    group_order.push(job.group_name.clone());
                }
                groups.entry(job.group_name.clone()).or_default().push(job);
            }

            let downloaded = Arc::new(AtomicUsize::new(0));
            let skipped = Arc::new(AtomicUsize::new(0));
            let failed = Arc::new(AtomicUsize::new(0));
            let completed = Arc::new(AtomicUsize::new(0));
            let active = Arc::new(AtomicUsize::new(0));
            let conflicts = Arc::new(Mutex::new(Vec::<ConflictItem>::new()));
            let errors = Arc::new(Mutex::new(Vec::<String>::new()));

            for group_name in group_order {
                if cancel_for_thread.load(Ordering::Relaxed) {
                    break;
                }

                let group_jobs = groups.remove(&group_name).unwrap_or_default();
                if let Some((current, album_total)) =
                    group_jobs.first().and_then(|j| j.album_position)
                {
                    let _ = tx.send(DownloadEvent::AlbumProgress {
                        current,
                        total: album_total,
                        album_name: group_name.clone(),
                    });
                } else {
                    let _ = tx.send(DownloadEvent::Preparing(format!(
                        "Jahresordner: {}",
                        group_name
                    )));
                }

                Self::download_group_parallel(
                    group_jobs,
                    parallel_downloads,
                    &client,
                    &base,
                    &api_key,
                    &target_dir,
                    existing_mode,
                    &tx,
                    total,
                    &cancel_for_thread,
                    &downloaded,
                    &skipped,
                    &failed,
                    &completed,
                    &active,
                    &conflicts,
                    &errors,
                );
            }

            if let Ok(errors) = errors.lock() {
                if !errors.is_empty() {
                    let error_path = Path::new(&target_dir).join("Immich_Download_Fehler.txt");
                    if let Ok(mut file) = fs::File::create(error_path) {
                        for line in errors.iter() {
                            let _ = writeln!(file, "{}", line);
                        }
                    }
                }
            }

            let conflict_list = conflicts.lock().map(|x| x.clone()).unwrap_or_default();
            let _ = tx.send(DownloadEvent::Finished {
                downloaded: downloaded.load(Ordering::Relaxed),
                skipped: skipped.load(Ordering::Relaxed),
                failed: failed.load(Ordering::Relaxed),
                conflicts: conflict_list,
                cancelled: cancel_for_thread.load(Ordering::Relaxed),
            });
        });

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn download_group_parallel(
        jobs: Vec<DownloadJob>,
        parallel_downloads: usize,
        client: &Client,
        base: &str,
        api_key: &str,
        target_dir: &str,
        existing_mode: ExistingMode,
        tx: &Sender<DownloadEvent>,
        total: usize,
        cancel: &Arc<AtomicBool>,
        downloaded: &Arc<AtomicUsize>,
        skipped: &Arc<AtomicUsize>,
        failed: &Arc<AtomicUsize>,
        completed: &Arc<AtomicUsize>,
        active: &Arc<AtomicUsize>,
        conflicts: &Arc<Mutex<Vec<ConflictItem>>>,
        errors: &Arc<Mutex<Vec<String>>>,
    ) {
        let queue = Arc::new(Mutex::new(VecDeque::from(jobs)));
        let mut handles = Vec::new();
        let worker_count = parallel_downloads
            .min(queue.lock().map(|q| q.len()).unwrap_or(1))
            .max(1);

        for _ in 0..worker_count {
            let queue = Arc::clone(&queue);
            let client = client.clone();
            let base = base.to_string();
            let api_key = api_key.to_string();
            let target_dir = target_dir.to_string();
            let tx = tx.clone();
            let cancel = Arc::clone(cancel);
            let downloaded = Arc::clone(downloaded);
            let skipped = Arc::clone(skipped);
            let failed = Arc::clone(failed);
            let completed = Arc::clone(completed);
            let active = Arc::clone(active);
            let conflicts = Arc::clone(conflicts);
            let errors = Arc::clone(errors);

            handles.push(thread::spawn(move || {
                loop {
                    if cancel.load(Ordering::Relaxed) {
                        break;
                    }

                    let job = {
                        let mut queue = match queue.lock() {
                            Ok(q) => q,
                            Err(_) => break,
                        };
                        queue.pop_front()
                    };

                    let Some(job) = job else {
                        break;
                    };
                    active.fetch_add(1, Ordering::Relaxed);
                    let file_name = Self::asset_file_name(&job.asset);
                    let folder = Path::new(&target_dir).join(&job.folder_name);

                    if let Err(e) = fs::create_dir_all(&folder) {
                        failed.fetch_add(1, Ordering::Relaxed);
                        if let Ok(mut err) = errors.lock() {
                            err.push(format!("{} : {}", file_name, e));
                        }
                    } else {
                        let target_path = folder.join(Self::sanitize_file_name(&file_name));

                        if target_path.exists() && existing_mode == ExistingMode::Ask {
                            let local_size =
                                fs::metadata(&target_path).map(|m| m.len()).unwrap_or(0);

                            // Tatsächliche Größe der Immich-Originaldatei ermitteln.
                            let remote_size =
                                Self::remote_original_size(&client, &base, &api_key, &job.asset.id);

                            // Gleich große Dateien sind für diesen Workflow bereits
                            // vollständig vorhanden und werden automatisch übersprungen.
                            // Sie erscheinen NICHT mehr im Konfliktfenster.
                            if remote_size == Some(local_size) {
                                skipped.fetch_add(1, Ordering::Relaxed);
                            } else {
                                if let Ok(mut list) = conflicts.lock() {
                                    list.push(ConflictItem {
                                        job: job.clone(),
                                        selected: false,
                                        local_size,
                                        remote_size,
                                    });
                                }
                                skipped.fetch_add(1, Ordering::Relaxed);
                            }
                        } else if !cancel.load(Ordering::Relaxed) {
                            let result = client
                                .get(format!("{}/api/assets/{}/original", base, job.asset.id))
                                .header("x-api-key", &api_key)
                                .send()
                                .and_then(|r| r.error_for_status());

                            match result {
                                Ok(mut response) => match fs::File::create(&target_path) {
                                    Ok(mut file) => match response.copy_to(&mut file) {
                                        Ok(_) => {
                                            downloaded.fetch_add(1, Ordering::Relaxed);
                                        }
                                        Err(e) => {
                                            failed.fetch_add(1, Ordering::Relaxed);
                                            if let Ok(mut err) = errors.lock() {
                                                err.push(format!("{} : {}", file_name, e));
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        failed.fetch_add(1, Ordering::Relaxed);
                                        if let Ok(mut err) = errors.lock() {
                                            err.push(format!("{} : {}", file_name, e));
                                        }
                                    }
                                },
                                Err(e) => {
                                    failed.fetch_add(1, Ordering::Relaxed);
                                    if let Ok(mut err) = errors.lock() {
                                        err.push(format!("{} : {}", file_name, e));
                                    }
                                }
                            }
                        }
                    }

                    active.fetch_sub(1, Ordering::Relaxed);
                    let current = completed.fetch_add(1, Ordering::Relaxed) + 1;
                    let _ = tx.send(DownloadEvent::Progress {
                        current,
                        total,
                        file_name,
                        downloaded: downloaded.load(Ordering::Relaxed),
                        skipped: skipped.load(Ordering::Relaxed),
                        failed: failed.load(Ordering::Relaxed),
                        active: active.load(Ordering::Relaxed),
                    });
                }
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    fn start_conflict_overwrite(&mut self, jobs: Vec<DownloadJob>) -> Result<(), String> {
        if jobs.is_empty() {
            return Err("Keine Dateien zum Überschreiben ausgewählt.".to_string());
        }
        if self.download_running {
            return Err("Ein Download läuft bereits.".to_string());
        }

        let base = self.base_url()?;
        let api_key = self.ensure_api_key()?.to_string();
        let target_dir = self.target_dir.clone();
        let parallel_downloads = self.parallel_downloads.max(1);
        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_thread = Arc::clone(&cancel);
        let (tx, rx) = mpsc::channel::<DownloadEvent>();

        self.download_receiver = Some(rx);
        self.download_cancel = Some(cancel);
        self.download_running = true;
        self.download_popup = true;
        self.conflict_popup = false;
        self.download_current = 0;
        self.download_total = jobs.len();
        self.download_file = "Vorhandene Dateien werden überschrieben ...".to_string();
        self.download_downloaded = 0;
        self.download_skipped = 0;
        self.download_failed = 0;
        self.download_active = 0;
        self.progress = 0.0;

        thread::spawn(move || {
            let client = match Client::builder().timeout(Duration::from_secs(600)).build() {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(DownloadEvent::Error(e.to_string()));
                    return;
                }
            };
            let total = jobs.len();
            let _ = tx.send(DownloadEvent::Prepared(total));
            let downloaded = Arc::new(AtomicUsize::new(0));
            let skipped = Arc::new(AtomicUsize::new(0));
            let failed = Arc::new(AtomicUsize::new(0));
            let completed = Arc::new(AtomicUsize::new(0));
            let active = Arc::new(AtomicUsize::new(0));
            let conflicts = Arc::new(Mutex::new(Vec::<ConflictItem>::new()));
            let errors = Arc::new(Mutex::new(Vec::<String>::new()));

            Self::download_group_parallel(
                jobs,
                parallel_downloads,
                &client,
                &base,
                &api_key,
                &target_dir,
                ExistingMode::Overwrite,
                &tx,
                total,
                &cancel_thread,
                &downloaded,
                &skipped,
                &failed,
                &completed,
                &active,
                &conflicts,
                &errors,
            );

            let _ = tx.send(DownloadEvent::Finished {
                downloaded: downloaded.load(Ordering::Relaxed),
                skipped: 0,
                failed: failed.load(Ordering::Relaxed),
                conflicts: Vec::new(),
                cancelled: cancel_thread.load(Ordering::Relaxed),
            });
        });

        Ok(())
    }

    fn poll_download_events(&mut self) {
        let mut pending = Vec::new();
        if let Some(rx) = &self.download_receiver {
            while let Ok(event) = rx.try_recv() {
                pending.push(event);
            }
        }

        for event in pending {
            match event {
                DownloadEvent::Preparing(text) => {
                    self.download_file = text.clone();
                    self.status = text;
                }
                DownloadEvent::AlbumProgress {
                    current,
                    total,
                    album_name,
                } => {
                    self.download_album_current = current;
                    self.download_album_total = total;
                    self.download_album_name = album_name.clone();
                    self.download_file = if self.english {
                        format!("Album {} of {}: {}", current, total, album_name)
                    } else {
                        format!("Album {} von {}: {}", current, total, album_name)
                    };
                }
                DownloadEvent::Prepared(total) => {
                    // Jeder neue Download beginnt sichtbar bei 0 %.
                    self.download_total = total;
                    self.download_current = 0;
                    self.download_downloaded = 0;
                    self.download_skipped = 0;
                    self.download_failed = 0;
                    self.download_active = 0;
                    self.progress = 0.0;
                }
                DownloadEvent::Progress {
                    current,
                    total,
                    file_name,
                    downloaded,
                    skipped,
                    failed,
                    active,
                } => {
                    self.download_current = current;
                    self.download_total = total;
                    self.download_file = file_name;
                    self.download_downloaded = downloaded;
                    self.download_skipped = skipped;
                    self.download_failed = failed;
                    self.download_active = active;
                    self.progress = if total > 0 {
                        current as f32 / total as f32
                    } else {
                        0.0
                    };
                }
                DownloadEvent::Finished {
                    downloaded,
                    skipped,
                    failed,
                    conflicts,
                    cancelled,
                } => {
                    self.download_running = false;
                    self.download_downloaded = downloaded;
                    self.download_skipped = skipped;
                    self.download_failed = failed;
                    self.download_active = 0;
                    if !cancelled {
                        self.progress = 1.0;
                    }
                    self.status = if cancelled {
                        if self.english {
                            format!(
                                "Cancelled. Downloaded: {} | Existing: {} | Errors: {}",
                                downloaded, skipped, failed
                            )
                        } else {
                            format!(
                                "Abgebrochen. Heruntergeladen: {} | Vorhanden: {} | Fehler: {}",
                                downloaded, skipped, failed
                            )
                        }
                    } else if self.english {
                        format!(
                            "Finished. Downloaded: {} | Existing: {} | Errors: {}",
                            downloaded, skipped, failed
                        )
                    } else {
                        format!(
                            "Fertig. Heruntergeladen: {} | Vorhanden: {} | Fehler: {}",
                            downloaded, skipped, failed
                        )
                    };
                    self.download_file = if cancelled {
                        if self.english {
                            "Download cancelled.".to_string()
                        } else {
                            "Download abgebrochen.".to_string()
                        }
                    } else if self.english {
                        "Download completed.".to_string()
                    } else {
                        "Download abgeschlossen.".to_string()
                    };
                    self.download_popup = false;
                    self.protocol_selected_albums = self.selected_album_count();
                    self.protocol_selected_years =
                        self.selected_year_count() + self.selected_all_year_count();
                    self.download_protocol_popup = true;
                    self.conflicts = conflicts;
                    if !self.conflicts.is_empty() && !cancelled {
                        self.conflict_index = 0;
                        self.conflict_preview_key.clear();
                        self.conflict_local_texture = None;
                        self.conflict_remote_texture = None;
                        self.conflict_local_dims = None;
                        self.conflict_remote_dims = None;
                        self.conflict_popup = true;
                    }
                }
                DownloadEvent::Error(msg) => {
                    self.download_running = false;
                    self.download_file = format!("Fehler: {}", msg);
                    self.status = self.download_file.clone();
                }
            }
        }

        if !self.download_running {
            self.download_receiver = None;
            self.download_cancel = None;
        }
    }

    fn header_logo_texture(&self) -> Option<&egui::TextureHandle> {
        if self.dark_mode {
            self.header_logo_dark_texture.as_ref()
        } else {
            self.header_logo_light_texture.as_ref()
        }
    }

    fn draw_header_logo(&self, ui: &egui::Ui, rect: egui::Rect) {
        let icon_size = HEADER_ICON_SIZE.min(rect.height());
        if let Some(texture) = self.header_logo_texture() {
            let icon_rect = egui::Rect::from_center_size(
                egui::pos2(rect.left() + icon_size / 2.0, rect.center().y),
                egui::vec2(icon_size, icon_size),
            );
            egui::Image::new(texture).paint_at(ui, icon_rect);
        }

        let text_left = rect.left() + icon_size + 8.0;
        let title_color = if self.dark_mode {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(3, 45, 92)
        };
        let subtitle_color = egui::Color32::from_rgb(0, 142, 240);
        let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
        let title_font = egui::FontId::proportional(17.5);
        let subtitle_font = egui::FontId::proportional(10.5);

        painter.text(
            egui::pos2(text_left, rect.center().y - 24.0),
            egui::Align2::LEFT_TOP,
            "Media Backup",
            title_font.clone(),
            title_color,
        );
        painter.text(
            egui::pos2(text_left, rect.center().y - 5.5),
            egui::Align2::LEFT_TOP,
            "Manager",
            title_font,
            title_color,
        );
        painter.text(
            egui::pos2(text_left, rect.center().y + 17.0),
            egui::Align2::LEFT_TOP,
            "for Immich",
            subtitle_font,
            subtitle_color,
        );
    }

    fn ui_main_panels(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("footer_panel")
            .exact_height(34.0)
            .frame(
                egui::Frame::none()
                    .fill(self.panel_bg())
                    .inner_margin(egui::Margin::symmetric(16.0, 7.0)),
            )
            .show(ctx, |ui| self.ui_footer(ui));

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .exact_height(HEADER_HEIGHT)
            .frame(egui::Frame::none().fill(self.header_bg()))
            .show(ctx, |ui| self.ui_header(ui));

        egui::SidePanel::right("download_settings_panel")
            .resizable(false)
            .exact_width(295.0)
            .frame(
                egui::Frame::none()
                    .fill(self.page_bg())
                    .inner_margin(egui::Margin {
                        left: 10.0,
                        right: 20.0,
                        top: 10.0,
                        bottom: 10.0,
                    }),
            )
            .show(ctx, |ui| {
                // Reserve the action before laying out the scrollable settings above it.
                egui::TopBottomPanel::bottom("download_action_panel")
                    .exact_height(48.0)
                    .frame(egui::Frame::none())
                    .show_inside(ui, |ui| {
                        self.ui_download_button(ui);
                    });
                egui::ScrollArea::vertical()
                    .id_salt("download_settings_scroll")
                    .min_scrolled_height(0.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| self.ui_download_settings_panel(ui));
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(self.page_bg())
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0)),
            )
            .show(ctx, |ui| {
                self.ui_tabs(ui);
            });
    }

    fn ui_header(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        egui::Frame::none()
            .fill(self.header_bg())
            .inner_margin(egui::Margin::symmetric(18.0, 6.0))
            .show(ui, |ui| {
                let width = ui.available_width();
                let logo_size = egui::vec2(HEADER_LOGO_WIDTH, 64.0);
                let actions_width = 360.0;
                let gap = 16.0;
                let text_width = (width - logo_size.x - actions_width - 2.0 * gap).max(1.0);
                let description = if en {
                    concat!(
                        "Download original photos and videos from Immich\n",
                        "for backups by album and year"
                    )
                } else {
                    concat!(
                        "Originalfotos und -videos aus Immich\n",
                        "für Backups nach Alben und Jahren herunterladen"
                    )
                };
                let text_color = ui.visuals().text_color();
                let mut description_font_size = 17.0;
                let galley = loop {
                    let mut job = egui::text::LayoutJob::simple(
                        description.to_owned(),
                        egui::FontId::proportional(description_font_size),
                        text_color,
                        f32::INFINITY,
                    );
                    job.sections[0].format.line_height = Some(description_font_size + 4.0);
                    let galley = ui.fonts(|fonts| fonts.layout_job(job));
                    if galley.size().x <= text_width || description_font_size <= 9.0 {
                        break galley;
                    }
                    description_font_size -= 0.5;
                };
                let height = (HEADER_HEIGHT - 12.0).max(logo_size.y).max(galley.size().y);
                let (row, _) =
                    ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
                let logo_rect = egui::Rect::from_center_size(
                    egui::pos2(row.left() + logo_size.x / 2.0, row.center().y),
                    logo_size,
                );
                self.draw_header_logo(ui, logo_rect);
                let text_rect = egui::Rect::from_min_size(
                    egui::pos2(logo_rect.right() + gap, row.top()),
                    egui::vec2(text_width, height),
                );
                let text_pos = egui::pos2(text_rect.left(), row.center().y - galley.size().y / 2.0);
                ui.painter()
                    .with_clip_rect(text_rect.intersect(ui.clip_rect()))
                    .galley(text_pos, galley, text_color);
                let actions_rect = egui::Rect::from_min_max(
                    egui::pos2(row.right() - actions_width, row.top()),
                    row.max,
                );
                let mut actions = ui.new_child(
                    egui::UiBuilder::new()
                        .id_salt("header_actions")
                        .max_rect(actions_rect)
                        .layout(egui::Layout::right_to_left(egui::Align::Center)),
                );
                actions.set_clip_rect(actions_rect.intersect(ui.clip_rect()));
                if actions.button("ⓘ  Info").clicked() {
                    self.info_popup = true;
                }
                if actions
                    .button(if en { "Settings" } else { "Einstellungen" })
                    .clicked()
                {
                    self.settings_popup = true;
                }
                if actions
                    .button(if self.dark_mode {
                        if en {
                            "☀ Light"
                        } else {
                            "☀ Hell"
                        }
                    } else if en {
                        "🌙 Dark"
                    } else {
                        "🌙 Dunkel"
                    })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }
                if actions
                    .button(if en { "Deutsch" } else { "English" })
                    .clicked()
                {
                    self.english = !self.english;
                }
            });
    }

    fn ui_media_album_filters(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        // Reserve the complete control before ComboBox creates its inner UI.
        // Its native button frame does not perform the parent's horizontal wrap.
        ui.allocate_ui_with_layout(
            egui::vec2(180.0, ui.spacing().interact_size.y),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::ComboBox::from_id_salt("media_type_combo")
                    .width(180.0)
                    .truncate()
                    .selected_text(match self.media_mode {
                        MediaMode::All => {
                            if en {
                                "Photos and videos"
                            } else {
                                "Fotos und Videos"
                            }
                        }
                        MediaMode::Photos => {
                            if en {
                                "Photos only"
                            } else {
                                "Nur Fotos"
                            }
                        }
                        MediaMode::Videos => {
                            if en {
                                "Videos only"
                            } else {
                                "Nur Videos"
                            }
                        }
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.media_mode,
                            MediaMode::All,
                            if en {
                                "Photos and videos"
                            } else {
                                "Fotos und Videos"
                            },
                        );
                        ui.selectable_value(
                            &mut self.media_mode,
                            MediaMode::Photos,
                            if en { "Photos only" } else { "Nur Fotos" },
                        );
                        ui.selectable_value(
                            &mut self.media_mode,
                            MediaMode::Videos,
                            if en { "Videos only" } else { "Nur Videos" },
                        );
                    });
            },
        );

        ui.add_space(4.0);
        ui.checkbox(
            &mut self.own_albums,
            if en { "Own albums" } else { "Eigene Alben" },
        )
        .on_hover_text(if en {
            "Applies when loading the album list."
        } else {
            "Gilt beim Laden der Albumliste."
        });
        ui.checkbox(
            &mut self.shared_albums,
            if en {
                "Shared albums"
            } else {
                "Geteilte Alben"
            },
        )
        .on_hover_text(if en {
            "Applies when loading the album list."
        } else {
            "Gilt beim Laden der Albumliste."
        });
    }

    fn show_settings_popup(&mut self, ctx: &egui::Context) {
        if !self.settings_popup {
            return;
        }
        let en = self.english;
        let mut open = self.settings_popup;
        let mut close_clicked = false;
        let bounds = ctx.screen_rect().shrink(16.0);
        egui::Window::new(if en { "Settings" } else { "Einstellungen" })
            .id(egui::Id::new("application_settings"))
            .open(&mut open)
            .collapsible(false)
            .default_width(540.0)
            .min_width(300.0_f32.min(bounds.width()))
            .max_width(bounds.width())
            .max_height(bounds.height())
            .constrain_to(bounds)
            .vscroll(true)
            .show(ctx, |ui| {
                self.ui_connection_settings(ui);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    if ui.button(if en { "Close" } else { "Schließen" }).clicked() {
                        close_clicked = true;
                    }
                });
            });
        self.settings_popup = open && !close_clicked;
        if !self.settings_popup {
            self.show_key = false;
        }
    }

    fn ui_connection_settings(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        ui.label(if en { "Immich server" } else { "Immich-Server" });
        ui.add_sized(
            [ui.available_width(), 30.0],
            egui::TextEdit::singleline(&mut self.server),
        );
        ui.label(if en { "API key" } else { "API-Key" });
        ui.horizontal(|ui| {
            let key_width = (ui.available_width() - 100.0 - ui.spacing().item_spacing.x).max(1.0);
            ui.add_sized(
                [key_width, 30.0],
                egui::TextEdit::singleline(&mut self.api_key).password(!self.show_key),
            );
            if ui
                .add_sized(
                    [100.0, 30.0],
                    egui::Button::new(if self.show_key {
                        if en {
                            "Hide"
                        } else {
                            "Verbergen"
                        }
                    } else if en {
                        "Show"
                    } else {
                        "Anzeigen"
                    }),
                )
                .clicked()
            {
                self.show_key = !self.show_key;
            }
        });
        ui.horizontal_wrapped(|ui| {
            if ui
                .button(if en {
                    "Delete saved API key"
                } else {
                    "Gespeicherten API-Key löschen"
                })
                .clicked()
            {
                self.api_key.clear();
                self.status = match delete_saved_settings() {
                    Ok(()) => {
                        if en {
                            "Saved API key deleted.".into()
                        } else {
                            "Gespeicherter API-Key wurde gelöscht.".into()
                        }
                    }
                    Err(e) => {
                        if en {
                            format!("API key could not be deleted: {}", e)
                        } else {
                            format!("API-Key konnte nicht gelöscht werden: {}", e)
                        }
                    }
                };
            }
        });
    }

    fn ui_download_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(ui.available_width());

        egui::Frame::group(ui.style())
            .fill(self.panel_bg())
            .stroke(egui::Stroke::new(
                1.0_f32,
                if self.dark_mode {
                    egui::Color32::from_rgb(78, 88, 101)
                } else {
                    egui::Color32::from_rgb(214, 219, 226)
                },
            ))
            .rounding(egui::Rounding::same(12.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.set_max_width(ui.available_width());

                ui.heading(
                    egui::RichText::new(if self.english {
                        "Download & settings"
                    } else {
                        "Download & Einstellungen"
                    })
                    .size(18.0)
                    .strong(),
                );

                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(if self.english {
                        "Destination folder"
                    } else {
                        "Zielordner"
                    })
                    .strong(),
                );
                ui.add_space(3.0);

                ui.add_sized(
                    [ui.available_width(), 32.0],
                    egui::TextEdit::singleline(&mut self.target_dir).hint_text(if self.english {
                        "Select backup folder"
                    } else {
                        "Backup-Ordner auswählen"
                    }),
                );

                ui.add_space(5.0);

                if ui
                    .add_sized(
                        [ui.available_width(), 32.0],
                        egui::Button::new(if self.english {
                            "Browse …"
                        } else {
                            "Durchsuchen …"
                        }),
                    )
                    .clicked()
                {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.target_dir = folder.to_string_lossy().to_string();
                    }
                }

                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(if self.english {
                        "Existing files"
                    } else {
                        "Vorhandene Dateien"
                    })
                    .strong(),
                );
                ui.add_space(3.0);

                egui::ComboBox::from_id_salt("existing_mode_side_combo")
                    .selected_text(match self.existing_mode {
                        ExistingMode::Ask => {
                            if self.english {
                                "Compare / ask"
                            } else {
                                "Vergleichen / nachfragen"
                            }
                        }
                        ExistingMode::Overwrite => {
                            if self.english {
                                "Overwrite directly"
                            } else {
                                "Direkt überschreiben"
                            }
                        }
                    })
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.existing_mode,
                            ExistingMode::Ask,
                            if self.english {
                                "Compare / ask"
                            } else {
                                "Vergleichen / nachfragen"
                            },
                        );
                        ui.selectable_value(
                            &mut self.existing_mode,
                            ExistingMode::Overwrite,
                            if self.english {
                                "Overwrite directly"
                            } else {
                                "Direkt überschreiben"
                            },
                        );
                    });

                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(if self.english {
                        "Parallel downloads"
                    } else {
                        "Parallele Downloads"
                    })
                    .strong(),
                );
                ui.add_space(3.0);

                egui::ComboBox::from_id_salt("parallel_downloads_side_combo")
                    .selected_text(if self.english {
                        format!("{} simultaneous", self.parallel_downloads)
                    } else {
                        format!("{} gleichzeitig", self.parallel_downloads)
                    })
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for n in [1usize, 2, 4, 6, 8, 12] {
                            ui.selectable_value(
                                &mut self.parallel_downloads,
                                n,
                                if self.english {
                                    format!("{} simultaneous", n)
                                } else {
                                    format!("{} gleichzeitig", n)
                                },
                            );
                        }
                    });
            });
    }

    fn show_download_protocol_popup(&mut self, ctx: &egui::Context) {
        if !self.download_protocol_popup {
            return;
        }

        let mut open = self.download_protocol_popup;
        egui::Window::new(if self.english {
            "Download log"
        } else {
            "Protokoll"
        })
        .open(&mut open)
        .resizable(false)
        .movable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .fixed_size(egui::vec2(520.0, 360.0))
        .show(ctx, |ui| {
            ui.heading("Status");
            ui.add_space(8.0);
            ui.label(egui::RichText::new(&self.status).strong());
            ui.add_space(14.0);

            egui::Grid::new("download_protocol_grid")
                .num_columns(2)
                .min_col_width(170.0)
                .spacing([24.0, 10.0])
                .show(ui, |ui| {
                    for (label, value) in [
                        (
                            if self.english {
                                "Selected albums"
                            } else {
                                "Ausgewählte Alben"
                            },
                            self.protocol_selected_albums.to_string(),
                        ),
                        (
                            if self.english {
                                "Selected years"
                            } else {
                                "Ausgewählte Jahre"
                            },
                            self.protocol_selected_years.to_string(),
                        ),
                        (
                            if self.english { "Files" } else { "Dateien" },
                            format!("{} / {}", self.download_current, self.download_total),
                        ),
                        (
                            if self.english {
                                "Downloaded"
                            } else {
                                "Heruntergeladen"
                            },
                            self.download_downloaded.to_string(),
                        ),
                        (
                            if self.english {
                                "Existing"
                            } else {
                                "Vorhanden"
                            },
                            self.download_skipped.to_string(),
                        ),
                        (
                            if self.english { "Errors" } else { "Fehler" },
                            self.download_failed.to_string(),
                        ),
                        (
                            if self.english {
                                "Active downloads"
                            } else {
                                "Aktive Downloads"
                            },
                            self.download_active.to_string(),
                        ),
                    ] {
                        ui.label(label);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(value);
                        });
                        ui.end_row();
                    }
                });
        });
        self.download_protocol_popup = open;
    }

    fn ui_download_button(&mut self, ui: &mut egui::Ui) {
        let selected_count = self.selected_album_count()
            + self.selected_year_count()
            + self.selected_all_year_count();
        let can_download =
            !self.download_running && selected_count > 0 && !self.target_dir.trim().is_empty();

        let download_fill = if can_download {
            egui::Color32::from_rgb(15, 157, 88)
        } else if self.dark_mode {
            egui::Color32::from_rgb(77, 91, 109)
        } else {
            egui::Color32::from_rgb(190, 201, 214)
        };

        let download_button = egui::Button::new(
            egui::RichText::new(if self.download_running {
                if self.english {
                    "↓ Download running …"
                } else {
                    "↓ Download läuft …"
                }
            } else if self.english {
                "↓ Download"
            } else {
                "↓ Herunterladen"
            })
            .color(egui::Color32::WHITE)
            .strong(),
        )
        .fill(download_fill)
        .stroke(egui::Stroke::new(1.0_f32, download_fill))
        .rounding(egui::Rounding::same(7.0))
        .min_size(egui::vec2(ui.available_width(), 38.0));

        if ui.add_enabled(can_download, download_button).clicked() {
            if let Err(e) = self.start_background_download() {
                self.status = if self.english {
                    format!("Error: {}", e)
                } else {
                    format!("Fehler: {}", e)
                };
            }
        }
    }

    fn ui_tabs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(self.panel_bg())
            .rounding(egui::Rounding::same(10.0))
            .inner_margin(egui::Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (tab, label) in [
                        (
                            ActiveTab::Albums,
                            if self.english { "Albums" } else { "Alben" },
                        ),
                        (
                            ActiveTab::NoAlbum,
                            if self.english {
                                "Photos without album by year"
                            } else {
                                "Fotos ohne Album nach Jahr"
                            },
                        ),
                        (
                            ActiveTab::AllByYear,
                            if self.english {
                                "All photos by year"
                            } else {
                                "Alle Fotos nach Jahr"
                            },
                        ),
                    ] {
                        let active = self.active_tab == tab;
                        let button = egui::Button::new(egui::RichText::new(label).strong().color(
                            if active {
                                egui::Color32::WHITE
                            } else if self.dark_mode {
                                egui::Color32::from_rgb(225, 230, 236)
                            } else {
                                egui::Color32::from_rgb(55, 65, 81)
                            },
                        ))
                        .fill(if active {
                            self.accent()
                        } else if self.dark_mode {
                            egui::Color32::from_rgb(63, 67, 72)
                        } else {
                            egui::Color32::from_rgb(246, 247, 249)
                        })
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            if active {
                                self.accent()
                            } else if self.dark_mode {
                                egui::Color32::from_rgb(96, 104, 115)
                            } else {
                                egui::Color32::from_rgb(218, 222, 228)
                            },
                        ))
                        .rounding(egui::Rounding::same(7.0));

                        if ui.add(button).clicked() {
                            self.active_tab = tab;
                        }
                    }
                });
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(4.0);
                match self.active_tab {
                    ActiveTab::Albums => self.ui_albums_tab(ui),
                    ActiveTab::NoAlbum => self.ui_no_album_tab(ui),
                    ActiveTab::AllByYear => self.ui_all_by_year_tab(ui),
                }
            });
    }

    fn ui_load_albums_button(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        let test_button = egui::Button::new(
            egui::RichText::new(if en {
                "Test connection / load albums"
            } else {
                "Verbindung testen / Alben laden"
            })
            .color(self.accent())
            .strong(),
        )
        .stroke(egui::Stroke::new(1.0_f32, self.accent()));
        if ui.add_sized([245.0, 30.0], test_button).clicked() {
            match self.load_albums() {
                Ok(()) => {
                    self.active_tab = ActiveTab::Albums;
                    if let Err(e) = save_settings(&self.server, &self.api_key) {
                        self.status = if en {
                            format!("Connection successful, saving failed: {}", e)
                        } else {
                            format!("Verbindung erfolgreich, Speichern fehlgeschlagen: {}", e)
                        };
                    }
                }
                Err(e) => {
                    self.status = if en {
                        format!("Error: {}", e)
                    } else {
                        format!("Fehler: {}", e)
                    };
                }
            }
        }
    }

    fn ui_album_search(&mut self, ui: &mut egui::Ui) {
        let width = ui.available_width().min(220.0).max(1.0);
        ui.add_sized(
            [width, 30.0],
            egui::TextEdit::singleline(&mut self.album_search).hint_text(if self.english {
                "Search albums ..."
            } else {
                "Album suchen ..."
            }),
        );
    }

    fn ui_albums_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(9.0, 6.0);
            self.ui_load_albums_button(ui);
            self.ui_album_search(ui);
            if ui
                .add_sized(
                    [118.0, 30.0],
                    egui::Button::new(if self.english {
                        "Select all"
                    } else {
                        "Alle auswählen"
                    }),
                )
                .clicked()
            {
                for item in &mut self.albums {
                    item.selected = true;
                }
            }
            if ui
                .add_sized(
                    [138.0, 30.0],
                    egui::Button::new(if self.english {
                        "Clear selection"
                    } else {
                        "Auswahl aufheben"
                    }),
                )
                .clicked()
            {
                for item in &mut self.albums {
                    item.selected = false;
                }
            }
            // Count after the actions so it updates in the same frame.
            let selected_count = self.selected_album_count();
            ui.label(
                egui::RichText::new(if self.english {
                    format!("{} selected", selected_count)
                } else {
                    format!("{} ausgewählt", selected_count)
                })
                .strong(),
            );
            ui.selectable_value(
                &mut self.album_view,
                AlbumView::Covers,
                if self.english {
                    "Album covers"
                } else {
                    "Albumcover"
                },
            );
            ui.selectable_value(
                &mut self.album_view,
                AlbumView::List,
                if self.english { "List" } else { "Liste" },
            );
        });
        ui.add_space(8.0);
        album_view::show(
            ui,
            &mut self.albums,
            &self.album_search,
            self.album_view,
            self.english,
        );
    }

    fn ui_no_album_tab(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        ui.scope(|ui| {
            // Seed the row height before laying out any label, checkbox or button.
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ui.spacing_mut().interact_size.y = 32.0;
            ui.spacing_mut().button_padding = egui::vec2(9.0, 6.0);
            ui.horizontal_wrapped(|ui| {
                let load_button = egui::Button::new(if en {
                    "Load photos without albums by year"
                } else {
                    "Fotos ohne Album nach Jahr laden"
                })
                .rounding(egui::Rounding::same(7.0));
                if ui.add(load_button).clicked() {
                    if let Err(e) = self.load_no_album_years() {
                        self.status = if en {
                            format!("Error: {}", e)
                        } else {
                            format!("Fehler: {}", e)
                        };
                    }
                }
                ui.add_sized(
                    [150.0_f32, 30.0_f32],
                    egui::TextEdit::singleline(&mut self.year_search).hint_text(if en {
                        "Search year"
                    } else {
                        "Jahr suchen"
                    }),
                );
                if ui
                    .button(if en { "Select all" } else { "Alle auswählen" })
                    .clicked()
                {
                    for item in &mut self.year_buckets {
                        item.selected = true;
                    }
                }
                if ui
                    .button(if en {
                        "Clear selection"
                    } else {
                        "Auswahl aufheben"
                    })
                    .clicked()
                {
                    for item in &mut self.year_buckets {
                        item.selected = false;
                    }
                }
                ui.label(if en {
                    format!("{} year folders selected", self.selected_year_count())
                } else {
                    format!("{} Jahresordner ausgewählt", self.selected_year_count())
                });
                ui.add_space(16.0);
                self.ui_media_album_filters(ui);
            });
        });
        ui.add_space(10.0_f32);

        album_view::show_years(
            ui,
            &mut self.year_buckets,
            &self.year_search,
            "no_album_year_scroll",
            en,
        );
    }

    fn ui_all_by_year_tab(&mut self, ui: &mut egui::Ui) {
        let en = self.english;
        ui.scope(|ui| {
            // Seed the row height before laying out any label, checkbox or button.
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ui.spacing_mut().interact_size.y = 32.0;
            ui.spacing_mut().button_padding = egui::vec2(9.0, 6.0);
            ui.horizontal_wrapped(|ui| {
                let load_button = egui::Button::new(if en {
                    "Load all photos by year"
                } else {
                    "Alle Fotos nach Jahr laden"
                })
                .rounding(egui::Rounding::same(7.0));
                if ui.add(load_button).clicked() {
                    if let Err(e) = self.load_all_assets_by_year() {
                        self.status = if en {
                            format!("Error: {}", e)
                        } else {
                            format!("Fehler: {}", e)
                        };
                    }
                }
                ui.add_sized(
                    [150.0_f32, 30.0_f32],
                    egui::TextEdit::singleline(&mut self.year_search).hint_text(if en {
                        "Search year"
                    } else {
                        "Jahr suchen"
                    }),
                );
                if ui
                    .button(if en { "Select all" } else { "Alle auswählen" })
                    .clicked()
                {
                    for item in &mut self.all_year_buckets {
                        item.selected = true;
                    }
                }
                if ui
                    .button(if en {
                        "Clear selection"
                    } else {
                        "Auswahl aufheben"
                    })
                    .clicked()
                {
                    for item in &mut self.all_year_buckets {
                        item.selected = false;
                    }
                }
                ui.label(if en {
                    format!("{} year folders selected", self.selected_all_year_count())
                } else {
                    format!("{} Jahresordner ausgewählt", self.selected_all_year_count())
                });
                ui.add_space(16.0);
                self.ui_media_album_filters(ui);
            });
        });
        ui.add_space(10.0_f32);
        album_view::show_years(
            ui,
            &mut self.all_year_buckets,
            &self.year_search,
            "all_year_scroll",
            en,
        );
    }

    fn ui_footer(&mut self, ui: &mut egui::Ui) {
        let lower_status = self.status.to_lowercase();
        let error = lower_status.starts_with("fehler") || lower_status.starts_with("error");
        let dot_color = if self.download_running {
            egui::Color32::from_rgb(245, 158, 11)
        } else if error {
            egui::Color32::from_rgb(239, 68, 68)
        } else {
            egui::Color32::from_rgb(34, 197, 94)
        };
        let (row, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 20.0), egui::Sense::hover());
        ui.painter()
            .circle_filled(egui::pos2(row.left() + 6.0, row.center().y), 5.0, dot_color);
        // Reserve the version first: long status messages cannot push it off-screen.
        let version_width = 110.0_f32.min(row.width());
        let version_rect =
            egui::Rect::from_min_max(egui::pos2(row.right() - version_width, row.top()), row.max);
        let message_rect = egui::Rect::from_min_max(
            egui::pos2(row.left() + 20.0, row.top()),
            egui::pos2(
                (version_rect.left() - 12.0).max(row.left() + 20.0),
                row.bottom(),
            ),
        );
        let status = if self.status == "Bereit." || self.status.is_empty() {
            if self.english {
                "Ready"
            } else {
                "Bereit"
            }
        } else {
            &self.status
        };
        let mut message_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(message_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        message_ui.set_clip_rect(message_rect.intersect(ui.clip_rect()));
        message_ui
            .add_sized(message_rect.size(), egui::Label::new(status).truncate())
            .on_hover_text(status);
        let mut version_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(version_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        version_ui.set_clip_rect(version_rect.intersect(ui.clip_rect()));
        version_ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
    }

    fn show_info_popup(&mut self, ctx: &egui::Context) {
        if !self.info_popup {
            return;
        }
        let year = Local::now().year();
        let mut open = self.info_popup;
        let mut close_clicked = false;
        let en = self.english;
        let bounds = ctx.screen_rect().shrink(16.0);
        // Window sizing controls content size; leave room for its title and frame as well.
        let max_content_height = (bounds.height() - 48.0).max(1.0);
        let width = 820.0_f32.min((bounds.width() - 16.0).max(1.0));
        let height = 650.0_f32.min(max_content_height);
        egui::Window::new(if en { "About" } else { "Info" })
            .id(egui::Id::new("about_window"))
            .collapsible(false)
            // Horizontal resizing is useful, but changing only the width must
            // never make the About window collapse vertically because of text
            // reflow. Keep its current target height fixed while it is open.
            .resizable([true, false])
            .default_width(width)
            .default_height(height)
            .min_width(360.0_f32.min(width))
            .min_height(height)
            .max_width(width)
            .max_height(height)
            .default_pos(bounds.center() - egui::vec2(width / 2.0, (height + 32.0) / 2.0))
            .constrain_to(bounds)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.heading("Media Backup Manager");
                ui.label(egui::RichText::new("for Immich").italics());
                ui.label(format!("Version {} · Open Source · Copyright © {} Ralf Ebert", env!("CARGO_PKG_VERSION"), year));
                ui.separator();
                // Reserve the footer before sizing the scrolling text. Its row must
                // already be tall enough for the padded button when laying out the label.
                let footer_height = 38.0;
                let footer_space = footer_height + 3.0 * ui.spacing().item_spacing.y + 6.0;
                let body_height = (ui.available_height() - footer_space).max(0.0);
                egui::ScrollArea::vertical()
                    .id_salt("info_scroll")
                    .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .max_height(body_height)
                    .min_scrolled_height(0.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    if en {
                        ui.heading("Program description – Media Backup Manager");
                        ui.label("Media Backup Manager is an independent third-party Windows application for downloading and backing up photos and videos from an existing Immich installation.");
                        ui.add_space(6.0);
                        ui.label("The application connects directly to the Immich server using the entered server address and a personal API key. These credentials are used exclusively to connect to the specified server.");
                        ui.add_space(12.0);
                        ui.heading("Supported backup methods");
                        ui.label(egui::RichText::new("Albums").strong());
                        ui.label("Selected Immich albums are downloaded completely. A separate folder bearing the album name is created in the destination folder for each album.");
                        ui.label(egui::RichText::new("Photos without albums by year").strong());
                        ui.label("Photos and videos that are not assigned to an album are sorted by capture year and saved in the corresponding year folders.");
                        ui.label(egui::RichText::new("All photos by year").strong());
                        ui.label("All photos and videos stored in Immich are downloaded by capture year, regardless of their album assignment.");
                        ui.add_space(12.0);
                        ui.heading("Album overview");
                        ui.label("Existing albums are shown with the album name, preview image, number of files, and an indicator showing whether the album is owned or shared. Multiple albums or year folders can be selected and downloaded in one operation.");
                        ui.add_space(12.0);
                        ui.heading("Existing files");
                        ui.label(egui::RichText::new("Compare / ask").strong());
                        ui.label("Existing files are not overwritten immediately. When the local file and the file stored in Immich differ, a comparison window opens. File size, capture time, resolution, storage location, and—when available—the image preview can be compared before deciding which file to keep.");
                        ui.label(egui::RichText::new("Overwrite directly").strong());
                        ui.label("Existing files are replaced by the Immich version without further confirmation. Files that already exist completely and have the same size can be skipped automatically.");
                        ui.add_space(12.0);
                        ui.heading("Duplicate and file comparison");
                        ui.label("Different versions of an existing file can be displayed side by side. The application can make a recommendation based on file size, but the final decision always remains with the user. Individual files can be marked manually to keep or overwrite them.");
                        ui.add_space(12.0);
                        ui.heading("Parallel downloads");
                        ui.label("Several files can be downloaded at the same time to speed up the backup. The number of parallel downloads can be selected in the settings. A very high setting may increase the load on the Immich server, network, and destination drive.");
                        ui.add_space(12.0);
                        ui.heading("Progress display");
                        ui.label("During a download, the application displays the current status, selected albums or years, total number of files, downloaded files, skipped existing files, errors, active downloads, and overall progress in percent.");
                        ui.add_space(12.0);
                        ui.heading("API key encryption and storage");
                        ui.label("The Immich API key is stored locally using Windows Data Protection API (DPAPI) encryption. The encrypted value is bound to the current Windows user account and normally cannot be decrypted by another Windows account or on another computer.");
                        ui.add_space(6.0);
                        ui.label("The encrypted settings are stored in:");
                        ui.monospace(r"%APPDATA%\Media_Backup_Manager\settings.json");
                        ui.add_space(6.0);
                        ui.label("If an older version previously stored the API key as plain text, the application automatically encrypts and migrates it when the settings are loaded successfully for the first time. The saved key can be removed at any time using the ‘Delete saved API key’ button.");
                        ui.add_space(6.0);
                        ui.label("The API key is used only for the connection to the Immich server entered by the user. It is not transmitted to the developer or to third-party servers. The settings file should still not be shared or included in support screenshots or public uploads.");
                        ui.add_space(12.0);
                        ui.heading("Immich access");
                        ui.label("Media Backup Manager communicates with Immich exclusively through the Immich API. It does not connect directly to or modify the Immich PostgreSQL database.");
                        ui.add_space(12.0);
                        ui.heading("Backup notice");
                        ui.label("Media Backup Manager downloads and backs up original media files. It does not back up the Immich database and therefore does not replace a complete Immich server backup. After every larger backup, verify that the expected files are present and readable in the destination folder.");
                        ui.add_space(12.0);
                        ui.heading("Open-source license");
                        ui.label("Open-source software");
                        ui.label("Licensed under the GNU General Public License v3.0");
                        ui.label(egui::RichText::new("Copyright © 2026 Ralf Ebert").strong());
                        ui.hyperlink_to(
                            "GNU GPL v3.0 license on GitHub",
                            "https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/blob/main/LICENSE",
                        );
                        ui.add_space(6.0);
                        ui.label("The software may be used, studied, modified, and redistributed under the terms of GPL-3.0-only. No additional restriction on commercial use is imposed by this project.");
                        ui.add_space(12.0);
                        ui.heading("Immich Notice");
                        ui.label("Media Backup Manager is an independent third-party application for Immich. It is not affiliated with, endorsed by, or sponsored by Immich or FUTO.");
                        ui.label("Immich and related trademarks are the property of their respective owners.");
                    } else {
                        ui.heading("Programmerklärung – Media Backup Manager");
                        ui.label("Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung für Windows zum Herunterladen und Sichern von Fotos und Videos aus einer bestehenden Immich-Installation.");
                        ui.add_space(6.0);
                        ui.label("Das Programm verbindet sich über die eingegebene Immich-Serveradresse und einen persönlichen API-Key direkt mit dem Immich-Server. Die Zugangsdaten werden ausschließlich für die Verbindung mit dem angegebenen Server verwendet.");
                        ui.add_space(12.0);
                        ui.heading("Unterstützte Sicherungsarten");
                        ui.label(egui::RichText::new("Alben").strong());
                        ui.label("Ausgewählte Immich-Alben werden vollständig heruntergeladen. Für jedes Album wird im Zielordner ein eigener Ordner mit dem jeweiligen Albumnamen erstellt.");
                        ui.label(egui::RichText::new("Fotos ohne Album nach Jahr").strong());
                        ui.label("Fotos und Videos, die keinem Album zugeordnet sind, werden nach ihrem Aufnahmejahr sortiert und in entsprechenden Jahresordnern gespeichert.");
                        ui.label(egui::RichText::new("Alle Fotos nach Jahr").strong());
                        ui.label("Alle in Immich vorhandenen Fotos und Videos werden unabhängig von ihrer Albumzuordnung nach Aufnahmejahr sortiert heruntergeladen.");
                        ui.add_space(12.0);
                        ui.heading("Albumübersicht");
                        ui.label("Die vorhandenen Alben werden mit Albumname, Vorschaubild, Anzahl der enthaltenen Dateien und Kennzeichnung als eigenes oder geteiltes Album angezeigt. Mehrere Alben oder Jahresordner können gleichzeitig ausgewählt und in einem Vorgang heruntergeladen werden.");
                        ui.add_space(12.0);
                        ui.heading("Vorhandene Dateien");
                        ui.label(egui::RichText::new("Vergleichen / nachfragen").strong());
                        ui.label("Bereits vorhandene Dateien werden nicht sofort überschrieben. Unterscheiden sich die vorhandene und die in Immich gespeicherte Datei, öffnet sich ein Vergleichsfenster. Dateigröße, Aufnahmezeit, Auflösung, Speicherort und – soweit verfügbar – die Bildvorschau können miteinander verglichen werden.");
                        ui.label(egui::RichText::new("Direkt überschreiben").strong());
                        ui.label("Vorhandene Dateien werden ohne weitere Nachfrage durch die Version aus Immich ersetzt. Gleich große und bereits vollständig vorhandene Dateien können automatisch übersprungen werden.");
                        ui.add_space(12.0);
                        ui.heading("Duplikat- und Dateivergleich");
                        ui.label("Unterschiedliche Versionen einer vorhandenen Datei können übersichtlich gegenübergestellt werden. Das Programm kann anhand der Dateigröße eine Empfehlung geben. Die endgültige Entscheidung bleibt beim Benutzer.");
                        ui.add_space(12.0);
                        ui.heading("Parallele Downloads");
                        ui.label("Zur Beschleunigung können mehrere Dateien gleichzeitig heruntergeladen werden. Die Anzahl lässt sich in den Einstellungen auswählen. Eine sehr hohe Anzahl kann Immich-Server, Netzwerk und Ziel-Laufwerk stärker belasten.");
                        ui.add_space(12.0);
                        ui.heading("Fortschrittsanzeige");
                        ui.label("Während des Downloads werden aktueller Status, ausgewählte Alben oder Jahre, Gesamtzahl der Dateien, heruntergeladene Dateien, übersprungene Dateien, Fehler, aktive Downloads und der Gesamtfortschritt in Prozent angezeigt.");
                        ui.add_space(12.0);
                        ui.heading("Verschlüsselung und Speicherung des API-Keys");
                        ui.label("Der Immich-API-Key wird lokal mit der Windows Data Protection API (DPAPI) verschlüsselt gespeichert. Der verschlüsselte Wert ist an das aktuelle Windows-Benutzerkonto gebunden und kann normalerweise weder von einem anderen Windows-Konto noch auf einem anderen Computer entschlüsselt werden.");
                        ui.add_space(6.0);
                        ui.label("Die verschlüsselten Einstellungen werden gespeichert unter:");
                        ui.monospace(r"%APPDATA%\Media_Backup_Manager\settings.json");
                        ui.add_space(6.0);
                        ui.label("Falls eine ältere Version den API-Key zuvor unverschlüsselt gespeichert hat, wird er beim ersten erfolgreichen Laden der Einstellungen automatisch verschlüsselt und in das neue Format übernommen. Über die Schaltfläche ‘Gespeicherten API-Key löschen’ kann der gespeicherte Schlüssel jederzeit entfernt werden.");
                        ui.add_space(6.0);
                        ui.label("Der API-Key wird ausschließlich für die Verbindung mit dem vom Benutzer eingetragenen Immich-Server verwendet. Er wird nicht an den Entwickler oder an fremde Server übertragen. Die Einstellungsdatei sollte trotzdem nicht weitergegeben und nicht in Support-Screenshots oder öffentliche Uploads aufgenommen werden.");
                        ui.add_space(12.0);
                        ui.heading("Zugriff auf Immich");
                        ui.label("Media Backup Manager kommuniziert ausschließlich über die Immich-API mit Immich. Das Programm greift nicht direkt auf die PostgreSQL-Datenbank von Immich zu und verändert diese nicht.");
                        ui.add_space(12.0);
                        ui.heading("Hinweis zu Backups");
                        ui.label("Media Backup Manager lädt originale Mediendateien herunter und sichert diese. Die Immich-Datenbank wird nicht gesichert. Das Programm ersetzt deshalb keine vollständige Sicherung des Immich-Servers. Nach jeder größeren Sicherung sollte geprüft werden, ob die erwarteten Dateien vollständig vorhanden und lesbar sind.");
                        ui.add_space(12.0);
                        ui.heading("Open-Source-Lizenz");
                        ui.label("Open-Source-Software");
                        ui.label("Lizenziert unter der GNU General Public License v3.0");
                        ui.label(egui::RichText::new("Copyright © 2026 Ralf Ebert").strong());
                        ui.hyperlink_to(
                            "GNU GPL v3.0-Lizenz auf GitHub",
                            "https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/blob/main/LICENSE",
                        );
                        ui.add_space(6.0);
                        ui.label("Die Software darf gemäß GPL-3.0-only genutzt, untersucht, verändert und weitergegeben werden. Das Projekt legt keine zusätzliche Einschränkung für kommerzielle Nutzung fest.");
                        ui.add_space(12.0);
                        ui.heading("Hinweis zu Immich");
                        ui.label("Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung für Immich und kein offizielles Produkt von Immich oder FUTO.");
                        ui.label("Das Projekt steht in keiner Verbindung zu Immich oder FUTO und wird von diesen weder unterstützt noch gesponsert.");
                        ui.label("Immich und die zugehörigen Marken sind Eigentum ihrer jeweiligen Rechteinhaber.");
                    }
                });
                ui.separator();
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), footer_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.columns(2, |columns| {
                            columns[0].with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(
                                                "Copyright © 2026 Ralf Ebert",
                                            )
                                            .strong(),
                                        )
                                        .wrap_mode(egui::TextWrapMode::Extend),
                                    );
                                },
                            );
                            columns[1].with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add_sized(
                                            [110.0, footer_height],
                                            egui::Button::new(if en {
                                                "Close"
                                            } else {
                                                "Schließen"
                                            }),
                                        )
                                        .clicked()
                                    {
                                        close_clicked = true;
                                    }
                                },
                            );
                        });
                    },
                );
            });
        self.info_popup = open && !close_clicked;
    }

    fn show_download_popup(&mut self, ctx: &egui::Context) {
        if !self.download_popup {
            return;
        }
        let mut open = self.download_popup;
        let mut close_clicked = false;
        let mut cancel_clicked = false;

        egui::Window::new(if self.english { "Download" } else { "Download" })
            .collapsible(false)
            .resizable(false)
            .default_width(520.0)
            .open(&mut open)
            .show(ctx, |ui| {
                if self.download_album_total > 0 && self.download_album_current > 0 {
                    let album_status = if self.english {
                        format!(
                            "Album {} of {}: {}",
                            self.download_album_current,
                            self.download_album_total,
                            self.download_album_name
                        )
                    } else {
                        format!(
                            "Album {} von {}: {}",
                            self.download_album_current,
                            self.download_album_total,
                            self.download_album_name
                        )
                    };
                    ui.label(egui::RichText::new(album_status).strong());
                    ui.add_space(4.0);
                }

                if self.download_total > 0 {
                    ui.heading(if self.english {
                        format!(
                            "File {} of {}",
                            self.download_current.min(self.download_total),
                            self.download_total
                        )
                    } else {
                        format!(
                            "Datei {} von {}",
                            self.download_current.min(self.download_total),
                            self.download_total
                        )
                    });
                } else {
                    ui.heading(if self.english {
                        "Preparing download ..."
                    } else {
                        "Download wird vorbereitet ..."
                    });
                }

                if self.download_total > 0 {
                    ui.add_space(8.0);
                    let frac = self.download_current as f32 / self.download_total as f32;
                    ui.add(
                        egui::ProgressBar::new(frac)
                            .show_percentage()
                            .desired_width(ui.available_width()),
                    );
                    ui.add_space(8.0);
                }
                ui.label(&self.download_file);
                ui.label(if self.english {
                    format!("Active downloads: {}", self.download_active)
                } else {
                    format!("Aktive Downloads: {}", self.download_active)
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(if self.english {
                        format!("Downloaded: {}", self.download_downloaded)
                    } else {
                        format!("Heruntergeladen: {}", self.download_downloaded)
                    });
                    ui.label(if self.english {
                        format!("Existing/skipped: {}", self.download_skipped)
                    } else {
                        format!("Vorhanden: {}", self.download_skipped)
                    });
                    ui.label(if self.english {
                        format!("Errors: {}", self.download_failed)
                    } else {
                        format!("Fehler: {}", self.download_failed)
                    });
                });

                ui.add_space(10.0);
                if self.download_running {
                    if ui
                        .button(if self.english {
                            "Cancel download"
                        } else {
                            "Download abbrechen"
                        })
                        .clicked()
                    {
                        cancel_clicked = true;
                    }
                } else if ui.button("Schließen").clicked() {
                    close_clicked = true;
                }
            });

        if cancel_clicked {
            if let Some(flag) = &self.download_cancel {
                flag.store(true, Ordering::Relaxed);
                self.status =
                    "Abbruch angefordert – laufende Dateien werden noch beendet ...".to_string();
                self.download_file = "Download wird abgebrochen ...".to_string();
            }
        }
        self.download_popup = open && !close_clicked;
    }

    fn conflict_target_path(&self, item: &ConflictItem) -> PathBuf {
        Path::new(&self.target_dir)
            .join(&item.job.folder_name)
            .join(Self::sanitize_file_name(&Self::asset_file_name(
                &item.job.asset,
            )))
    }

    fn texture_and_size_from_bytes(
        ctx: &egui::Context,
        id: &str,
        bytes: &[u8],
        apply_exif_orientation: bool,
    ) -> Option<(egui::TextureHandle, (u32, u32))> {
        let mut decoded = image::load_from_memory(bytes).ok()?;

        // Lokale Originaldateien benötigen die EXIF-Ausrichtung. Immich liefert
        // seine Vorschaubilder bereits ausgerichtet; dort darf sie nicht erneut
        // angewendet werden, sonst können Bilder auf dem Kopf erscheinen.
        if apply_exif_orientation {
            let mut cursor = std::io::Cursor::new(bytes);
            if let Ok(exif) = exif::Reader::new().read_from_container(&mut cursor) {
                if let Some(field) = exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                    if let Some(orientation) = field.value.get_uint(0) {
                        decoded = match orientation {
                            2 => decoded.fliph(),
                            3 => decoded.rotate180(),
                            4 => decoded.flipv(),
                            5 => decoded.rotate90().fliph(),
                            6 => decoded.rotate90(),
                            7 => decoded.rotate270().fliph(),
                            8 => decoded.rotate270(),
                            _ => decoded,
                        };
                    }
                }
            }
        }

        let dims = (decoded.width(), decoded.height());
        let rgba = decoded.to_rgba8();
        let size = [decoded.width() as usize, decoded.height() as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let texture = ctx.load_texture(id.to_string(), color_image, egui::TextureOptions::LINEAR);
        Some((texture, dims))
    }

    fn ensure_current_conflict_preview(&mut self, ctx: &egui::Context) {
        let Some(item) = self.conflicts.get(self.conflict_index).cloned() else {
            return;
        };
        let preview_key = format!("{}:{}", item.job.asset.id, self.conflict_index);
        if self.conflict_preview_key == preview_key {
            return;
        }

        self.conflict_preview_key = preview_key;
        self.conflict_local_texture = None;
        self.conflict_remote_texture = None;
        self.conflict_local_dims = None;
        self.conflict_remote_dims = None;

        let local_path = self.conflict_target_path(&item);
        if let Ok(bytes) = fs::read(&local_path) {
            if let Some((texture, dims)) = Self::texture_and_size_from_bytes(
                ctx,
                &format!("conf_local_{}", item.job.asset.id),
                &bytes,
                true,
            ) {
                self.conflict_local_texture = Some(texture);
                self.conflict_local_dims = Some(dims);
            }
        }

        if let (Ok(base), Ok(key), Ok(client)) =
            (self.base_url(), self.ensure_api_key(), self.client())
        {
            let url = format!(
                "{}/api/assets/{}/thumbnail?size=thumbnail",
                base, item.job.asset.id
            );
            if let Ok(resp) = client
                .get(url)
                .header("x-api-key", key)
                .send()
                .and_then(|r| r.error_for_status())
            {
                if let Ok(bytes) = resp.bytes() {
                    if let Some((texture, dims)) = Self::texture_and_size_from_bytes(
                        ctx,
                        &format!("conf_remote_{}", item.job.asset.id),
                        bytes.as_ref(),
                        false,
                    ) {
                        self.conflict_remote_texture = Some(texture);
                        self.conflict_remote_dims = Some(dims);
                    }
                }
            }
        }
    }

    fn show_conflict_popup(&mut self, ctx: &egui::Context) {
        if !self.conflict_popup || self.conflicts.is_empty() {
            return;
        }

        self.ensure_current_conflict_preview(ctx);

        let total_groups = self.conflicts.len();
        let current_index = self.conflict_index.min(total_groups.saturating_sub(1));
        let item = self.conflicts[current_index].clone();
        let local_path = self.conflict_target_path(&item);
        let local_keep = !item.selected;
        let remote_keep = item.selected;
        let english = self.english;

        let mut close_clicked = false;
        let mut overwrite_selected = false;
        let mut overwrite_all = false;
        let mut keep_larger = false;

        let title = if english {
            "Duplicates / existing files"
        } else {
            "Duplikate / vorhandene Dateien"
        };

        let viewport_id = egui::ViewportId::from_hash_of("duplicate_management_window");
        let viewport_builder = egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([1100.0, 680.0])
            .with_min_inner_size([900.0, 580.0]);

        ctx.show_viewport_immediate(viewport_id, viewport_builder, |popup_ctx, _class| {
            if popup_ctx.input(|i| i.viewport().close_requested()) {
                close_clicked = true;
            }

            if popup_ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::N)) {
                if self.conflict_index + 1 < self.conflicts.len() {
                    self.conflict_index += 1;
                    self.conflict_preview_key.clear();
                }
            }
            if popup_ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                if self.conflict_index > 0 {
                    self.conflict_index -= 1;
                    self.conflict_preview_key.clear();
                }
            }
            if popup_ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                if let Some(item) = self.conflicts.get_mut(self.conflict_index) {
                    item.selected = !item.selected;
                }
            }

            Self::apply_style(popup_ctx, self.dark_mode);

            egui::CentralPanel::default()
                .frame(
                    egui::Frame::none()
                        .fill(self.page_bg())
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0)),
                )
                .show(popup_ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.heading(if english {
                            format!("Group {} of {}", current_index + 1, total_groups)
                        } else {
                            format!("Gruppe {} von {}", current_index + 1, total_groups)
                        });
                        ui.separator();
                        ui.label(if english {
                            format!("File: {}", Self::asset_file_name(&item.job.asset))
                        } else {
                            format!("Datei: {}", Self::asset_file_name(&item.job.asset))
                        });
                    });
                    ui.label(if english {
                        "Left/right arrow keys or N = next group, Space = toggle selection"
                    } else {
                        "Pfeiltasten links/rechts oder N = nächste Gruppe, Leertaste = Auswahl umschalten"
                    });
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(if english { "← Previous group" } else { "← Vorherige Gruppe" }).clicked() && self.conflict_index > 0 {
                            self.conflict_index -= 1;
                            self.conflict_preview_key.clear();
                        }
                        if ui.button(if english { "Next group →" } else { "Nächste Gruppe →" }).clicked() && self.conflict_index + 1 < total_groups {
                            self.conflict_index += 1;
                            self.conflict_preview_key.clear();
                        }
                        if ui.button(if english { "This one only: keep larger Immich file" } else { "Nur diese: größere Immich-Datei behalten" }).clicked() {
                            if let Some(cur) = self.conflicts.get_mut(current_index) {
                                cur.selected = cur.remote_size.map(|size| size > cur.local_size).unwrap_or(false);
                            }
                        }
                        if ui.button(if english { "All: mark larger Immich files" } else { "Alle: größere Immich-Dateien markieren" }).clicked() {
                            for c in &mut self.conflicts {
                                c.selected = c.remote_size.map(|size| size > c.local_size).unwrap_or(false);
                            }
                        }
                    });
                    ui.separator();

                    ui.columns(2, |cols| {
                        const CARD_HEIGHT: f32 = 420.0;
                        const PREVIEW_HEIGHT: f32 = 160.0;
                        let card_background = if self.dark_mode {
                            egui::Color32::from_rgb(24, 34, 40)
                        } else {
                            egui::Color32::from_rgb(248, 249, 250)
                        };
                        let keep_color = egui::Color32::from_rgb(40, 180, 95);
                        let reject_color = egui::Color32::from_rgb(230, 70, 70);
                        let left_label = if local_keep {
                            if english { "Will be kept" } else { "Wird behalten" }
                        } else if english { "Not selected" } else { "Nicht ausgewählt" };
                        let right_label = if remote_keep {
                            if english { "Will be kept" } else { "Wird behalten" }
                        } else if english { "Not selected" } else { "Nicht ausgewählt" };

                        let draw_preview = |ui: &mut egui::Ui, texture: Option<&egui::TextureHandle>| {
                            let preview_width = ui.available_width();
                            let preview_size = egui::vec2(preview_width, PREVIEW_HEIGHT);
                            if let Some(texture) = texture {
                                ui.add(
                                    egui::Image::new(texture)
                                        .fit_to_exact_size(preview_size)
                                        .maintain_aspect_ratio(true)
                                        .sense(egui::Sense::hover()),
                                );
                            } else {
                                let (r, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
                                ui.painter().rect_filled(r, 6.0, card_background);
                                ui.painter().text(
                                    r.center(),
                                    egui::Align2::CENTER_CENTER,
                                    if english { "No preview" } else { "Keine Vorschau" },
                                    egui::FontId::proportional(16.0),
                                    ui.visuals().weak_text_color(),
                                );
                            }
                        };

                        let card_width = cols[0].available_width();
                        egui::Frame::none()
                            .fill(card_background)
                            .stroke(egui::Stroke::new(1.5_f32, if local_keep { keep_color } else { reject_color }))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(&mut cols[0], |ui| {
                                ui.set_min_size(egui::vec2(card_width, CARD_HEIGHT));
                                ui.set_max_size(egui::vec2(card_width, CARD_HEIGHT));
                                ui.heading(if english { "Existing file in destination folder" } else { "Vorhandene Datei im Zielordner" });
                                ui.colored_label(if local_keep { keep_color } else { reject_color }, left_label);
                                ui.add_space(6.0);
                                draw_preview(ui, self.conflict_local_texture.as_ref());
                                ui.add_space(8.0);
                                ui.label(if english { format!("File name: {}", Self::asset_file_name(&item.job.asset)) } else { format!("Dateiname: {}", Self::asset_file_name(&item.job.asset)) });
                                ui.label(if english {
                                    format!("Capture time: {}", if item.job.asset.local_date_time.trim().is_empty() { "unknown" } else { &item.job.asset.local_date_time })
                                } else {
                                    format!("Aufnahmezeit: {}", if item.job.asset.local_date_time.trim().is_empty() { "unbekannt" } else { &item.job.asset.local_date_time })
                                });
                                ui.label(if english { format!("File size: {}", Self::format_bytes_u64(item.local_size)) } else { format!("Dateigröße: {}", Self::format_bytes_u64(item.local_size)) });
                                ui.label(if english {
                                    format!("Resolution: {}", self.conflict_local_dims.map(|(w,h)| format!("{} × {} px", w, h)).unwrap_or_else(|| "unknown".to_string()))
                                } else {
                                    format!("Auflösung: {}", self.conflict_local_dims.map(|(w,h)| format!("{} × {} px", w, h)).unwrap_or_else(|| "unbekannt".to_string()))
                                });
                                ui.label(if english { format!("Path: {}", local_path.display()) } else { format!("Pfad: {}", local_path.display()) });
                                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                    if ui.button(if english { "Keep this local file" } else { "Diese lokale Datei behalten" }).clicked() {
                                        if let Some(cur) = self.conflicts.get_mut(current_index) { cur.selected = false; }
                                    }
                                });
                            });

                        let card_width = cols[1].available_width();
                        egui::Frame::none()
                            .fill(card_background)
                            .stroke(egui::Stroke::new(1.5_f32, if remote_keep { keep_color } else { reject_color }))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(&mut cols[1], |ui| {
                                ui.set_min_size(egui::vec2(card_width, CARD_HEIGHT));
                                ui.set_max_size(egui::vec2(card_width, CARD_HEIGHT));
                                ui.heading(if english { "Immich file" } else { "Immich-Datei" });
                                ui.colored_label(if remote_keep { keep_color } else { reject_color }, right_label);
                                ui.add_space(6.0);
                                draw_preview(ui, self.conflict_remote_texture.as_ref());
                                ui.add_space(8.0);
                                ui.label(if english { format!("File name: {}", Self::asset_file_name(&item.job.asset)) } else { format!("Dateiname: {}", Self::asset_file_name(&item.job.asset)) });
                                ui.label(if english {
                                    format!("Capture time: {}", if item.job.asset.local_date_time.trim().is_empty() { "unknown" } else { &item.job.asset.local_date_time })
                                } else {
                                    format!("Aufnahmezeit: {}", if item.job.asset.local_date_time.trim().is_empty() { "unbekannt" } else { &item.job.asset.local_date_time })
                                });
                                ui.label(if english {
                                    format!("File size: {}", item.remote_size.map(Self::format_bytes_u64).unwrap_or_else(|| "not available".to_string()))
                                } else {
                                    format!("Dateigröße: {}", item.remote_size.map(Self::format_bytes_u64).unwrap_or_else(|| "nicht ermittelbar".to_string()))
                                });
                                ui.label(if english {
                                    format!("Resolution: {}", self.conflict_remote_dims.map(|(w,h)| format!("{} × {} px", w, h)).unwrap_or_else(|| "unknown".to_string()))
                                } else {
                                    format!("Auflösung: {}", self.conflict_remote_dims.map(|(w,h)| format!("{} × {} px", w, h)).unwrap_or_else(|| "unbekannt".to_string()))
                                });
                                ui.label(if english { format!("Destination folder: {}", item.job.folder_name) } else { format!("Zielordner: {}", item.job.folder_name) });
                                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                    if ui.button(if english { "Keep / overwrite with this Immich file" } else { "Diese Immich-Datei behalten / überschreiben" }).clicked() {
                                        if let Some(cur) = self.conflicts.get_mut(current_index) { cur.selected = true; }
                                    }
                                });
                            });
                    });
                    ui.add_space(10.0);
                    let selected_count = self.conflicts.iter().filter(|c| c.selected).count();
                    ui.label(if english {
                        format!("Marked for overwrite: {} of {}", selected_count, total_groups)
                    } else {
                        format!("Für Überschreiben markiert: {} von {}", selected_count, total_groups)
                    });
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(if english { "Overwrite selected" } else { "Ausgewählte überschreiben" }).clicked() { overwrite_selected = true; }
                        if ui.button(if english { "Overwrite all" } else { "Alle überschreiben" }).clicked() { overwrite_all = true; }
                        if ui.button(if english { "Keep the larger file in each case" } else { "Jeweils größere Datei behalten" }).clicked() { keep_larger = true; }
                        if ui.button(if english { "Close" } else { "Schließen" }).clicked() { close_clicked = true; }
                    });
                });
        });

        if close_clicked {
            self.conflict_popup = false;
        }

        let jobs = if overwrite_all {
            Some(
                self.conflicts
                    .iter()
                    .map(|x| x.job.clone())
                    .collect::<Vec<_>>(),
            )
        } else if overwrite_selected {
            Some(
                self.conflicts
                    .iter()
                    .filter(|x| x.selected)
                    .map(|x| x.job.clone())
                    .collect::<Vec<_>>(),
            )
        } else if keep_larger {
            Some(
                self.conflicts
                    .iter()
                    .filter(|x| {
                        x.remote_size
                            .map(|size| size > x.local_size)
                            .unwrap_or(false)
                    })
                    .map(|x| x.job.clone())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };

        if let Some(jobs) = jobs {
            if jobs.is_empty() {
                self.status = if english {
                    "No Immich file could be confirmed as larger than the existing file."
                } else {
                    "Keine sicher ermittelte Immich-Datei ist größer als die vorhandene Datei."
                }
                .to_string();
                self.conflict_popup = false;
            } else if let Err(e) = self.start_conflict_overwrite(jobs) {
                self.status = if english {
                    format!("Error: {}", e)
                } else {
                    format!("Fehler: {}", e)
                };
            }
        }
    }
}

impl eframe::App for ImmichApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::apply_style(ctx, self.dark_mode);
        self.poll_download_events();
        self.poll_thumbnail_events(ctx);

        self.ui_main_panels(ctx);

        self.show_settings_popup(ctx);
        self.show_info_popup(ctx);
        self.show_download_popup(ctx);
        self.show_download_protocol_popup(ctx);
        self.show_conflict_popup(ctx);
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn load_window_icon() -> egui::viewport::IconData {
    let image = image::load_from_memory(include_bytes!("../app.png"))
        .expect("Das eingebettete Programmsymbol konnte nicht geladen werden.")
        .resize_exact(256, 256, image::imageops::FilterType::Lanczos3)
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::viewport::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Media Backup Manager")
            .with_icon(load_window_icon())
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([900.0, 600.0])
            .with_maximized(false),
        ..Default::default()
    };

    eframe::run_native(
        "Media Backup Manager",
        options,
        Box::new(|cc| {
            let mut app = ImmichApp::default();

            let load_header_icon = |id: &str| {
                let decoded = image::load_from_memory(include_bytes!("../app.png"))
                    .expect("Das eingebettete Header-Symbol konnte nicht geladen werden.")
                    .into_rgba8();
                let size = [decoded.width() as usize, decoded.height() as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, decoded.as_raw());
                cc.egui_ctx
                    .load_texture(id, color_image, egui::TextureOptions::LINEAR)
            };

            app.header_logo_light_texture = Some(load_header_icon("header_icon_light"));
            app.header_logo_dark_texture = Some(load_header_icon("header_icon_dark"));
            Ok(Box::new(app))
        }),
    )
}
