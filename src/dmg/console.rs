use super::dmg_cpu::Cpu;
use super::interconnect::Interconnect;
pub use super::gamepad::{InputEvent,Gamepad,Button,ButtonState};

pub use super::cart::Cart;

// Trait for objects that receive video data, and then render video to display video frames.
pub trait VideoSink {
    fn frame_available(&mut self, frame: &Box<[u32]>);
}

// FrameHandler: A struct that contains any ???
struct FrameHandler<'a> {
    frame_available: bool,
    video_sink: &'a mut dyn VideoSink,
}

impl<'a> FrameHandler<'a> {
    fn new(video_sink: &'a mut dyn VideoSink) -> Self {
        FrameHandler {
            frame_available: false,
            video_sink,
        }
    }
}

impl<'a> VideoSink for FrameHandler<'a> {
    fn frame_available(&mut self, frame: &Box<[u32]>) {
        self.video_sink.frame_available(frame);
        self.frame_available = true;
    }
}

pub struct Console {
    cpu: Cpu,
}

impl Console {
    pub fn new(cart: Cart) -> Console {
        let interconnect = Interconnect::new(cart);
        Console {
            cpu: Cpu::new(interconnect),
        }
    }

    pub fn run_for_one_frame(&mut self, video_sink: &mut dyn VideoSink) {
        let mut frame_handler = FrameHandler::new(video_sink);
        while !frame_handler.frame_available {
            self.cpu.step(&mut frame_handler);
        }
    }
    
    pub fn handle_event(&mut self, input_event: InputEvent) {
        self.cpu.interconnect.gamepad.handle_event(input_event);
    }

    /* TODO: implement copy_ram in cart?
        pub fn copy_cart_ram(&self) -> Option<Box<[u8]>> {
            self.cpu.interconnect.cart.copy_ram()
        }
    */
}


