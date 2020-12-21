mod tour;
mod pick_list_ex;
mod pane;
pub fn run_iced_tour(){
	tour::run_tour();
}

pub fn run_pick_list(){
	pick_list_ex::run_pick_list();
}

pub fn run_pane_test(){
	pane::run_pane();
}