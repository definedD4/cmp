extern crate glium;

use glium::glutin::GlutinFacade;

pub struct Display {
	window: GlutinFacade,
}

pub impl Display {
	pub fn new() -> Display {
		use glium::DisplayBuild;
		let window = glium::glutin::WindowBuilder::new().build_glium().unwrap();
		Display{window:window}
	}

	pub fn run(&mut self) {
		use glium::Surface;
    
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