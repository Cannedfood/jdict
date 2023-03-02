use std::sync::mpsc;

use egui::ScrollArea;
use jdict_shared::{shared_api::{SearchResult, self}, database::Config};
use itertools::Itertools;

struct JDictApp {
	search: String,
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
			should_refresh: false,
			results: None,
			inbox: rx,
			send_results: tx,
		}
	}

	fn refresh_search(&mut self, ctx: egui::Context) {
		println!("search: {}", self.search);
		if self.search.is_empty() {
			self.results = None;
			return;
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

			if sent {
				ui.memory_mut(|mem| mem.request_focus(re.id));
				self.should_refresh = true;
			}

			if let Some(results) = &self.results {
				ui.label(format!("{} results in {}", results.results_total, results.time));
			}
		});
	}

	fn draw_results(&mut self, ui: &mut egui::Ui) {
		if let Some(results) = &self.results {
			ScrollArea::vertical()
			.min_scrolled_width(ui.available_width())
			.show(ui, |ui| {
				for entry in &results.results {
					ui.separator();
					ui.horizontal(|ui| {
						for reading in &entry.readings {
							ui.small(format!("{}, ", reading.value));
						}
					});
					ui.horizontal(|ui| {
						for kanji in &entry.kanji {
							if ui.small_button(&kanji.value).clicked() {
								self.search = kanji.value.clone();
								self.should_refresh = true;
							}
						}
					});
					for sense in &entry.senses {
						ui.label(sense.glosses.iter().map(|p| &p.value).join(", "));
					}
				}
			});
		}
	}
}

impl eframe::App for JDictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if let Ok(results) = self.inbox.try_recv() {
			self.results = Some(results);
		}

        egui::CentralPanel::default().show(ctx, |ui| {
			self.draw_searchbar(ui);
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
	shared_api::load_db_async(Config {
		jmdict_file:   "res/JMdict_e.gz".into(),
		kanjidic_file: "res/kanjidic2.xml.gz".into(),
		kanjivg_file:  "res/kanjivg.xml.gz".into(),
	});

	eframe::run_native(
		"jdict-egui",
		eframe::NativeOptions {
			..Default::default()
		},
		Box::new(|_ctxt| Box::new(JDictApp::new()))
	).unwrap();
}
