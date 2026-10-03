use eframe::egui;
use egui_graph::{edge::Edge, Graph, node::Node, NodeId, View};

struct MyApp {
    view: View,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            view: View::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Простейший пример egui_graph (v0.16.4)");
            ui.separator();

            Graph::new("my_main_graph")
                .background(true)
                .dot_grid(true)
                .show(&mut self.view, ui, |ui, show| {
                    
                    // ИСПРАВЛЕНИЕ 1: Используем цепочку вызовов (method chaining)
                    // .nodes() возвращает show, к которому мы сразу применяем .edges()
                    show.nodes(ui, |nctx, ui| {
                        Node::new("node_1")
                            .inputs(1)
                            .outputs(1)
                            .show(nctx, ui, |node_ctx| {
                                // ИСПРАВЛЕНИЕ 2: Второй аргумент - это SocketLayout, а не nctx.
                                // Префикс "_" убирает предупреждение компилятора.
                                node_ctx.framed(|ui, _socket_layout| {
                                    ui.label("Привет, Узел 1!");
                                })
                            });

                        Node::new("node_2")
                            .inputs(1)
                            .outputs(1)
                            .show(nctx, ui, |node_ctx| {
                                node_ctx.framed(|ui, _socket_layout| {
                                    ui.label("Привет");
                                })
                            });
                    })
                    .edges(ui, |ectx, ui| {
                        let mut is_selected = false;
                        
                        Edge::new(
                            (NodeId::new("node_1"), 0),
                            (NodeId::new("node_2"), 0),
                            &mut is_selected,
                        )
                        .show(ectx, ui);
                    }); // <-- Здесь цепочка замыкается
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Simple egui_graph",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}