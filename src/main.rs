
use chrono::Local;
use eframe::egui;
use rusqlite::{params, Connection};
use calamine::{open_workbook_auto, Data, Reader};
use rfd::FileDialog;
const DB_PATH: &str = "components.db";

// ============================================================
// COMPONENT
// ============================================================

#[derive(Clone, Default)]
struct Component {
    id: i32,
    bps_part_number: String,
    part_value: String,
    package_material: String,
    mpn: String,
    sap_description: String,
    part_description: String,
    detailed_description: String,
    manufacturer: String,
    operating_temp: String,
    datasheet: String,
    added_on: String,
}

// ============================================================
// DATABASE
// ============================================================

struct Database {
    conn: Connection,
}

impl Database {
    fn new(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS components (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                bps_part_number TEXT UNIQUE NOT NULL,
                part_value TEXT,
                package_material TEXT,
                mpn TEXT,
                sap_description TEXT,
                part_description TEXT,
                detailed_description TEXT,
                manufacturer TEXT,
                operating_temp TEXT,
                datasheet TEXT,
                added_on TEXT
            )
            ",
            [],
        )?;

        Ok(Self { conn })
    }

    // ========================================================
    // LOAD ALL
    // ========================================================

    fn load_all(&self) -> rusqlite::Result<Vec<Component>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT
                id,
                bps_part_number,
                part_value,
                package_material,
                mpn,
                sap_description,
                part_description,
                detailed_description,
                manufacturer,
                operating_temp,
                datasheet,
                added_on
            FROM components
            ORDER BY id
            ",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Component {
                id: row.get(0)?,
                bps_part_number: row.get(1)?,
                part_value: row.get(2)?,
                package_material: row.get(3)?,
                mpn: row.get(4)?,
                sap_description: row.get(5)?,
                part_description: row.get(6)?,
                detailed_description: row.get(7)?,
                manufacturer: row.get(8)?,
                operating_temp: row.get(9)?,
                datasheet: row.get(10)?,
                added_on: row.get(11)?,
            })
        })?;

        rows.collect()
    }

    // ========================================================
    // SEARCH
    // ========================================================

    fn search(&self, keyword: &str) -> rusqlite::Result<Vec<Component>> {
        let pattern = format!("%{}%", keyword);

        let mut stmt = self.conn.prepare(
            "
            SELECT
                id,
                bps_part_number,
                part_value,
                package_material,
                mpn,
                sap_description,
                part_description,
                detailed_description,
                manufacturer,
                operating_temp,
                datasheet,
                added_on
            FROM components
            WHERE
                bps_part_number LIKE ?1
                OR part_value LIKE ?1
                OR package_material LIKE ?1
                OR mpn LIKE ?1
                OR sap_description LIKE ?1
                OR part_description LIKE ?1
                OR detailed_description LIKE ?1
                OR manufacturer LIKE ?1
                OR operating_temp LIKE ?1
                OR datasheet LIKE ?1
            ORDER BY id
            ",
        )?;

        let rows = stmt.query_map(params![pattern], |row| {
            Ok(Component {
                id: row.get(0)?,
                bps_part_number: row.get(1)?,
                part_value: row.get(2)?,
                package_material: row.get(3)?,
                mpn: row.get(4)?,
                sap_description: row.get(5)?,
                part_description: row.get(6)?,
                detailed_description: row.get(7)?,
                manufacturer: row.get(8)?,
                operating_temp: row.get(9)?,
                datasheet: row.get(10)?,
                added_on: row.get(11)?,
            })
        })?;

        rows.collect()
    }

    // ========================================================
    // ADD
    // ========================================================

    fn add(&self, component: &Component) -> rusqlite::Result<()> {
        self.conn.execute(
            "
            INSERT INTO components (
                bps_part_number,
                part_value,
                package_material,
                mpn,
                sap_description,
                part_description,
                detailed_description,
                manufacturer,
                operating_temp,
                datasheet,
                added_on
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ",
            params![
                component.bps_part_number,
                component.part_value,
                component.package_material,
                component.mpn,
                component.sap_description,
                component.part_description,
                component.detailed_description,
                component.manufacturer,
                component.operating_temp,
                component.datasheet,
                component.added_on,
            ],
        )?;

        Ok(())
    }

    // ========================================================
    // UPDATE
    // ========================================================

    fn update(&self, component: &Component) -> rusqlite::Result<()> {
        self.conn.execute(
            "
            UPDATE components
            SET
                bps_part_number = ?1,
                part_value = ?2,
                package_material = ?3,
                mpn = ?4,
                sap_description = ?5,
                part_description = ?6,
                detailed_description = ?7,
                manufacturer = ?8,
                operating_temp = ?9,
                datasheet = ?10
            WHERE id = ?11
            ",
            params![
                component.bps_part_number,
                component.part_value,
                component.package_material,
                component.mpn,
                component.sap_description,
                component.part_description,
                component.detailed_description,
                component.manufacturer,
                component.operating_temp,
                component.datasheet,
                component.id,
            ],
        )?;

        Ok(())
    }

    // ========================================================
    // DELETE
    // ========================================================

    fn delete(&self, id: i32) -> rusqlite::Result<()> {
        self.conn.execute(
            "DELETE FROM components WHERE id = ?1",
            params![id],
        )?;

        Ok(())
    }
}



