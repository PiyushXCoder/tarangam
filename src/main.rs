/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#![windows_subsystem = "windows"]

mod config;

use gio::{prelude::*, ApplicationFlags};

#[tokio::main]
async fn main() {
    let app = gtk::Application::new(Some("sng.tarangm"), ApplicationFlags::default());

    let conf = config::Config::new(&app);

    app.connect_activate(move |app| {
        tarangam::build_ui(app, &conf.borrow().ui_file);
    });
    app.run();
}
