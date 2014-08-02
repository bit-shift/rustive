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
	let mut behaviour : Box<Behaviour<int>> = box Behaviour { 
		f: |time: Tm| -> int { return time.tm_sec as int; } 
	};
	loop {
		println!("{}", behaviour.pull(time::now()));	
	}
	
}

