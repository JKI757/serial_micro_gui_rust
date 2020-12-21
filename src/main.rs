mod serial_talker;
mod view;

pub use crate::serial_talker::*;

pub use crate::view::*;

fn main(){
	let current_ports = serial_talker::get_ports();
		
	// view::run_iced_tour();
//	view::run_pick_list();
	view::run_pane_test();
}