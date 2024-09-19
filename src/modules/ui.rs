use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, CheckButton, DropTarget, FileChooserDialog, Label,
    Orientation, ResponseType, ScrolledWindow, Separator, TextView,
};
use gtk4::gdk::DragAction;
use gtk4::gio::File;
use crate::modules::installation;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ile - Package Installer")
        .default_width(500)
        .default_height(400)
        .modal(true)
        .build();

    let vbox = create_main_box();
    let (label, file_button, status_label, with_dependencies, log_view) = create_ui_elements();

    vbox.append(&label);
    vbox.append(&file_button);
    vbox.append(&status_label);
    vbox.append(&with_dependencies);
    vbox.append(&Separator::new(Orientation::Horizontal));

    let scrolled_window = ScrolledWindow::builder()
        .child(&log_view)
        .hexpand(true)
        .vexpand(true)
        .build();
    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));

    setup_file_button(&file_button, &window, &status_label, &with_dependencies, &log_view);
    setup_drop_target(&window, &status_label, &with_dependencies, &log_view);

    window.set_icon_name(Some("resources/icons/launcher.png"));
    window.show();
}

fn create_main_box() -> Box {
    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.set_margin_bottom(10);
    vbox.set_margin_end(10);
    vbox.set_margin_start(10);
    vbox.set_margin_top(10);
    vbox
}

fn create_ui_elements() -> (Label, Button, Label, CheckButton, TextView) {
    let label = Label::new(Some(
        "Select a package to install or drag and drop a .deb file:",
    ));
    let file_button = Button::with_label("Select Package");
    let status_label = Label::new(Some("No package selected."));
    let with_dependencies = CheckButton::builder()
        .label("Install with dependencies")
        .active(true)
        .build();
    let log_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .build();

    (label, file_button, status_label, with_dependencies, log_view)
}

fn setup_file_button(
    button: &Button,
    window: &ApplicationWindow,
    status_label: &Label,
    with_dependencies: &CheckButton,
    log_view: &TextView,
) {
    button.connect_clicked({
        let window = window.clone();
        let status_label = status_label.clone();
        let with_dependencies = with_dependencies.clone();
        let log_view = log_view.clone();
        move |_| {
            let dialog = FileChooserDialog::builder()
                .title("Select Package")
                .action(gtk4::FileChooserAction::Open)
                .transient_for(&window)
                .modal(true)
                .build();

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Open", ResponseType::Accept);

            let value = status_label.clone();
            let deps = with_dependencies.clone();
            let logs = log_view.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            installation::handle_file_selection(path, &value, &deps, &logs);
                        }
                    }
                }
                dialog.close();
            });

            dialog.show();
        }
    });
}

fn setup_drop_target(
    window: &ApplicationWindow,
    status_label: &Label,
    with_dependencies: &CheckButton,
    log_view: &TextView,
) {
    let drop_target = DropTarget::new(File::static_type(), DragAction::COPY);
    drop_target.connect_drop({
        let status_label = status_label.clone();
        let with_dependencies = with_dependencies.clone();
        let log_view = log_view.clone();
        
        move |_target, value, _x, _y| {
            if let Ok(file) = value.get::<File>() {
                if let Some(path) = file.path() {
                    installation::handle_file_selection(path, &status_label, &with_dependencies, &log_view);
                    return true;
                }
            }
            false
        }
    });

    window.add_controller(drop_target);
}
