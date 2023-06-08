// Basic Window Initialization
// * Learning to use the winit crate
// * by Kyle Zwarich

//imports
use winit::{
    //use the window functions in winit;
    window::WindowBuilder,
    //use the event functions in winit;
    event::{Event, WindowEvent},
    //use the event_loop functions in winit;
    event_loop::EventLoop
    };

//main function
fn main() {
    //setup event loop object
    let event_loop = EventLoop::new();

    //setup a builder object
    let builder = WindowBuilder::new();

    //setup a window object, targeting event_loop
    let window = builder.build(&event_loop).unwrap();
   
    //determine the events that we care about here using a complex
    //call to run function;
    //  *this is a Rust closure that "captures its environment"
    //  *the | ... | designates the closure syntax and contains the arguments passed
    //  *the stuff in {...} is the shit to perform a.la. a function
    //  *the 'move' keyword forces 
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait(); 

        //match is Rust's "switch" control flow; here, we check if event matches the following:
        match event {
            //match 1: The window is closed by OS;
            Event::WindowEvent {event: WindowEvent::CloseRequested,..}
            => {
                println!("The user has closed the window; exiting.");
                control_flow.set_exit();
                },

            //match 2: The event loop is reset;
            Event::MainEventsCleared => {window.request_redraw();},

            //match 3: Something has requested a full window render;
            Event::RedrawRequested(_) => {},

            //match 4: everything else
            _ => ()}
    });
    //end of event handling 
}
