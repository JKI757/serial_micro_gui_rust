mod serial_talker;
mod gui;

pub use crate::serial_talker::*;

pub use crate::gui::*;

fn main(){
	let current_ports = serial_talker::get_ports();
		
//	gui::run_iced_tour();
	gui::run_pick_list();
}