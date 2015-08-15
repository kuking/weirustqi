
pub trait BrainKeeper {
    fn should_continue(&self, playouts :u32, millis :u32, win_est_range :u16) -> bool;
}

pub struct DefaultBrainKeeper {
    max_playouts :u32,
    max_millis  :u32,
    min_win_est_range :u16
}

impl DefaultBrainKeeper {
    pub fn new(max_playouts :u32, max_millis :u32, min_win_est_range :u16) -> Self {
        DefaultBrainKeeper {
            max_playouts :max_playouts,
            max_millis :max_millis,
            min_win_est_range :min_win_est_range
        }
    }
}

impl BrainKeeper for DefaultBrainKeeper {
    fn should_continue(&self, playouts :u32, millis :u32, win_est_range :u16) -> bool {
        self.max_playouts < playouts && self.max_millis < millis && self.min_win_est_range > win_est_range
    }
}
