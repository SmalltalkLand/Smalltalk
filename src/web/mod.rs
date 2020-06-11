trait Web{
    fn request_animation_frame(&mut self,f: dyn FnMut ());
    fn mark_function<T>(&mut self,f: T) -> T where T: FnMut;
}