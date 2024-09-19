use gtk4::prelude::*;
use gtk4::{Label, TextView, CheckButton};
use std::path::PathBuf;
use std::process::Command;
use crate::modules::package;

pub fn handle_file_selection(
    path: PathBuf,
    status_label: &Label,
    with_dependencies: &CheckButton,
    log_view: &TextView
) {
    if path.extension().and_then(|s| s.to_str()) == Some("deb") {
        let package_info = package::get_package_info(&path);
        let package_details = package::get_package_details(&path);

        status_label.set_text(&format!("Package selected:\n{}", package_info));
        log_view.buffer().set_text(&package_details);

        install_package(
            path,
            status_label.clone(),
            with_dependencies.is_active(),
            log_view.clone(),
        )
    } else {
        status_label.set_text("Invalid file. Please select a .deb package.");
    }
}

pub fn install_package(
    path: PathBuf,
    status_label: Label,
    with_dependencies: bool,
    log_view: TextView,
) {
    glib::spawn_future_local(async move {
        let package_name = path.file_stem().unwrap().to_str().unwrap();

        // Check if package is already installed
        let check_command = Command::new("dpkg").arg("-s").arg(package_name).output();

        match check_command {
            Ok(output) => {
                if output.status.success() {
                    status_label.set_text("Package is already installed.");
                    log_view
                        .buffer()
                        .set_text("Package is already installed. No action taken.");
                    return;
                }
            }
            Err(_) => {
                status_label.set_text("Error checking package status.");
                log_view.buffer().set_text("Error checking package status.");
                return;
            }
        }

        let mut command = Command::new("pkexec");
        command.arg("apt").arg("install").arg("-y");

        if with_dependencies {
            command.arg("-f");
        }

        command.arg(path.to_str().unwrap());

        let log_buffer = log_view.buffer();
        log_buffer.set_text(&format!("Executing command: {:?}", command));

        let result = command.output();

        glib::MainContext::default().spawn_local(async move {
            match result {
                Ok(output) => {
                    if output.status.success() {
                        status_label.set_text("Package installed successfully!");
                        log_buffer.set_text(&format!(
                            "Success:\n{}",
                            String::from_utf8_lossy(&output.stdout)
                        ));
                    } else {
                        let error = String::from_utf8_lossy(&output.stderr);
                        status_label.set_text("Failed to install package. Check error message.");
                        log_buffer.set_text(&format!(
                            "Command failed:\nStatus: {:?}\nError: {}",
                            output.status, error
                        ));
                    }
                }
                Err(e) => {
                    status_label.set_text("Failed to run installation command.");
                    log_buffer.set_text(&format!("Error executing command: {}", e));
                }
            }
        });
    });
}
