use super::{SelectableAlbum, YearBucket};
use eframe::egui;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(super) enum AlbumView {
    #[default]
    Covers,
    List,
}

const CARD_WIDTH: f32 = 190.0;
const CARD_GAP: f32 = 12.0;
const CARD_PADDING: f32 = 10.0;
const TITLE_LINE_HEIGHT: f32 = 18.0;
const TITLE_HEIGHT: f32 = TITLE_LINE_HEIGHT * 3.0;
const LIST_HEIGHT: f32 = 42.0;
const YEAR_HEIGHT: f32 = 92.0;

fn grid_geometry(available_width: f32) -> (usize, f32) {
    let width = available_width.max(1.0);
    let columns = (((width + CARD_GAP) / (CARD_WIDTH + CARD_GAP)).floor() as usize).clamp(1, 12);
    // Keep cards fixed in size; only shrink a single card in an unusually narrow viewport.
    (columns, width.min(CARD_WIDTH))
}

fn title_job(text: &str, width: f32, color: egui::Color32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::simple(
        text.to_owned(),
        egui::FontId::proportional(13.0),
        color,
        width.max(1.0),
    );
    job.wrap.max_rows = 3;
    job.sections[0].format.line_height = Some(TITLE_LINE_HEIGHT);
    job
}

fn cover_uv(size: egui::Vec2) -> egui::Rect {
    // Center-crop to a square, without stretching the source image.
    let side = size.x.min(size.y).max(1.0);
    let extent = egui::vec2(side / size.x.max(1.0), side / size.y.max(1.0));
    egui::Rect::from_center_size(egui::pos2(0.5, 0.5), extent)
}

fn pixel_aligned_rect(ui: &egui::Ui, rect: egui::Rect) -> egui::Rect {
    let pixels_per_point = ui.ctx().pixels_per_point().max(1.0);
    let snap = |value: f32| (value * pixels_per_point).round() / pixels_per_point;
    egui::Rect::from_min_max(
        egui::pos2(snap(rect.min.x), snap(rect.min.y)),
        egui::pos2(snap(rect.max.x), snap(rect.max.y)),
    )
}

fn clipped_text(ui: &egui::Ui, rect: egui::Rect, text: String, size: f32, color: egui::Color32) {
    let mut job = egui::text::LayoutJob::simple(
        text,
        egui::FontId::proportional(size),
        color,
        rect.width().max(1.0),
    );
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    let galley = ui.fonts(|fonts| fonts.layout_job(job));
    let pos = egui::pos2(rect.left(), rect.center().y - galley.size().y / 2.0);
    ui.painter()
        .with_clip_rect(rect.intersect(ui.clip_rect()))
        .galley(pos, galley, color);
}

