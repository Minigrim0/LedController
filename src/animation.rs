use rs_ws281x::Controller;

pub trait Animation {
    fn next_frame(&mut self, controller: &mut Controller) -> bool;
    fn start(&mut self) -> ();
    fn stop(&mut self) -> ();
}