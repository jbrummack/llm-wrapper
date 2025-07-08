pub trait Model {
    fn modelname(&self) -> &'static str;
    fn allow_structured(&self) -> bool;
    fn allow_images_input(&self) -> bool {
        false
    }
    fn allow_images_output(&self) -> bool {
        false
    }
}