fn draw_album(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    item: &mut SelectableAlbum,
    view: AlbumView,
    english: bool,
) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    let dark = ui.visuals().dark_mode;
    let text = ui.visuals().text_color();
    let muted = ui.visuals().weak_text_color();
    let mut response = ui.interact(
        rect,
        ui.id().with(("album", &item.album.id)),
        egui::Sense::click(),
    );

    let fill = if item.selected {
        if dark {
            egui::Color32::from_rgb(20, 51, 57)
        } else {
            egui::Color32::from_rgb(233, 249, 251)
        }
    } else if response.hovered() {
        if dark {
            ui.visuals().widgets.hovered.bg_fill
        } else {
            egui::Color32::from_rgb(236, 239, 243)
        }
    } else if dark {
        egui::Color32::from_rgb(24, 32, 39)
    } else {
        egui::Color32::from_rgb(244, 246, 248)
    };
    let pixels_per_point = ui.ctx().pixels_per_point().max(1.0);
    let one_pixel = 1.0_f32 / pixels_per_point;
    let stroke = if item.selected || response.has_focus() {
        egui::Stroke::new(
            2.0_f32 / pixels_per_point,
            egui::Color32::from_rgb(10, 159, 166),
        )
    } else if dark {
        egui::Stroke::new(one_pixel, ui.visuals().widgets.inactive.bg_stroke.color)
    } else if response.hovered() {
        egui::Stroke::new(one_pixel, egui::Color32::from_rgb(170, 180, 190))
    } else {
        egui::Stroke::new(one_pixel, egui::Color32::from_rgb(199, 205, 212))
    };

    // Draw album cards on the physical pixel grid just like year cards.
    // Keeping the full stroke inside the card prevents individual left edges
    // from being clipped or rendered thinner at Windows display scaling.
    let paint_rect = pixel_aligned_rect(ui, rect);
    let stroke_rect = paint_rect.shrink(one_pixel * 0.5);
    let painter = ui.painter().with_clip_rect(ui.clip_rect());
    painter.rect_filled(paint_rect, 8.0, fill);
    painter.rect_stroke(stroke_rect, 8.0 - one_pixel * 0.5, stroke);

    let inner = rect.shrink(CARD_PADDING);
    let title_top = if view == AlbumView::Covers {
        let image_rect = egui::Rect::from_min_size(inner.min, egui::Vec2::splat(inner.width()));
        if let Some(texture) = &item.thumbnail {
            let mut image_ui = ui.new_child(egui::UiBuilder::new().max_rect(image_rect));
            image_ui.set_clip_rect(image_rect.intersect(ui.clip_rect()));
            egui::Image::new(texture)
                .uv(cover_uv(texture.size_vec2()))
                .rounding(6.0)
                .paint_at(&image_ui, image_rect);
        } else {
            painter.rect_filled(image_rect, 6.0, ui.visuals().extreme_bg_color);
            clipped_text(
                ui,
                image_rect,
                if english { "Preview" } else { "Vorschau" }.to_owned(),
                12.0,
                muted,
            );
        }
        image_rect.bottom() + 8.0
    } else {
        inner.top()
    };

    let checkbox_rect =
        egui::Rect::from_min_size(egui::pos2(inner.left(), title_top), egui::vec2(22.0, 22.0));
    let mut checkbox_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt(("album_checkbox", &item.album.id))
            .max_rect(checkbox_rect),
    );
    // Native checkbox strokes and hover expansion extend beyond the widget rectangle.
    // Clip to the containing card, not to the checkbox edge.
    checkbox_ui.set_clip_rect(rect.intersect(ui.clip_rect()));
    if !checkbox_ui.visuals().dark_mode {
        // egui's light theme has no inactive border by default.
        checkbox_ui.visuals_mut().widgets.inactive.bg_stroke =
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(150));
    }
    let checkbox = checkbox_ui.add(egui::Checkbox::new(&mut item.selected, ""));
    let checkbox_clicked = checkbox.clicked();
    checkbox.on_hover_text(if english {
        "Select album"
    } else {
        "Album auswählen"
    });
    if response.clicked() && !checkbox_clicked {
        item.selected = !item.selected;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            item.selected,
            &item.album.album_name,
        )
    });
    response.on_hover_text(&item.album.album_name);

    let files = if english {
        format!("{} files", item.album.asset_count)
    } else {
        format!("{} Dateien", item.album.asset_count)
    };
    let ownership = if item.album.shared {
        if english {
            "shared"
        } else {
            "geteilt"
        }
    } else if english {
        "own"
    } else {
        "eigen"
    };

    if view == AlbumView::Covers {
        let title_rect = egui::Rect::from_min_max(
            egui::pos2(checkbox_rect.right() + 4.0, title_top),
            egui::pos2(inner.right(), title_top + TITLE_HEIGHT),
        );
        let galley = ui.fonts(|fonts| {
            fonts.layout_job(title_job(&item.album.album_name, title_rect.width(), text))
        });
        painter
            .with_clip_rect(title_rect.intersect(ui.clip_rect()))
            .galley(title_rect.min, galley, text);

        let meta_rect = egui::Rect::from_min_max(
            egui::pos2(inner.left(), title_rect.bottom() + 8.0),
            inner.max,
        );
        let badge_width = 54.0;
        let badge = egui::Rect::from_min_max(
            egui::pos2(meta_rect.right() - badge_width, meta_rect.top()),
            meta_rect.max,
        );
        let count_rect = egui::Rect::from_min_max(
            meta_rect.min,
            egui::pos2(badge.left() - 4.0, meta_rect.bottom()),
        );
        clipped_text(ui, count_rect, files, 12.0, muted);
        painter.rect_filled(badge, 5.0, ui.visuals().extreme_bg_color);
        clipped_text(
            ui,
            badge.shrink2(egui::vec2(4.0, 0.0)),
            ownership.to_owned(),
            11.0,
            muted,
        );
    } else {
        // One bounded text area also works for narrow windows and very long album names.
        let title_rect = egui::Rect::from_min_max(
            egui::pos2(checkbox_rect.right() + 6.0, inner.top()),
            inner.max,
        );
        clipped_text(
            ui,
            title_rect,
            format!("{}  ·  {}  ·  {}", item.album.album_name, files, ownership),
            13.0,
            text,
        );
    }
}

