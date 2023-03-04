use std::sync::mpsc;

use egui::ScrollArea;
use jdict_shared::{shared_api::{SearchResult, self, DB_LOADING}, kanjidic::ReadingType, database::DictData};
use itertools::Itertools;

struct JDictApp {
	search: String,
	history: Vec<String>,
	should_refresh: bool,
	results: Option<SearchResult>,
	inbox: mpsc::Receiver<SearchResult>,
	send_results: mpsc::Sender<SearchResult>,
}
impl JDictApp {
	fn new() -> Self {
		let (tx, rx) = mpsc::channel();
		Self {
			search: String::new(),
			history: Vec::new(),
			should_refresh: false,
			results: None,
			inbox: rx,
			send_results: tx,
		}
	}

	fn refresh_search(&mut self, ctx: egui::Context) {
		self.results = None;

		if self.search.is_empty() {
			return;
		}

		if self.history.last() != Some(&self.search) {
			self.history.push(self.search.clone());
		}

		let query_copy = self.search.clone();
		let sender = self.send_results.clone();
		std::thread::spawn(move || {
			sender.send(shared_api::search(&query_copy, None, None)).unwrap();
			ctx.request_repaint();
		});
	}
}

impl JDictApp {
	fn draw_searchbar(&mut self, ui: &mut egui::Ui) {
		ui.horizontal_top(|ui| {
			let re = ui.add(
				egui::TextEdit::singleline(&mut self.search)
				.hint_text("Search")
			);

			let sent = re.lost_focus() && re.ctx.input(|input| input.key_pressed(egui::Key::Enter));
			self.should_refresh |= sent;

			// if sent {
				ui.memory_mut(|mem| mem.request_focus(re.id));
			// }

			if let Some(results) = &self.results {
				ui.label(format!("{} results in {}", results.results_total, results.time));
			}

			if DB_LOADING.load(std::sync::atomic::Ordering::Relaxed) {
				ui.spinner();
			}
		});
	}

	fn draw_results(&mut self, ui: &mut egui::Ui) {
		if let Some(results) = &self.results {
			ScrollArea::vertical()
			.min_scrolled_width(ui.available_width())
			.show(ui, |ui| {
				for entry in &results.results {
					ui.small(entry.readings.iter().map(|r| &r.value).join(", "));
					ui.horizontal(|ui| {
						for kanji in &entry.kanji {
							if ui.small_button(&kanji.value).clicked() {
								self.search = kanji.value.clone();
								self.should_refresh = true;
							}
						}
					});
					for sense in &entry.senses {
						ui.label(format!(" - {}", sense.glosses.iter().map(|g| &g.value).join(", ")));
					}
					ui.separator();
				}
			});
		}
	}
}

impl eframe::App for JDictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(3.0);

		if let Ok(results) = self.inbox.try_recv() {
			self.results = Some(results);
		}

		ctx.input_mut(|input| {
			if input.pointer.button_pressed(egui::PointerButton::Middle) {
				self.history.pop();
				self.search = match self.history.last() {
					Some(s) => s.clone(),
					None => String::new(),
				};
				self.should_refresh = true;
			}
		});

		egui::TopBottomPanel::top("search_bar").show(ctx, |ui| self.draw_searchbar(ui));

		if let Some(result) = &self.results {
			if !result.kanji.is_empty() {
				egui::SidePanel::right("kanji_panel").show(ctx, |ui| {
					ui.heading("Kanji");
					if let Some(result) = &self.results {
						for kanji in &result.kanji {
							ui.separator();
							// let kanjivg = result.kanjivg.iter().find(|vg| vg.kanji == kanji.literal);

							if ui.button(&kanji.literal).clicked() {
								self.search = kanji.literal.clone();
								self.should_refresh = true;
							}
							for rm in &kanji.reading_meaning_groups {
								let kun = rm.readings.iter().filter(|r| r.typ == ReadingType::ja_kun).map(|r| &r.value).join(", ");
								let on  = rm.readings.iter().filter(|r| r.typ == ReadingType::ja_on).map(|r| &r.value).join(", ");

								ui.label(
									rm.meanings.iter().filter(|m| m.lang == "en").map(|m| &m.value).join(", ")
								);

								if !kun.is_empty() {
									ui.small(format!("kun: {}", kun));
								}
								if !on.is_empty() {
									ui.small(format!("on: {}", on));
								}
							}
						}
					}
				});
			}
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.results.is_some() {
				self.draw_results(ui);
			}
		});

		if self.should_refresh {
			self.should_refresh = false;
			self.refresh_search(ctx.clone());
		}
    }
}

fn main() {
	shared_api::parse_db_async(DictData::<'static> {
		dict:     include_bytes!("../../../res/JMdict_e.gz"),
		kanjidic: include_bytes!("../../../res/kanjidic2.xml.gz"),
		kanjivg:  include_bytes!("../../../res/kanjivg.xml.gz"),
	});

	eframe::run_native(
		"jdict-egui",
		eframe::NativeOptions {
			..Default::default()
		},
		Box::new(|ctx| {
			let mut fonts = egui::FontDefinitions::default();
			fonts.font_data.insert(
				"JP".into(),
				egui::FontData::from_static(
					include_bytes!("../../../res/NotoSansCJKjp-Regular.otf")
				)
			);

			fonts.families.entry(egui::FontFamily::Proportional).or_default().push("JP".into());
			fonts.families.entry(egui::FontFamily::Monospace).or_default().push("JP".into());

			ctx.egui_ctx.set_fonts(fonts);

			Box::new(JDictApp::new())
		})
	).unwrap();
}
