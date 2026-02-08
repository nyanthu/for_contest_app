use eframe::egui::*;
use egui_extras::{Column, TableBuilder};

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
}

impl Default for DataTool {
    fn default() -> Self {
        Self {
            progress: 0.0,
            is_checked: false,
            input: "".to_string(),
            already_approved: false,
            managername: "".to_string(),
            passcode: "1234".to_string(),
            user_admin: false,
            name: "".to_string(),
            id: "".to_string(),
            description: "".to_string(),

            manage_items: (0..10)
                .map(|i| Item {
                    added_user: "A".to_string(),
                    name: format!("Item {i}"),
                    id: i.to_string(),
                    value: (i * 10).to_string(),
                })
                .collect(),
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
        ctx.set_visuals(egui::Visuals::light());

        CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("Data Aggregation Tool").size(24.0));

            if !self.user_admin {
                ui.label("Manager name:");
                ui.text_edit_singleline(&mut self.managername);

                ui.label("Pass code:");
                ui.text_edit_singleline(&mut self.passcode);



                if !self.managername.trim().is_empty() {
                    ui.label(
                        "→This application is intended for legal purposes only.\nAll actions performed are entirely your responsibility.\n",
                    );
                    ui.label("→Do you agree to proceed?");
                    ui.add_enabled(
                        !self.already_approved,
                        egui::Checkbox::new(&mut self.is_checked, "Yes, I agree."),
                    );


                    if self.is_checked == true && ui
                        .button(egui::RichText::new("Continue").size(15.0))
                        .clicked()
                    {
                        if self.passcode == "2345"{
                            self.user_admin = true;
                            self.already_approved = true;
                        }
                    }
                }
            } else {
                ui.separator();
                ui.group(|ui| {
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

                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);

                ui.label("ID:");
                ui.text_edit_singleline(&mut self.id);

                ui.label("Description");
                ui.text_edit_multiline(&mut self.description);

                if ui
                    .button(egui::RichText::new("Continue").size(15.0))
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
