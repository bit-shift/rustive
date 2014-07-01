//!----------------------------------------------------------------------------
//! rustive - Rust port of sbase FRP framework (https://github.com/camio/sbase)
//!----------------------------------------------------------------------------
extern crate time;

use time::Tm;
use std::option;


struct Behaviour<'a, T> {
	f: |time: Tm| : 'a -> T,		
}


impl<'a, T> Behaviour<'a, T> {
	fn pull(&mut self, t: Tm) -> T {
		(self.f)(t)
	}
}

fn main() {
	let time_function = |time: Tm| -> int { return 4; };
	let mut behaviour : Box<Behaviour<int>> = box Behaviour { f: time_function };
	println!("{}", behaviour.pull(time::now()));
}