pub(super) fn show(
    ui: &mut egui::Ui,
    albums: &mut [SelectableAlbum],
    filter: &str,
    view: AlbumView,
    english: bool,
) -> egui::scroll_area::ScrollAreaOutput<()> {
    let filter = filter.to_lowercase();
    let visible: Vec<usize> = albums
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            filter.is_empty() || item.album.album_name.to_lowercase().contains(&filter)
        })
        .map(|(index, _)| index)
        .collect();

    egui::ScrollArea::vertical()
        .id_salt(("album_preview_scroll", view))
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .min_scrolled_height(0.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if visible.is_empty() {
                ui.label(if albums.is_empty() {
                    if english {
                        "Load albums to get started."
                    } else {
                        "Bitte zuerst Alben laden."
                    }
                } else if english {
                    "No matching albums."
                } else {
                    "Keine passenden Alben gefunden."
                });
                return;
            }
            // Calculate inside the viewport: the solid scrollbar is already excluded.
            let viewport_width = ui.available_width().max(1.0);
            let (columns, width) = if view == AlbumView::Covers {
                grid_geometry(viewport_width)
            } else {
                (1, viewport_width)
            };
            let height = if view == AlbumView::Covers {
                width + 8.0 + TITLE_HEIGHT + 8.0 + 22.0
            } else {
                LIST_HEIGHT
            };
            ui.spacing_mut().item_spacing.y = if view == AlbumView::Covers {
                CARD_GAP
            } else {
                5.0
            };
            for row in visible.chunks(columns) {
                let (row_rect, _) = ui
                    .allocate_exact_size(egui::vec2(viewport_width, height), egui::Sense::hover());
                for (column, &index) in row.iter().enumerate() {
                    let rect = egui::Rect::from_min_size(
                        row_rect.min + egui::vec2(column as f32 * (width + CARD_GAP), 0.0),
                        egui::vec2(width, height),
                    );
                    draw_album(ui, rect, &mut albums[index], view, english);
                }
            }
        })
}

fn draw_year(ui: &mut egui::Ui, rect: egui::Rect, item: &mut YearBucket, english: bool) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    let mut response = ui.interact(
        rect,
        ui.id().with(("year", &item.year)),
        egui::Sense::click(),
    );
    let dark = ui.visuals().dark_mode;
    let fill = if item.selected {
        if dark {
            egui::Color32::from_rgb(20, 51, 57)
        } else {
            egui::Color32::from_rgb(233, 249, 251)
        }
    } else if response.hovered() {
        if dark {
            ui.visuals().widgets.hovered.bg_fill
        } else {
            egui::Color32::from_rgb(236, 239, 243)
        }
    } else if dark {
        egui::Color32::from_rgb(24, 32, 39)
    } else {
        egui::Color32::from_rgb(244, 246, 248)
    };
    let pixels_per_point = ui.ctx().pixels_per_point().max(1.0);
    let one_pixel = 1.0_f32 / pixels_per_point;
    let stroke = if item.selected || response.has_focus() {
        egui::Stroke::new(
            2.0_f32 / pixels_per_point,
            egui::Color32::from_rgb(10, 159, 166),
        )
    } else if dark {
        egui::Stroke::new(one_pixel, ui.visuals().widgets.inactive.bg_stroke.color)
    } else if response.hovered() {
        egui::Stroke::new(one_pixel, egui::Color32::from_rgb(170, 180, 190))
    } else {
        egui::Stroke::new(one_pixel, egui::Color32::from_rgb(199, 205, 212))
    };

    // Snap the complete card to the physical pixel grid. The grid can produce
    // fractional UI coordinates at Windows display scaling, which previously
    // made individual left borders look thinner or disappear. The border is
    // then inset by half a physical pixel and painted without clipping it to
    // the card itself, so every edge is rendered consistently in both themes.
    let paint_rect = pixel_aligned_rect(ui, rect);
    let stroke_rect = paint_rect.shrink(one_pixel * 0.5);
    let painter = ui.painter().with_clip_rect(ui.clip_rect());
    painter.rect_filled(paint_rect, 8.0, fill);
    painter.rect_stroke(stroke_rect, 8.0 - one_pixel * 0.5, stroke);

    let inner = rect.shrink(CARD_PADDING);
    let checkbox_rect = egui::Rect::from_min_size(inner.min, egui::vec2(22.0, 24.0));
    let mut checkbox_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt(("year_checkbox", &item.year))
            .max_rect(checkbox_rect),
    );
    // Native checkbox strokes and hover expansion extend beyond the widget rectangle.
    // Clip to the containing card, not to the checkbox edge.
    checkbox_ui.set_clip_rect(rect.intersect(ui.clip_rect()));
    if !checkbox_ui.visuals().dark_mode {
        // Keep the light-theme checkbox visible against the year-card surface.
        checkbox_ui.visuals_mut().widgets.inactive.bg_stroke =
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(154, 163, 173));
    }
    let checkbox = checkbox_ui.add(egui::Checkbox::new(&mut item.selected, ""));
    let checkbox_clicked = checkbox.clicked();
    checkbox.on_hover_text(if english {
        "Select year folder"
    } else {
        "Jahresordner auswählen"
    });
    if response.clicked() && !checkbox_clicked {
        item.selected = !item.selected;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            item.selected,
            &item.year,
        )
    });
    let files = if english {
        format!("{} files", item.count)
    } else {
        format!("{} Dateien", item.count)
    };
    let size = super::ImmichApp::format_bytes_i64(item.total_size);
    response.on_hover_text(format!("{} · {} · {}", item.year, files, size));
    clipped_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(checkbox_rect.right() + 4.0, inner.top()),
            egui::pos2(inner.right(), inner.top() + 24.0),
        ),
        item.year.clone(),
        16.0,
        if dark {
            ui.visuals().text_color()
        } else {
            egui::Color32::from_rgb(47, 52, 58)
        },
    );
    clipped_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(inner.left(), inner.top() + 30.0),
            egui::pos2(inner.right(), inner.top() + 48.0),
        ),
        files,
        12.0,
        if dark {
            ui.visuals().weak_text_color()
        } else {
            egui::Color32::from_rgb(110, 118, 129)
        },
    );
    clipped_text(
        ui,
        egui::Rect::from_min_max(egui::pos2(inner.left(), inner.top() + 51.0), inner.max),
        size,
        12.0,
        if dark {
            ui.visuals().weak_text_color()
        } else {
            egui::Color32::from_rgb(110, 118, 129)
        },
    );
}