// ============================================================
// GUI APPLICATION
// ============================================================

struct ComponentApp {
    db: Database,

    // Form fields
    bps_part_number: String,
    part_value: String,
    package_material: String,
    mpn: String,
    sap_description: String,
    part_description: String,
    detailed_description: String,
    manufacturer: String,
    operating_temp: String,
    datasheet: String,

    // Table
    components: Vec<Component>,
    selected_id: Option<i32>,

    // Search
    search_text: String,

    // Messages
    message: String,
}

impl ComponentApp {
    fn new() -> Self {
        let db = Database::new(DB_PATH)
            .expect("Failed to open database");

        let components = db
            .load_all()
            .expect("Failed to load components");

        Self {
            db,

            bps_part_number: String::new(),
            part_value: String::new(),
            package_material: String::new(),
            mpn: String::new(),
            sap_description: String::new(),
            part_description: String::new(),
            detailed_description: String::new(),
            manufacturer: String::new(),
            operating_temp: String::new(),
            datasheet: String::new(),

            components,
            selected_id: None,

            search_text: String::new(),

            message: String::new(),
        }
    }

    // ========================================================
    // CLEAR FORM
    // ========================================================

    fn clear_fields(&mut self) {
        self.bps_part_number.clear();
        self.part_value.clear();
        self.package_material.clear();
        self.mpn.clear();
        self.sap_description.clear();
        self.part_description.clear();
        self.detailed_description.clear();
        self.manufacturer.clear();
        self.operating_temp.clear();
        self.datasheet.clear();

        self.selected_id = None;
    }

    // ========================================================
    // CREATE COMPONENT FROM FORM
    // ========================================================

