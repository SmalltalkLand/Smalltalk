pub fn global<'b, R>(r: R<'b>) -> R<'static> {
    unsafe{core::mem::transmute::<R<'b>, R<'static>>(r)}
}
pub fn local<'b, 'c, R>(r: &'b mut R<'static>)
-> &'b mut R<'c> {
unsafe{core::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)}
}
pub unsafe fn mutt<T>(e: &T) -> &mut T{
    (core::mem::transmute<*const T,*mut T>(&e as *const T)) as &mut T
}
pub fn duplicate<T>(a: &T) -> [&T; 2]{
    unsafe{
        let n = a as *const T as u64;
        let na = [n as *const T as &T,n as *const T as &T];
        na
    }
}