pub(super) fn show_years(
    ui: &mut egui::Ui,
    years: &mut [YearBucket],
    filter: &str,
    scroll_id: &str,
    english: bool,
) -> egui::scroll_area::ScrollAreaOutput<()> {
    let filter = filter.to_lowercase();
    let visible: Vec<usize> = years
        .iter()
        .enumerate()
        .filter(|(_, item)| filter.is_empty() || item.year.to_lowercase().contains(&filter))
        .map(|(index, _)| index)
        .collect();
    egui::ScrollArea::vertical()
        .id_salt(scroll_id)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .min_scrolled_height(0.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if visible.is_empty() {
                ui.label(if years.is_empty() {
                    if english {
                        "Load year folders to get started."
                    } else {
                        "Bitte zuerst Jahresordner laden."
                    }
                } else if english {
                    "No matching years."
                } else {
                    "Keine passenden Jahre gefunden."
                });
                return;
            }
            // Share the album geometry, measured after subtracting the scrollbar.
            let viewport_width = ui.available_width().max(1.0);
            let (columns, width) = grid_geometry(viewport_width);
            ui.spacing_mut().item_spacing.y = CARD_GAP;
            for row in visible.chunks(columns) {
                let (row_rect, _) = ui.allocate_exact_size(
                    egui::vec2(viewport_width, YEAR_HEIGHT),
                    egui::Sense::hover(),
                );
                for (column, &index) in row.iter().enumerate() {
                    let rect = egui::Rect::from_min_size(
                        row_rect.min + egui::vec2(column as f32 * (width + CARD_GAP), 0.0),
                        egui::vec2(width, YEAR_HEIGHT),
                    );
                    draw_year(ui, rect, &mut years[index], english);
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn albums() -> Vec<SelectableAlbum> {
        (0..40)
            .map(|index| SelectableAlbum {
                album: crate::Album {
                    id: format!("album-{index}"),
                    album_name: format!(
                        "Album {index} – Ein langer Titel mit mehreren Wörtern und Umlauten"
                    ),
                    asset_count: index * 1000,
                    shared: index % 2 == 0,
                    album_thumbnail_asset_id: None,
                },
                selected: false,
                thumbnail: None,
            })
            .collect()
    }

    fn years() -> Vec<YearBucket> {
        (1990..2027)
            .map(|year| YearBucket {
                year: year.to_string(),
                count: 53823,
                total_size: 4_500_000_000,
                selected: false,
            })
            .collect()
    }

    fn painted_text(output: egui::FullOutput) -> Vec<(String, egui::Rect, egui::Rect)> {
        fn collect(
            shape: &egui::Shape,
            clip: egui::Rect,
            result: &mut Vec<(String, egui::Rect, egui::Rect)>,
        ) {
            match shape {
                egui::Shape::Text(text) => result.push((
                    text.galley.job.text.clone(),
                    // The paint origin is the right/center anchor for aligned text,
                    // not necessarily its top-left corner.
                    text.galley.rect.translate(text.pos.to_vec2()),
                    clip,
                )),
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, clip, result);
                    }
                }
                _ => {}
            }
        }
        let mut result = Vec::new();
        for shape in output.shapes {
            collect(&shape.shape, shape.clip_rect, &mut result);
        }
        result
    }

    #[test]
    fn actual_main_panels_keep_header_and_actions_in_their_assigned_regions() {
        for english in [false, true] {
            let ctx = egui::Context::default();
            let mut app = crate::ImmichApp::default();
            app.english = english;
            app.albums = albums();
            crate::ImmichApp::apply_style(&ctx, true);
            app.header_logo_dark_texture = Some(ctx.load_texture(
                "panel_header_test",
                egui::ColorImage::new([90, 45], egui::Color32::WHITE),
                egui::TextureOptions::LINEAR,
            ));
            for (width, height) in [
                (900.0, 600.0),
                (1280.0, 720.0),
                (3440.0, 1440.0),
                (900.0, 600.0),
            ] {
                let mut painted = Vec::new();
                for _ in 0..3 {
                    let input = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, height),
                        )),
                        ..Default::default()
                    };
                    painted = painted_text(ctx.run(input, |ctx| app.ui_main_panels(ctx)));
                }
                let description = painted
                    .iter()
                    .find(|(text, _, _)| text.contains('\n') && text.contains("Immich"))
                    .unwrap();
                assert!(
                    description.1.top() >= 0.0 && description.1.bottom() < crate::HEADER_HEIGHT
                );
                assert!(description.2.contains_rect(description.1));
                let download = painted
                    .iter()
                    .find(|(text, _, _)| {
                        text == if english {
                            "↓ Download"
                        } else {
                            "↓ Herunterladen"
                        }
                    })
                    .unwrap();
                assert!(download.1.left() >= width - 295.0);
                assert!(download.1.top() >= height - 104.0);
                assert!(download.1.bottom() < height - 34.0);
                let load = painted
                    .iter()
                    .find(|(text, _, _)| {
                        text == if english {
                            "Test connection / load albums"
                        } else {
                            "Verbindung testen / Alben laden"
                        }
                    })
                    .unwrap();
                let search = painted
                    .iter()
                    .find(|(text, _, _)| {
                        text == if english {
                            "Search albums ..."
                        } else {
                            "Album suchen ..."
                        }
                    })
                    .unwrap();
                assert!(load.1.top() > crate::HEADER_HEIGHT);
                assert!(load.1.right() < search.1.left());
                assert!((load.1.center().y - search.1.center().y).abs() <= 5.0);
                for label in [
                    if english {
                        "Album covers"
                    } else {
                        "Albumcover"
                    },
                    if english { "List" } else { "Liste" },
                ] {
                    assert!(painted.iter().any(|(text, _, _)| text == label));
                }
            }
        }
    }

    #[test]
    fn about_footer_stays_visible_after_window_resize() {
        for english in [false, true] {
            let ctx = egui::Context::default();
            let mut app = crate::ImmichApp::default();
            app.english = english;
            app.info_popup = true;
            crate::ImmichApp::apply_style(&ctx, true);
            for (width, height) in [(1280.0, 720.0), (900.0, 600.0), (1280.0, 720.0)] {
                let mut painted = Vec::new();
                for _ in 0..3 {
                    let input = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, height),
                        )),
                        ..Default::default()
                    };
                    painted = painted_text(ctx.run(input, |ctx| app.show_info_popup(ctx)));
                }
                let close = painted
                    .iter()
                    .find(|(text, _, _)| text == if english { "Close" } else { "Schließen" })
                    .unwrap();
                assert!(close.1.bottom() <= height - 16.0);
                assert!(close.2.contains_rect(close.1));
                assert!(
                    painted.iter().any(|(text, rect, clip)| {
                        text == "Copyright © 2026 Ralf Ebert"
                            && (rect.center().y - close.1.center().y).abs() < 5.0
                            && clip.contains_rect(*rect)
                    }),
                    "About footer: english={english}, viewport={width}x{height}, close={close:?}, copyright={:?}",
                    painted.iter().filter(|(text, _, _)| text == "Copyright © 2026 Ralf Ebert").collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn compact_header_centers_description_and_keeps_connection_fields_in_settings() {
        fn texts(shape: &egui::Shape, result: &mut Vec<(String, f32)>) {
            match shape {
                egui::Shape::Text(text) => result.push((
                    text.galley.job.text.clone(),
                    text.pos.y + text.galley.size().y / 2.0,
                )),
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        texts(shape, result);
                    }
                }
                _ => {}
            }
        }
        for english in [false, true] {
            let ctx = egui::Context::default();
            let mut app = crate::ImmichApp::default();
            app.english = english;
            app.server = "https://example.invalid".to_owned();
            app.api_key = "not-a-real-key".to_owned();
            crate::ImmichApp::apply_style(&ctx, true);
            app.header_logo_dark_texture = Some(ctx.load_texture(
                "header_test",
                egui::ColorImage::new([96, 34], egui::Color32::WHITE),
                egui::TextureOptions::LINEAR,
            ));
            for width in [900.0, 1280.0, 1920.0] {
                let mut expected_center = 0.0;
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 600.0),
                    )),
                    ..Default::default()
                };
                let output = ctx.run(input, |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let start_y = ui.cursor().top();
                        expected_center = start_y + crate::HEADER_HEIGHT / 2.0;
                        app.ui_header(ui);
                        assert!(ui.cursor().top() - start_y < 115.0);
                    });
                });
                let mut painted = Vec::new();
                for shape in output.shapes {
                    texts(&shape.shape, &mut painted);
                }
                let description = painted
                    .iter()
                    .find(|(text, _)| text.contains('\n') && text.contains("Immich"))
                    .unwrap();
                assert!((description.1 - expected_center).abs() < 1.0);
                assert!(painted
                    .iter()
                    .any(|(text, _)| text == if english { "Settings" } else { "Einstellungen" }));
                for expected in [
                    if english { "☀ Light" } else { "☀ Hell" },
                    if english { "Deutsch" } else { "English" },
                ] {
                    assert!(painted.iter().any(|(text, _)| text == expected));
                }
                assert!(!painted.iter().any(|(text, _)| {
                    [
                        "Fotos und Videos",
                        "Photos and videos",
                        "Eigene Alben",
                        "Own albums",
                        "Geteilte Alben",
                        "Shared albums",
                    ]
                    .contains(&text.as_str())
                }));
                assert!(!painted
                    .iter()
                    .any(|(text, _)| text.contains("example.invalid")
                        || text.contains("not-a-real-key")
                        || text.contains("Verbindung testen")));
            }
            app.settings_popup = true;
            let mut settings_text = Vec::new();
            for _ in 0..3 {
                settings_text.clear();
                let output = ctx.run(egui::RawInput::default(), |ctx| {
                    app.show_settings_popup(ctx);
                });
                for shape in output.shapes {
                    texts(&shape.shape, &mut settings_text);
                }
            }
            for expected in [
                "https://example.invalid",
                if english {
                    "Delete saved API key"
                } else {
                    "Gespeicherten API-Key löschen"
                },
            ] {
                assert!(settings_text.iter().any(|(text, _)| text == expected));
            }
            assert!(!settings_text
                .iter()
                .any(|(text, _)| text.contains("not-a-real-key")));
        }
    }

    #[test]
    fn rendered_views_keep_content_inside_the_scrollbar_at_different_sizes() {
        let ctx = egui::Context::default();
        let mut albums = albums();
        for dark in [false, true] {
            crate::ImmichApp::apply_style(&ctx, dark);
            for english in [false, true] {
                for view in [AlbumView::Covers, AlbumView::List] {
                    for width in [240.0, 420.0, 550.0, 960.0, 1500.0, 3000.0, 550.0] {
                        for _ in 0..3 {
                            let input = egui::RawInput {
                                screen_rect: Some(egui::Rect::from_min_size(
                                    egui::Pos2::ZERO,
                                    egui::vec2(width, 360.0),
                                )),
                                ..Default::default()
                            };
                            let _ = ctx.run(input, |ctx| {
                                egui::CentralPanel::default().show(ctx, |ui| {
                                    let output = show(ui, &mut albums, "", view, english);
                                    assert!(
                                        output.content_size.x <= output.inner_rect.width() + 1.0
                                    );
                                    assert!(output.inner_rect.bottom() <= 360.0);
                                });
                            });
                        }
                    }
                }
            }
            for scroll_id in ["no_album_year_scroll", "all_year_scroll"] {
                let mut years = years();
                for width in [240.0, 420.0, 550.0, 960.0, 1500.0, 3000.0, 550.0] {
                    for _ in 0..3 {
                        let input = egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(width, 360.0),
                            )),
                            ..Default::default()
                        };
                        let _ = ctx.run(input, |ctx| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                let output = show_years(ui, &mut years, "", scroll_id, false);
                                assert!(output.content_size.x <= output.inner_rect.width() + 1.0);
                                assert!(output.inner_rect.bottom() <= 360.0);
                            });
                        });
                    }
                }
            }
        }
    }

    #[test]
    fn card_and_checkbox_click_once_and_selection_survives_view_and_filter_changes() {
        let ctx = egui::Context::default();
        let mut albums = albums();
        let mut frame = |view, filter: &str, events| {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(ui, &mut albums, filter, view, false);
                });
            });
            albums[0].selected
        };
        frame(AlbumView::Covers, "", vec![]);
        frame(AlbumView::Covers, "", vec![]);
        // The first click hits the cover; the second hits its checkbox below the image.
        for (pos, selected) in [
            (egui::pos2(90.0, 90.0), true),
            (egui::pos2(25.0, 205.0), false),
        ] {
            frame(
                AlbumView::Covers,
                "",
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            assert_eq!(
                frame(
                    AlbumView::Covers,
                    "",
                    vec![egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE
                    },]
                ),
                selected
            );
            assert_eq!(
                frame(AlbumView::List, "not a matching name", vec![]),
                selected
            );
            assert_eq!(frame(AlbumView::List, "ALBUM", vec![]), selected);
            assert_eq!(frame(AlbumView::Covers, "", vec![]), selected);
        }
    }

    #[test]
    fn grid_stays_inside_viewport_at_every_width() {
        let twelve_columns_width = 12.0 * CARD_WIDTH + 11.0 * CARD_GAP;
        assert_eq!(grid_geometry(twelve_columns_width).0, 12);
        assert_eq!(grid_geometry(twelve_columns_width - 1.0).0, 11);
        for pixels in 1..=7680 {
            let available = pixels as f32 / 2.0;
            let (columns, width) = grid_geometry(available);
            assert!((1..=12).contains(&columns));
            assert!(width <= CARD_WIDTH);
            assert!(columns as f32 * width + (columns - 1) as f32 * CARD_GAP <= available.max(1.0));
            if available >= CARD_WIDTH {
                assert_eq!(width, CARD_WIDTH);
            }
        }
    }

    #[test]
    fn covers_crop_portrait_and_landscape_without_stretching() {
        for size in [
            egui::vec2(400.0, 200.0),
            egui::vec2(200.0, 400.0),
            egui::vec2(200.0, 200.0),
        ] {
            let uv = cover_uv(size);
            assert_eq!(uv.center(), egui::pos2(0.5, 0.5));
            assert!((uv.width() * size.x - uv.height() * size.y).abs() < 0.001);
            assert!(uv.min.x >= 0.0 && uv.min.y >= 0.0 && uv.max.x <= 1.0 && uv.max.y <= 1.0);
        }
    }

    #[test]
    fn titles_use_three_real_font_rows_before_elision() {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            ctx.fonts(|fonts| {
                let short = fonts.layout_job(title_job("Urlaub", 144.0, egui::Color32::WHITE));
                assert_eq!(short.rows.len(), 1);
                assert!(!short.elided);
                let three = fonts.layout_job(title_job("Zeile 1\nZeile 2\nZeile 3", 144.0, egui::Color32::WHITE));
                assert_eq!(three.rows.len(), 3);
                assert!(!three.elided);
                for name in [
                    "Schnelligkeitswettbewerb der Feuerwehr Lammertsfehn mit Gästen und Siegerehrung 2026".to_owned(),
                    "W".repeat(150),
                    "ÄÖÜß日本語 Urlaub ".repeat(20),
                    "Zeile 1\nZeile 2\nZeile 3\nZeile 4".to_owned(),
                ] {
                    let galley = fonts.layout_job(title_job(&name, 144.0, egui::Color32::WHITE));
                    assert_eq!(galley.rows.len(), 3);
                    assert!(galley.elided);
                    assert!(galley.size().y <= TITLE_HEIGHT + 1.0);
                    assert!(galley.size().x <= 145.0);
                }
            });
        });
    }
    #[test]
    fn year_toolbar_filters_and_status_stay_in_their_assigned_regions() {
        fn check_media_text(shape: &egui::Shape) {
            match shape {
                egui::Shape::Text(text)
                    if [
                        "Photos and videos",
                        "Fotos und Videos",
                        "Photos only",
                        "Nur Fotos",
                        "Videos only",
                        "Nur Videos",
                    ]
                    .contains(&text.galley.job.text.as_str()) =>
                {
                    assert!(
                        !text.galley.elided,
                        "Media selection must remain fully readable: {}",
                        text.galley.job.text
                    );
                }
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        check_media_text(shape);
                    }
                }
                _ => {}
            }
        }
        for english in [false, true] {
            let ctx = egui::Context::default();
            let mut app = crate::ImmichApp::default();
            app.english = english;
            app.year_buckets = years();
            app.all_year_buckets = years();
            app.status =
                "53823 Fotos/Videos insgesamt geladen, gruppiert in 33 Jahresordner.".to_owned();
            crate::ImmichApp::apply_style(&ctx, true);
            for tab in [crate::ActiveTab::NoAlbum, crate::ActiveTab::AllByYear] {
                app.active_tab = tab;
                for mode in [
                    crate::MediaMode::All,
                    crate::MediaMode::Photos,
                    crate::MediaMode::Videos,
                ] {
                    app.media_mode = mode;
                    for width in [
                        900.0, 1024.0, 1152.0, 1279.0, 1280.0, 1281.0, 1366.0, 1600.0, 1920.0,
                        3440.0, 900.0,
                    ] {
                        let mut painted = Vec::new();
                        for _ in 0..3 {
                            let input = egui::RawInput {
                                screen_rect: Some(egui::Rect::from_min_size(
                                    egui::Pos2::ZERO,
                                    egui::vec2(width, 720.0),
                                )),
                                ..Default::default()
                            };
                            let output = ctx.run(input, |ctx| app.ui_main_panels(ctx));
                            for shape in &output.shapes {
                                check_media_text(&shape.shape);
                            }
                            painted = painted_text(output);
                        }
                        let find = |label: &str| {
                            painted.iter().find(|(text, _, _)| text == label).unwrap()
                        };
                        let count = find(if english {
                            "0 year folders selected"
                        } else {
                            "0 Jahresordner ausgewählt"
                        });
                        let filters = [
                            match mode {
                                crate::MediaMode::All => {
                                    if english {
                                        "Photos and videos"
                                    } else {
                                        "Fotos und Videos"
                                    }
                                }
                                crate::MediaMode::Photos => {
                                    if english {
                                        "Photos only"
                                    } else {
                                        "Nur Fotos"
                                    }
                                }
                                crate::MediaMode::Videos => {
                                    if english {
                                        "Videos only"
                                    } else {
                                        "Nur Videos"
                                    }
                                }
                            },
                            if english {
                                "Own albums"
                            } else {
                                "Eigene Alben"
                            },
                            if english {
                                "Shared albums"
                            } else {
                                "Geteilte Alben"
                            },
                        ];
                        for label in filters {
                            let item = find(label);
                            assert!(item.1.top() > crate::HEADER_HEIGHT, "{item:?}");
                            assert!(
                                item.1.right() < width - 295.0,
                                "Toolbar overflow: english={english}, width={width}, item={item:?}"
                            );
                            assert!(
                                item.2.contains_rect(item.1),
                                "Toolbar clipping: english={english}, width={width}, item={item:?}"
                            );
                            if width >= 3440.0 {
                                assert!(item.1.left() > count.1.right());
                                assert!((item.1.center().y - count.1.center().y).abs() < 5.0);
                            }
                        }
                        let status = find(&app.status);
                        let version = find(&format!("Version {}", env!("CARGO_PKG_VERSION")));
                        assert!(status.1.top() >= 720.0 - 34.0);
                        assert!(status.1.right() < version.1.left());
                        assert!(status.2.contains_rect(status.1));
                        assert!(
                        version.2.contains_rect(version.1),
                        "Version clipping: english={english}, width={width}, version={version:?}"
                    );
                    }
                }
            }
            app.settings_popup = true;
            let mut painted = Vec::new();
            for _ in 0..3 {
                painted = painted_text(ctx.run(egui::RawInput::default(), |ctx| {
                    app.show_settings_popup(ctx)
                }));
            }
            assert!(!painted.iter().any(|(text, _, _)| text == &app.status));
            assert!(!painted.iter().any(|(text, _, _)| [
                "☀ Light",
                "☀ Hell",
                "English",
                "Deutsch"
            ]
            .contains(&text.as_str())));
        }
    }

    #[test]
    fn checkbox_strokes_are_not_clipped_at_the_widget_edge() {
        fn inspect(shape: &egui::Shape, clip: egui::Rect, found: &mut usize) {
            match shape {
                egui::Shape::Rect(shape)
                    if shape.rect.width() <= 24.0 && shape.stroke.width > 0.0 =>
                {
                    *found += 1;
                    assert!(
                        clip.contains_rect(shape.rect.expand(shape.stroke.width / 2.0)),
                        "checkbox={:?}, clip={clip:?}",
                        shape.rect
                    );
                }
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        inspect(shape, clip, found);
                    }
                }
                _ => {}
            }
        }
        for dark in [false, true] {
            for selected in [false, true] {
                for kind in 0..3 {
                    let ctx = egui::Context::default();
                    crate::ImmichApp::apply_style(&ctx, dark);
                    let mut album = albums().remove(0);
                    let mut year = years().remove(0);
                    album.selected = selected;
                    year.selected = selected;
                    let height = if kind == 0 {
                        282.0
                    } else if kind == 1 {
                        LIST_HEIGHT
                    } else {
                        YEAR_HEIGHT
                    };
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(20.0, 20.0),
                        egui::vec2(CARD_WIDTH, height),
                    );
                    let pointer = if kind == 0 {
                        egui::pos2(35.0, 215.0)
                    } else {
                        egui::pos2(35.0, 35.0)
                    };
                    for hovered in [false, true] {
                        for _ in 0..3 {
                            let input = egui::RawInput {
                                screen_rect: Some(egui::Rect::from_min_size(
                                    egui::Pos2::ZERO,
                                    egui::vec2(400.0, 400.0),
                                )),
                                events: if hovered {
                                    vec![egui::Event::PointerMoved(pointer)]
                                } else {
                                    vec![]
                                },
                                ..Default::default()
                            };
                            let output = ctx.run(input, |ctx| {
                                egui::CentralPanel::default().show(ctx, |ui| {
                                    if kind == 2 {
                                        draw_year(ui, rect, &mut year, false);
                                    } else {
                                        draw_album(
                                            ui,
                                            rect,
                                            &mut album,
                                            if kind == 0 {
                                                AlbumView::Covers
                                            } else {
                                                AlbumView::List
                                            },
                                            false,
                                        );
                                    }
                                });
                            });
                            let mut found = 0;
                            for shape in output.shapes {
                                inspect(&shape.shape, shape.clip_rect, &mut found);
                            }
                            assert!(
                                found > 0,
                                "No checkbox border: dark={dark}, selected={selected}, kind={kind}, hovered={hovered}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn year_card_and_checkbox_toggle_once_and_preserve_filtered_selection() {
        let ctx = egui::Context::default();
        let mut years = years();
        let mut frame = |filter: &str, events| {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show_years(ui, &mut years, filter, "year_click_test", false);
                });
            });
            years[0].selected
        };
        frame("", vec![]);
        frame("", vec![]);
        for (pos, selected) in [
            (egui::pos2(90.0, 70.0), true),
            (egui::pos2(25.0, 25.0), false),
        ] {
            frame(
                "",
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            assert_eq!(
                frame(
                    "",
                    vec![egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    }]
                ),
                selected
            );
            assert_eq!(frame("no matching year", vec![]), selected);
            assert_eq!(frame("1990", vec![]), selected);
        }
    }
}
