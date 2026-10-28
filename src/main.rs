#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const FOCUS_DURATION: Duration = Duration::from_secs(25 * 60);
const BREAK_DURATION: Duration = Duration::from_secs(5 * 60);

const QUOTES: &[&str] = &[
    "Quaque! Bora focar! 🚀",
    "Beba água agora! 💧",
    "Endireite essa coluna! 🪑",
    "Você está mandando bem! ⭐",
    "Quaque quack! 🥰",
    "Bongo Duck acredita em você! 🦆",
];

// Fallbacks de segurança
const FALLBACK_IDLE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M 68,145 L 68,85 A 32,32 0 0,1 132,85 L 132,145 Z" fill="#FFFFFF" stroke="#000000" stroke-width="6.5"/><ellipse cx="100" cy="100" rx="44" ry="15" fill="#FF8A19" stroke="#000000" stroke-width="6"/><circle cx="83" cy="78" r="13" fill="#000000"/><circle cx="116" cy="81" r="12" fill="#000000"/><rect x="25" y="125" width="150" height="52" rx="14" fill="#A86E58" stroke="#000000" stroke-width="6.5" transform="rotate(14 100 150)"/><circle cx="70" cy="138" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/><circle cx="140" cy="155" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/></svg>"##;
const FALLBACK_TYPE1: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M 68,145 L 68,85 A 32,32 0 0,1 132,85 L 132,145 Z" fill="#FFFFFF" stroke="#000000" stroke-width="6.5"/><ellipse cx="100" cy="100" rx="44" ry="15" fill="#FF8A19" stroke="#000000" stroke-width="6"/><circle cx="83" cy="78" r="13" fill="#000000"/><circle cx="116" cy="81" r="12" fill="#000000"/><rect x="25" y="125" width="150" height="52" rx="14" fill="#A86E58" stroke="#000000" stroke-width="6.5" transform="rotate(14 100 150)"/><circle cx="56" cy="116" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/><circle cx="140" cy="155" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/></svg>"##;
const FALLBACK_TYPE2: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M 68,145 L 68,85 A 32,32 0 0,1 132,85 L 132,145 Z" fill="#FFFFFF" stroke="#000000" stroke-width="6.5"/><ellipse cx="100" cy="100" rx="44" ry="15" fill="#FF8A19" stroke="#000000" stroke-width="6"/><circle cx="83" cy="78" r="13" fill="#000000"/><circle cx="116" cy="81" r="12" fill="#000000"/><rect x="25" y="125" width="150" height="52" rx="14" fill="#A86E58" stroke="#000000" stroke-width="6.5" transform="rotate(14 100 150)"/><circle cx="70" cy="138" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/><circle cx="142" cy="136" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/></svg>"##;
const FALLBACK_PET: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M 68,145 L 68,85 A 32,32 0 0,1 132,85 L 132,145 Z" fill="#FFFFFF" stroke="#000000" stroke-width="6.5"/><path d="M 74,80 Q 84,68 94,80" stroke="#000000" stroke-width="5.5" stroke-linecap="round" fill="none"/><path d="M 106,83 Q 116,71 126,83" stroke="#000000" stroke-width="5.5" stroke-linecap="round" fill="none"/><ellipse cx="100" cy="100" rx="44" ry="15" fill="#FF8A19" stroke="#000000" stroke-width="6"/><rect x="25" y="125" width="150" height="52" rx="14" fill="#A86E58" stroke="#000000" stroke-width="6.5" transform="rotate(14 100 150)"/><circle cx="48" cy="118" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/><circle cx="152" cy="132" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/></svg>"##;
const FALLBACK_SLEEP: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M 68,145 L 68,85 A 32,32 0 0,1 132,85 L 132,145 Z" fill="#FFFFFF" stroke="#000000" stroke-width="6.5"/><path d="M 74,77 Q 84,86 94,79" stroke="#000000" stroke-width="5.5" stroke-linecap="round" fill="none"/><path d="M 106,81 Q 116,90 126,83" stroke="#000000" stroke-width="5.5" stroke-linecap="round" fill="none"/><path d="M 97,112 C 94,115 92,121 92,123 A 4.5,4.5 0 0,0 101,123 C 101,121 99,115 97,112 Z" fill="#0099FF" stroke="#000000" stroke-width="3"/><ellipse cx="100" cy="100" rx="44" ry="15" fill="#FF8A19" stroke="#000000" stroke-width="6"/><rect x="25" y="125" width="150" height="52" rx="14" fill="#A86E58" stroke="#000000" stroke-width="6.5" transform="rotate(14 100 150)"/><circle cx="70" cy="138" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/><circle cx="140" cy="155" r="9" fill="#FFFFFF" stroke="#000000" stroke-width="4.5"/></svg>"##;

