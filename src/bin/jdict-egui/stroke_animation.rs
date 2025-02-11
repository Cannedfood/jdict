use std::sync::LazyLock;
use std::time::Instant;

use egui::Vec2;
use jdict2::kanjivg::{self, Coord, StrokeGroup};

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

pub(crate) fn kanji_stroke_animation(ui: &mut egui::Ui, size: f32, kanji: &StrokeGroup) {
    let (rect, res) = ui.allocate_exact_size((size, size).into(), egui::Sense::hover());

    let style = ui.style();
    let interacted_style = style.interact(&res);

    let mut f = if res.hovered() {
        ui.ctx().request_repaint();
        START_TIME.elapsed().as_secs_f32() % measure(kanji)
    }
    else {
        0.0
    };

    ui.painter().rect_filled(
        rect,
        interacted_style.corner_radius,
        style.visuals.extreme_bg_color,
    );

    draw_recursive(
        &ui.painter_at(rect.shrink(3.0)),
        kanji,
        if f > 0.0 {
            interacted_style.bg_stroke
        }
        else {
            interacted_style.fg_stroke
        },
        #[allow(const_item_mutation)]
        &mut f32::INFINITY,
    );

    if f > 0.0 {
        draw_recursive(
            &ui.painter_at(rect.shrink(3.0)),
            kanji,
            interacted_style.fg_stroke,
            &mut f,
        );
    }

    fn measure(kanji: &StrokeGroup) -> f32 {
        kanji
            .subgroups
            .iter()
            .map(|child| match child {
                kanjivg::Child::Group(group) => measure(group),
                kanjivg::Child::Stroke(stroke) => stroke.path.length(),
            })
            .sum()
    }

    fn draw_recursive(
        painter: &egui::Painter,
        path: &kanjivg::StrokeGroup,
        brush: egui::Stroke,
        length_budget: &mut f32,
    ) {
        for child in &path.subgroups {
            match child {
                kanjivg::Child::Stroke(stroke) => {
                    draw_path(painter, &stroke.path, brush, length_budget);
                }
                kanjivg::Child::Group(group) => {
                    draw_recursive(painter, group, brush, length_budget);
                }
            }
        }
    }
    fn draw_path(
        painter: &egui::Painter,
        path: &kanjivg::Path,
        brush: egui::Stroke,
        length_budget: &mut f32,
    ) {
        let mut brush_position = Vec2::new(0.0, 0.0);
        for cmd in &path.0 {
            match cmd {
                kanjivg::Command::MoveTo(Coord { x, y }) => {
                    brush_position = Vec2::new(*x, *y);
                }
                kanjivg::Command::LineTo(Coord { x, y }) => {
                    // painter.line_segment(
                    //     [
                    //         offset + brush_position * scale,
                    //         offset + Vec2::new(*x, *y) * scale,
                    //     ],
                    //     brush,
                    // );
                    // brush_position = Vec2::new(*x, *y);
                    painter.add(take_line_segment(
                        &painter.clip_rect(),
                        brush_position,
                        Vec2::new(*x, *y),
                        brush,
                        length_budget,
                    ));
                }
                kanjivg::Command::CubicBezier(c1, c2, to) => {
                    let c1 = Vec2::new(c1.x, c1.y);
                    let c2 = Vec2::new(c2.x, c2.y);
                    let to = Vec2::new(to.x, to.y);
                    for (a, b) in [(brush_position, c1), (c1, c2), (c2, to)] {
                        painter.add(take_line_segment(
                            &painter.clip_rect(),
                            a,
                            b,
                            brush,
                            length_budget,
                        ));
                    }
                    brush_position = to;
                }
                _ => eprintln!("Unimplented stroke type: {cmd:?}"),
            }
        }

        fn take_line_segment(
            rect: &egui::Rect,
            from: Vec2,
            to: Vec2,
            stroke: egui::Stroke,
            length_budget: &mut f32,
        ) -> egui::Shape {
            let length = (to - from).length();
            if length > *length_budget {
                *length_budget = 0.0;
                return egui::Shape::Noop;
            }

            let length = length.min(*length_budget);
            *length_budget -= length;

            let to = from + (to - from).normalized() * length;

            let from = rect.min + from * rect.size();
            let to = rect.min + to * rect.size();

            egui::Shape::line_segment([from, to], stroke)
        }
    }
}
