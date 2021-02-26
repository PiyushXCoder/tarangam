/*
    This file is part of Tarangam.

    Tarangam is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Tarangam is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Tarangam.  If not, see <https://www.gnu.org/licenses/>
*/

//! Feel free to see through codes. Application is not written to be used as a library for other app. :)

pub struct Config {
    pub ui_file: String
}

impl Config {
    pub fn generate() -> Self {
        let ui_file = std::env::var("TARANGAM_UI_FILE");
        
        Config {
            ui_file: match ui_file {
                Ok(val) => val, 
                Err(_) => std::env::current_exe().unwrap().parent().unwrap()
                    .join("ui.glade").to_str().unwrap().to_owned()
            }
        }        
    }
}