#[derive(PartialEq, Clone, Copy)]
enum PomodoroMode {
    Focus,
    ShortBreak,
}

struct Heart {
    pos: Pos2,
    alpha: f32,
}

struct DuckApp {
    mode: PomodoroMode,
    is_running: bool,
    time_remaining: Duration,
    last_tick: Instant,

    quote: String,
    quote_timer: f32,
    pet_timer: f32,
    hearts: Vec<Heart>,
    quote_idx: usize,

    duck_scale: f32,
    has_custom_svgs: bool,

    svg_idle: Vec<u8>,
    svg_type1: Vec<u8>,
    svg_type2: Vec<u8>,
    svg_pet: Vec<u8>,
    svg_sleep: Vec<u8>,
}

fn find_file(filename: &str) -> Option<PathBuf> {
    let p = Path::new(filename);
    if p.exists() {
        return Some(p.to_path_buf());
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let next_to_exe = parent.join(filename);
            if next_to_exe.exists() {
                return Some(next_to_exe);
            }
        }
    }
    None
}

// Carrega o logo.png para ser o ícone nativo do aplicativo
fn load_window_icon() -> Option<egui::IconData> {
    let icon_path = find_file("logo.png")?;
    let img = image::open(icon_path).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    println!("🎨 [Bongo Duck] Ícone logo.png carregado com sucesso!");
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}

fn load_svg_file(path: &str, fallback: &'static str) -> Vec<u8> {
    if let Some(resolved_path) = find_file(path) {
        if let Ok(bytes) = std::fs::read(&resolved_path) {
            println!("🦆 [Bongo Duck] SVG carregado com sucesso: {:?}", resolved_path);
            return bytes;
        }
    }
    fallback.as_bytes().to_vec()
}

fn is_tall_canvas(bytes: &[u8]) -> bool {
    if let Ok(s) = std::str::from_utf8(bytes) {
        if let Some(tag_end) = s.find('>') {
            let header = &s[..tag_end];
            if header.contains("450") || header.contains("300") {
                return true;
            }
        }
    }
    true
}

impl Default for DuckApp {
    fn default() -> Self {
        let has_custom = find_file("pato.svg").is_some()
            || find_file("pato1.svg").is_some()
            || find_file("pato4.svg").is_some();

        Self {
            mode: PomodoroMode::Focus,
            is_running: false,
            time_remaining: FOCUS_DURATION,
            last_tick: Instant::now(),
            quote: "Quaque! Bora focar? 💻".to_string(),
            quote_timer: 4.0_f32,
            pet_timer: 0.0_f32,
            hearts: Vec::new(),
            quote_idx: 0,
            duck_scale: 1.0_f32,
            has_custom_svgs: has_custom,

            svg_idle: load_svg_file("pato.svg", FALLBACK_IDLE),
            svg_type1: load_svg_file("pato1.svg", FALLBACK_TYPE1),
            svg_type2: load_svg_file("pato2.svg", FALLBACK_TYPE2),
            svg_pet: load_svg_file("pato3.svg", FALLBACK_PET),
            svg_sleep: load_svg_file("pato4.svg", FALLBACK_SLEEP),
        }
    }
}

impl DuckApp {
    fn notify_kde(&self, title: &str, msg: &str) {
        let _ = Command::new("notify-send")
            .args(["-a", "Bongo Duck", "-u", "normal", title, msg])
            .spawn();

        if let Some(audio_path) = find_file("quack.mp3") {
            let path_str = audio_path.to_string_lossy().to_string();
            if Command::new("pw-play").arg(&path_str).spawn().is_err() {
                if Command::new("paplay").arg(&path_str).spawn().is_err() {
                    let _ = Command::new("gst-play-1.0")
                        .args(["--no-interactive", &path_str])
                        .spawn();
                }
            }
        } else {
            let _ = Command::new("paplay")
                .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
                .spawn();
        }
    }
}