    fn form_component(&self) -> Component {
        Component {
            id: self.selected_id.unwrap_or(0),

            bps_part_number: self.bps_part_number.trim().to_string(),
            part_value: self.part_value.clone(),
            package_material: self.package_material.clone(),
            mpn: self.mpn.clone(),
            sap_description: self.sap_description.clone(),
            part_description: self.part_description.clone(),
            detailed_description: self.detailed_description.clone(),
            manufacturer: self.manufacturer.clone(),
            operating_temp: self.operating_temp.clone(),
            datasheet: self.datasheet.clone(),

            added_on: Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }

    // ========================================================
    // ADD
    // ========================================================

    fn add_record(&mut self) {
        if self.bps_part_number.trim().is_empty() {
            self.message =
                "BPS Part Number is required".to_string();
            return;
        }

        let component = self.form_component();

        match self.db.add(&component) {
            Ok(_) => {
                self.message =
                    "New component added".to_string();

                self.components =
                    self.db.load_all().unwrap_or_default();

                self.clear_fields();
            }

            Err(error) => {
                self.message =
                    format!("Insert error: {}", error);
            }
        }
    }

    // ========================================================
    // UPDATE
    // ========================================================

    fn update_record(&mut self) {
        let Some(id) = self.selected_id else {
            self.message =
                "Select a record first".to_string();
            return;
        };

        let mut component = self.form_component();

        component.id = id;

        match self.db.update(&component) {
            Ok(_) => {
                self.message =
                    "Record updated successfully".to_string();

                self.components =
                    self.db.load_all().unwrap_or_default();

                self.clear_fields();
            }

            Err(error) => {
                self.message =
                    format!("Update error: {}", error);
            }
        }
    }

    // ========================================================
    // DELETE
    // ========================================================

    fn delete_record(&mut self) {
        let Some(id) = self.selected_id else {
            self.message =
                "Select a record first".to_string();
            return;
        };

        match self.db.delete(id) {
            Ok(_) => {
                self.message =
                    "Record deleted".to_string();

                self.components =
                    self.db.load_all().unwrap_or_default();

                self.clear_fields();
            }

            Err(error) => {
                self.message =
                    format!("Delete error: {}", error);
            }
        }
    }

    // ========================================================
    // SEARCH
    // ========================================================

    fn search(&mut self) {
        let keyword = self.search_text.trim();

        if keyword.is_empty() {
            self.components =
                self.db.load_all().unwrap_or_default();
        } else {
            self.components =
                self.db.search(keyword).unwrap_or_default();
        }
    }

    // ========================================================
    // SELECT RECORD
    // ========================================================

    fn select_component(&mut self, component: &Component) {
        self.selected_id = Some(component.id);

        self.bps_part_number =
            component.bps_part_number.clone();

        self.part_value =
            component.part_value.clone();

        self.package_material =
            component.package_material.clone();

        self.mpn =
            component.mpn.clone();

        self.sap_description =
            component.sap_description.clone();

        self.part_description =
            component.part_description.clone();

        self.detailed_description =
            component.detailed_description.clone();

        self.manufacturer =
            component.manufacturer.clone();

        self.operating_temp =
            component.operating_temp.clone();

        self.datasheet =
            component.datasheet.clone();
    }

    // ========================================================
    // FORM
    // ========================================================

    fn draw_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("Component Details");

        ui.separator();

        ui.label("BPS Part Number");
        ui.text_edit_singleline(&mut self.bps_part_number);

        ui.label("Value");
        ui.text_edit_singleline(&mut self.part_value);

        ui.label("Package/Material");
        ui.text_edit_singleline(&mut self.package_material);

        ui.label("MPN");
        ui.text_edit_singleline(&mut self.mpn);

        ui.label("SAP Description");
        ui.text_edit_singleline(&mut self.sap_description);

        ui.label("Description");
        ui.text_edit_singleline(&mut self.part_description);

        ui.label("Detailed Description");
        ui.text_edit_singleline(
            &mut self.detailed_description,
        );

        ui.label("Manufacturer");
        ui.text_edit_singleline(&mut self.manufacturer);

        ui.label("Operating Temperature");
        ui.text_edit_singleline(
            &mut self.operating_temp,
        );

        ui.label("Datasheet");
        ui.text_edit_singleline(&mut self.datasheet);

        ui.separator();

        if ui.button("Add").clicked() {
            self.add_record();
        }

        if ui.button("Update").clicked() {
            self.update_record();
        }

        if ui.button("Delete").clicked() {
            self.delete_record();
        }

        if ui.button("Clear").clicked() {
            self.clear_fields();
        }
        if ui.button("Import Excel").clicked() {
    self.import_excel();
}

        ui.separator();

        ui.label(&self.message);
    }

    // ========================================================
    // TABLE
    // ========================================================

