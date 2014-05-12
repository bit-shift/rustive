use std::task::spawn;
use std::container::Map;
use std::vec::Vec;

trait Stream {
	fn data() -> f64;
}

struct PortAudio {
	adress: uint,
}

trait AudioOutput {
	fn write(channel: uint, stream: ~Stream);
	fn read(channel: uint) -> ~Stream;
}

impl AudioOutput for PortAudio {
	fn write(channel: uint, stream: ~Stream){

	}
	fn read(channel: uint) -> ~Stream {

	}
}

struct Mixer {
	channels: uint,
}


fn main() {
	spawn(proc() {
    	println!("I'm a new task")
	});
}

