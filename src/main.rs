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

use gio::prelude::*;

use std::env::args;
use std::sync::Arc;

use tarangam_dwij::util::Config;


#[tokio::main]
async fn main() {
    let conf = Arc::new(Config::default());

    let app = gtk::Application::new(Some("sng.tarangm"), Default::default())
    .expect("Failed to initiate gtk");

    let tmp_conf = Arc::clone(&conf);
    app.connect_activate(move |app| {
        tarangam_dwij::build_ui(app, &tmp_conf);
    });

    app.run(&args().collect::<Vec<_>>());
}