    fn draw_table(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");

            ui.text_edit_singleline(
                &mut self.search_text,
            );

            if ui.button("Search").clicked() {
                self.search();
            }

            if ui.button("Show All").clicked() {
                self.components =
                    self.db.load_all().unwrap_or_default();
            }
        });

        ui.separator();

        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                egui::Grid::new("component_table")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {

                    ui.label("ID");
                    ui.label("BPS Part Number");
                    ui.label("Value");
                    ui.label("Package/Material");
                    ui.label("MPN");
                    ui.label("SAP Description");
                    ui.label("Description");
                    ui.label("Detailed Description");
                    ui.label("Manufacturer");
                    ui.label("Operating Temp");
                    ui.label("Datasheet");
                    ui.label("Added On");

                    ui.end_row();

                    for component in self.components.clone() {
                        let selected =
                            self.selected_id == Some(component.id);

                        if ui
                            .selectable_label(
                                selected,
                                component.id.to_string(),
                            )
                            .clicked()
                        {
                            self.select_component(&component);
                        }

                        ui.label(&component.bps_part_number);
                        ui.label(&component.part_value);
                        ui.label(&component.package_material);
                        ui.label(&component.mpn);
                        ui.label(&component.sap_description);
                        ui.label(&component.part_description);
                        ui.label(&component.detailed_description);
                        ui.label(&component.manufacturer);
                        ui.label(&component.operating_temp);
                        ui.label(&component.datasheet);
                        ui.label(&component.added_on);

                        ui.end_row();
                    }
                });
            });
    }

    fn import_excel(&mut self) {
    let Some(path) = FileDialog::new()
        .add_filter("Excel files", &["xlsx", "xls"])
        .pick_file()
    else {
        return;
    };

    let mut workbook = match open_workbook_auto(&path) {
        Ok(wb) => wb,
        Err(e) => {
            self.message = format!("Failed to open Excel file: {}", e);
            return;
        }
    };

    let range = match workbook.worksheet_range("EBOM") {
        Ok(range) => range,
        Err(e) => {
            self.message = format!("Failed to read EBOM sheet: {}", e);
            return;
        }
    };

    /*
     * Your Python code used:
     *
     * pd.read_excel(file_path, sheet_name="EBOM", header=4)
     *
     * Therefore:
     *   Excel row 1 -> ignored
     *   Excel row 2 -> ignored
     *   Excel row 3 -> ignored
     *   Excel row 4 -> ignored
     *   Excel row 5 -> column headers
     */

    let mut rows = range.rows();

    // Skip first 4 rows
    for _ in 0..4 {
        rows.next();
    }

    // Row 5 = header
    let headers: Vec<String> = match rows.next() {
        Some(row) => row
            .iter()
            .map(|cell| cell.to_string().trim().to_string())
            .collect(),
        None => {
            self.message = "Excel file does not contain a header row.".to_string();
            return;
        }
    };

    // Find column index by column name
    let column_index = |name: &str| -> Option<usize> {
        headers
            .iter()
            .position(|header| header.eq_ignore_ascii_case(name))
    };

    let bps_idx = column_index("Synedyne Part Number");
    let value_idx = column_index("Value");
    let package_idx = column_index("Package/Material");
    let mpn_idx = column_index("New Part Number");
    let sap_idx = column_index("SAP Description");
    let description_idx = column_index("Description");
    let detailed_idx = column_index("Detailed Description");
    let manufacturer_idx = column_index("Manufacturer");
    let operating_temp_idx = column_index("Operating Temp");

    if bps_idx.is_none() {
        self.message =
            "Excel file is missing 'Synedyne Part Number' column.".to_string();
        return;
    }

    let get_cell = |row: &[Data], index: Option<usize>| -> String {
        match index {
            Some(i) if i < row.len() => {
                let value = &row[i];

                match value {
                    Data::Empty => String::new(),

                    Data::String(s) => {
                        s.trim().to_string()
                    }

                    Data::Float(v) => {
                        v.to_string()
                    }

                    Data::Int(v) => {
                        v.to_string()
                    }

                    Data::Bool(v) => {
                        v.to_string()
                    }

                    Data::DateTime(v) => {
                        v.to_string()
                    }

                    Data::DateTimeIso(v) => {
                        v.to_string()
                    }

                    Data::DurationIso(v) => {
                        v.to_string()
                    }

                    Data::Error(e) => {
                        format!("{:?}", e)
                    }
                }
            }

            _ => String::new(),
        }
    };

    let mut imported = 0;
    let mut skipped_empty = 0;
    let mut skipped_duplicate = 0;

    for row in rows {
        let bps_part_number = get_cell(row, bps_idx);

        // Same logic as your Python application
        if bps_part_number.is_empty() {
            skipped_empty += 1;
            continue;
        }

        let component = Component {
            id: 0,

            bps_part_number,

            part_value: get_cell(row, value_idx),

            package_material: get_cell(row, package_idx),

            mpn: get_cell(row, mpn_idx),

            sap_description: get_cell(row, sap_idx),

            part_description: get_cell(row, description_idx),

            detailed_description: get_cell(row, detailed_idx),

            manufacturer: get_cell(row, manufacturer_idx),

            operating_temp: get_cell(row, operating_temp_idx),

            // Same as your Python code
            datasheet: String::new(),

            added_on: chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        };

        match self.db.add(&component) {
            Ok(_) => {
                imported += 1;
            }

            Err(e) => {
                let error = e.to_string();

                // SQLite UNIQUE constraint means duplicate BPS number
                if error.contains("UNIQUE constraint")
                    || error.contains("unique")
                {
                    skipped_duplicate += 1;
                } else {
                    eprintln!(
                        "Failed to import {}: {}",
                        component.bps_part_number,
                        error
                    );
                }
            }
        }
    }

    // Reload database contents into GUI
    match self.db.load_all() {
        Ok(records) => {
            self.components = records;
        }

        Err(e) => {
            self.message = format!(
                "Import completed, but failed to refresh table: {}",
                e
            );
            return;
        }
    }

    self.message = format!(
        "Import completed: {} imported, {} duplicate, {} empty BPS number.",
        imported,
        skipped_duplicate,
        skipped_empty
    );
}
}

// ============================================================
// EFRAME
// ============================================================

impl eframe::App for ComponentApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::SidePanel::left("form_panel")
            .resizable(true)
            .show(ctx, |ui| {
                self.draw_form(ui);
            });

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.draw_table(ui);
            });
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true),

        ..Default::default()
    };

    eframe::run_native(
        "BPS Component Database",
        options,
        Box::new(|_cc| {
            Ok(Box::new(ComponentApp::new()))
        }),
    )
}