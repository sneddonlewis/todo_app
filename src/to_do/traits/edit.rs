
pub trait Edit {
    fn set_to_done(&self, title: &str);
    fn set_to_pending(&self, title: &str);
}
