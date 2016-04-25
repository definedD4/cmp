use glium::glutin::Window;

struct Display {
	window: Window,
}

impl Display {
	pub fn new() -> Display {
		let window = glium::glutin::WindowBuilder::new().build_glium().unwrap();
		Display{window:window}
	}

	pub fn run(&mut self) {
		use glium::{DisplayBuild, Surface};
    
	    loop {
	        let mut target = self.window.draw();
	        target.clear_color(0.0, 0.0, 1.0, 1.0);
	        target.finish().unwrap();

	        for ev in self.window.wait_events() {
	            match ev {
	                glium::glutin::Event::Closed => return,
	                _ => ()
	            }
	        }
	    }
	}
}