impl eframe::App for DuckApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));

        let now = Instant::now();
        let dt = (now - self.last_tick).as_secs_f32();
        self.last_tick = now;

        if self.is_running {
            if self.time_remaining > Duration::ZERO {
                self.time_remaining = self.time_remaining.saturating_sub(Duration::from_secs_f32(dt));
            } else {
                match self.mode {
                    PomodoroMode::Focus => {
                        self.mode = PomodoroMode::ShortBreak;
                        self.time_remaining = BREAK_DURATION;
                        self.quote = "Hora do descanso! ☕ Zzz...".into();
                        self.quote_timer = 6.0_f32;
                        self.notify_kde("Bongo Duck", "Tempo de foco concluído! Hora de descansar!");
                    }
                    PomodoroMode::ShortBreak => {
                        self.mode = PomodoroMode::Focus;
                        self.time_remaining = FOCUS_DURATION;
                        self.quote = "Bora produzir! 💻 Foco total!".into();
                        self.quote_timer = 6.0_f32;
                        self.notify_kde("Bongo Duck", "Pausa concluída! Bora focar!");
                    }
                }
            }
        }

        if self.quote_timer > 0.0_f32 {
            self.quote_timer -= dt;
        }

        if self.pet_timer > 0.0_f32 {
            self.pet_timer -= dt;
        }

        for h in &mut self.hearts {
            h.pos.y -= 35.0_f32 * dt;
            h.alpha -= 0.8_f32 * dt;
        }
        self.hearts.retain(|h| h.alpha > 0.0_f32);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let ox = rect.min.x;
                let oy = rect.min.y;
                let p = |x: f32, y: f32| Pos2::new(ox + x, oy + y);

                let painter = ui.painter().clone();
                let time = ctx.input(|i| i.time) as f32;

                let pet_area = Rect::from_min_max(p(0.0_f32, 40.0_f32), p(240.0_f32, 220.0_f32));
                let pet_resp = ui.interact(pet_area, egui::Id::new("duck_pet_main"), egui::Sense::click_and_drag());
                
                if pet_resp.drag_started_by(egui::PointerButton::Primary) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                } else if pet_resp.clicked() {
                    self.pet_timer = 2.0_f32;
                    self.hearts.push(Heart { pos: p(120.0_f32, 45.0_f32), alpha: 1.0_f32 });
                    self.quote_idx = (self.quote_idx + 1) % QUOTES.len();
                    self.quote = QUOTES[self.quote_idx].to_string();
                    self.quote_timer = 3.5_f32;
                }

                let scroll = ui.input(|i| i.raw_scroll_delta.y);
                if pet_resp.hovered() && scroll != 0.0_f32 {
                    self.duck_scale = (self.duck_scale + scroll * 0.005_f32).clamp(0.6_f32, 2.5_f32);
                }

                let bubble_rect = Rect::from_min_max(p(12.0_f32, 8.0_f32), p(198.0_f32, 38.0_f32));
                painter.rect_filled(bubble_rect, 15.0_f32, Color32::from_rgba_unmultiplied(20, 22, 28, 230));
                painter.rect_stroke(bubble_rect, 15.0_f32, Stroke::new(1.5_f32, Color32::from_rgb(255, 215, 64)));

                let current_msg = if self.pet_timer > 0.0_f32 {
                    "Quaque! Que carinho bom! 🥰".into()
                } else if self.quote_timer > 0.0_f32 {
                    self.quote.clone()
                } else if self.mode == PomodoroMode::ShortBreak {
                    "☕ Descanso... Zzz...".into()
                } else if self.is_running && self.mode == PomodoroMode::Focus {
                    "🔥 Digitando a todo vapor!".into()
                } else {
                    "Quaque! Me dê carinho!".into()
                };

                painter.text(
                    p(105.0_f32, 23.0_f32),
                    egui::Align2::CENTER_CENTER,
                    current_msg,
                    egui::FontId::proportional(11.5_f32),
                    Color32::WHITE,
                );

                let close_rect = Rect::from_center_size(p(222.0_f32, 23.0_f32), Vec2::splat(18.0_f32));
                let close_resp = ui.allocate_rect(close_rect, egui::Sense::click());
                painter.circle_filled(p(222.0_f32, 23.0_f32), 8.5_f32, Color32::from_rgba_unmultiplied(210, 45, 45, 210));
                painter.text(p(222.0_f32, 23.0_f32), egui::Align2::CENTER_CENTER, "✕", egui::FontId::proportional(10.0_f32), Color32::WHITE);
                if close_resp.clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                let is_typing = self.is_running && self.mode == PomodoroMode::Focus;
                let is_break = self.mode == PomodoroMode::ShortBreak;
                let beat = ((time * 8.0_f32) as i32) % 2;

                let (active_uri, active_bytes) = if self.pet_timer > 0.0_f32 {
                    ("bytes://pato3.svg", &self.svg_pet)
                } else if is_break {
                    ("bytes://pato4.svg", &self.svg_sleep)
                } else if is_typing {
                    if beat == 0 {
                        ("bytes://pato1.svg", &self.svg_type1)
                    } else {
                        ("bytes://pato2.svg", &self.svg_type2)
                    }
                } else {
                    ("bytes://pato.svg", &self.svg_idle)
                };

                let duck_center_window = p(120.0_f32, 130.0_f32);

                if self.has_custom_svgs && is_tall_canvas(active_bytes) {
                    let mult = 7.5_f32 * self.duck_scale;
                    let canvas_w = 160.0_f32 * mult;
                    let canvas_h = 240.0_f32 * mult;

                    let top_left_x = duck_center_window.x - (canvas_w * 0.54_f32);
                    let top_left_y = duck_center_window.y - (canvas_h * 0.53_f32);
                    let canvas_rect = Rect::from_min_size(
                        Pos2::new(top_left_x, top_left_y),
                        Vec2::new(canvas_w, canvas_h),
                    );

                    let svg_img = egui::Image::from_bytes(active_uri, active_bytes.clone())
                        .fit_to_exact_size(Vec2::new(canvas_w, canvas_h));
                    ui.put(canvas_rect, svg_img);
                } else {
                    let duck_size = Vec2::splat(160.0_f32 * self.duck_scale);
                    let dest_rect = Rect::from_center_size(duck_center_window, duck_size);
                    let svg_img = egui::Image::from_bytes(active_uri, active_bytes.clone())
                        .fit_to_exact_size(duck_size);
                    ui.put(dest_rect, svg_img);
                }

                if is_break && self.pet_timer <= 0.0_f32 {
                    let z_cycle = (time * 1.5_f32).fract();
                    let z_x = duck_center_window.x + 55.0_f32 * self.duck_scale + (z_cycle * 8.0_f32);
                    let z_y = duck_center_window.y - 55.0_f32 * self.duck_scale - (z_cycle * 18.0_f32);
                    let z_alpha = ((1.0_f32 - z_cycle) * 235.0_f32) as u8;
                    painter.text(
                        Pos2::new(z_x, z_y),
                        egui::Align2::CENTER_CENTER,
                        "Zzz...",
                        egui::FontId::proportional(15.0_f32),
                        Color32::from_rgba_unmultiplied(100, 190, 255, z_alpha),
                    );
                }

                for h in &self.hearts {
                    let alpha = (h.alpha.clamp(0.0_f32, 1.0_f32) * 255.0_f32) as u8;
                    painter.text(
                        h.pos,
                        egui::Align2::CENTER_CENTER,
                        "❤️",
                        egui::FontId::proportional(15.0_f32),
                        Color32::from_rgba_unmultiplied(255, 50, 80, alpha),
                    );
                }

                let hud_rect = Rect::from_min_max(p(12.0_f32, 222.0_f32), p(228.0_f32, 296.0_f32));
                painter.rect_filled(hud_rect, 12.0_f32, Color32::from_rgba_unmultiplied(20, 22, 28, 235));
                painter.rect_stroke(hud_rect, 12.0_f32, Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 40)));

                let mins = self.time_remaining.as_secs() / 60;
                let secs = self.time_remaining.as_secs() % 60;
                let timer_text = format!("{:02}:{:02}", mins, secs);

                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(hud_rect), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0_f32);
                        let timer_color = if self.mode == PomodoroMode::Focus {
                            Color32::from_rgb(255, 215, 64)
                        } else {
                            Color32::from_rgb(100, 200, 255)
                        };
                        ui.heading(egui::RichText::new(timer_text).size(22.0_f32).strong().color(timer_color));

                        ui.add_space(2.0_f32);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(5.0_f32, 0.0_f32);
                            ui.add_space(14.0_f32);

                            let play_icon = if self.is_running { "⏸ Pausa" } else { "▶ Iniciar" };
                            if ui.button(play_icon).clicked() {
                                self.is_running = !self.is_running;
                            }

                            let mode_icon = if self.mode == PomodoroMode::Focus { "☕ Pausa" } else { "🍅 Foco" };
                            if ui.button(mode_icon).clicked() {
                                self.is_running = false;
                                if self.mode == PomodoroMode::Focus {
                                    self.mode = PomodoroMode::ShortBreak;
                                    self.time_remaining = BREAK_DURATION;
                                } else {
                                    self.mode = PomodoroMode::Focus;
                                    self.time_remaining = FOCUS_DURATION;
                                }
                            }

                            if ui.button("🔄").clicked() {
                                self.is_running = false;
                                self.time_remaining = if self.mode == PomodoroMode::Focus { FOCUS_DURATION } else { BREAK_DURATION };
                            }
                        });

                        ui.add_space(2.0_f32);
                        ui.label(
                            egui::RichText::new("Arraste para mover • Scroll: tamanho")
                                .size(8.5_f32)
                                .color(Color32::from_gray(140)),
                        );
                    });
                });
            });
    }
}

fn main() -> eframe::Result<()> {
    if std::env::var("WINIT_UNIX_BACKEND").is_err() {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Bongo Duck")
        .with_app_id("bongo_duck")
        .with_inner_size([240.0_f32, 305.0_f32])
        .with_transparent(true)
        .with_decorations(false)
        .with_always_on_top()
        .with_window_level(egui::WindowLevel::AlwaysOnTop)
        .with_resizable(false);

    // Carrega o logo.png como ícone da janela
    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Bongo Duck",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(DuckApp::default()))
        }),
    )
}