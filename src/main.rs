mod modules;

use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let application = Application::builder()
        .application_id("com.dragonailab.ile_package_installer")
        .build();

    application.connect_activate(modules::ui::build_ui);
    application.run();
}
