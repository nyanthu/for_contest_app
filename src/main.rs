use eframe::egui::*;
use egui_extras::{Column, TableBuilder};
use std::net::UdpSocket;

#[derive(Debug, Clone, PartialEq)]
struct Item {
    added_user: String,
    name: String,
    id: String,
    value: String,
}

#[derive(PartialEq, Debug)]
struct DataTool {
    progress: f32,
    is_checked: bool,
    input: String,
    already_approved: bool,
    managername: String,
    passcode: String,
    // User is OK
    user_admin: bool,
    // manage
    name: String,
    id: String,
    description: String,
    //manage data
    manage_items: Vec<Item>,
    //info alert
    show_alert: bool,
}

impl Default for DataTool {
    fn default() -> Self {
        Self {
            progress: 0.0,
            is_checked: false,
            input: "".to_string(),
            already_approved: false,
            managername: "".to_string(),
            passcode: "".to_string(),
            user_admin: false,
            name: "".to_string(),
            id: "".to_string(),
            description: "".to_string(),

            manage_items: (0..10)
                .map(|i| Item {
                    added_user: "Test".to_string(),
                    name: format!("Item {i}"),
                    id: i.to_string(),
                    value: (i * 10).to_string(),
                })
                .collect(),
            show_alert: false,
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Data Aggregation Tool",
        options,
        Box::new(|_cc| Ok(Box::new(DataTool::default()))),
    )
}

//
// User Interface
//
impl eframe::App for DataTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //ctx.set_visuals(egui::Visuals::light());
        ctx.set_pixels_per_point(1.2);

        if self.show_alert {
            egui::Window::new("Info")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("password wasn't match account.\nMake new account?");
                    ui.separator();

                    if ui.button("Make New Account").clicked() {
                        self.show_alert = false;
                    }
                    if ui.button("Retry").clicked() {
                        self.show_alert = false;
                    }
                });
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("Data Aggregation Tool").size(24.0));
            ui.separator();

            if !self.user_admin {

                ui.label("It nice tool");

                ui.label("Manager name:");
                ui.text_edit_singleline(&mut self.managername);

                ui.label("Pass code:");
                ui.text_edit_singleline(&mut self.passcode);



                if !self.managername.trim().is_empty() && !self.passcode.trim().is_empty() {
                    ui.label(
                        "→This application is intended for legal purposes only.\nAll actions performed are entirely your responsibility.\n",
                    );
                    ui.label("→Do you agree to proceed?");
                    ui.add_enabled(
                        !self.already_approved,
                        egui::Checkbox::new(&mut self.is_checked, "Yes, I agree."),
                    );


                    if ui
                        .add_enabled(
                            self.is_checked,
                            egui::Button::new("Continue"),
                        )
                        .clicked()
                    {
                        if self.passcode == "2345"{
                            self.user_admin = true;
                            self.already_approved = true;
                        } else {
                            self.show_alert = true;

                        }
                    }
                }
            } else {

                let ip = UdpSocket::bind("0.0.0.0:0")
                    .and_then(|s| {
                        s.connect("8.8.8.8:80")?;
                        s.local_addr()
                    })
                    .map(|a| a.ip().to_string())
                    .unwrap_or("unknown".into());

                ui.label(format!(
                    "Share databases from sites only within the same Wi-Fi network.\nhttp://{}:14143/{}/",
                    ip, self.managername.trim()
                ));
                ui.label("Alert: url isn't https");

                ui.group(|ui| {
                    // グループの中身の高さを固定
                    egui::ScrollArea::vertical()
                        .max_height(200.0) // ← ここが「維持したい高さ」
                        .show(ui, |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(Layout::left_to_right(Align::Center))
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::remainder())
                                .header(20.0, |mut header| {
                                    header.col(|ui| { ui.label("By"); });
                                    header.col(|ui| { ui.label("Name"); });
                                    header.col(|ui| { ui.label("ID"); });
                                    header.col(|ui| { ui.label("Description"); });
                                })
                                .body(|mut body| {
                                    for item in &self.manage_items {
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| { ui.label(&item.added_user); });
                                            row.col(|ui| { ui.label(&item.name); });
                                            row.col(|ui| { ui.label(item.id.to_string()); });
                                            row.col(|ui| { ui.label(item.value.to_string()); });
                                        });
                                    }
                                });
                        });
                });


                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);

                ui.label("ID:");
                ui.text_edit_singleline(&mut self.id);

                ui.label("Description");
                ui.text_edit_multiline(&mut self.description);

                if ui
                    .button(egui::RichText::new("AddItem").size(15.0))
                    .clicked()
                {
                    self.manage_items.push(Item {
                        added_user: self.managername.to_string(),
                        name: self.name.to_string(),
                        id: self.id.to_string(),
                        value: self.description.to_string(),
                    });

                    self.name.clear();
                    self.id.clear();
                    self.description.clear();
                }
            }
        });
    }
}
