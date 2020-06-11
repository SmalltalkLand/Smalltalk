use alloc::vec::*;
extern "C" pub fn createH(t: [u8],args: [u64]);
extern "C" pub fn callH(h: u64,method: [u8],args: [u64]) -> u64;
extern "C" pub fn memAt(m: u64) -> u64;
extern "C" pub fn memPut(m: u64,v: u64);
extern "C" pub fn ccall(c: *mut u8);
extern "C" pub fn throw(h: u64) -> !;
extern "C" pub fn catch_base(f: fn (u64,*mut ()),cf: fn(u64,*mut ()),data: *mut ()) -> *mut ();
unsafe extern "C" pub fn trampoline_catch<F,T>(result: u64, user_data: *mut c_void)
where
    F: FnMut(u64) -> &T,
{
    let user_data = &mut *(user_data as *mut Vec<F>);
    return user_data[0](result) as *mut ();
}
pub fn get_trampoline_catch<F,T>(_closure: &F) -> AddCallback
where
    F: FnMut(u64) -> &T,
{
    trampoline_catch::<F>
}
unsafe extern "C" pub fn trampoline_catch_cc<F,T>(result: u64, user_data: *mut c_void)
where
    F: FnMut(u64) -> &T,
{
    let user_data = &mut *(user_data as *mut Vec<F>);
    return user_data[1](result) as *mut ();
}
pub fn get_trampoline_catch_cc<F,T>(_closure: &F) -> AddCallback
where
    F: FnMut(u64) -> &T,
{
    trampoline_catch_cc::<F>
}
pub fn catch<T>(t: &dyn Fn(u64) -> T,c: &dyn Fn(u64) -> T) -> T{
    unsafe{
        catch_base(get_trampoline_catch(t),get_trampoline_catch_cc(c),vec![t,c]) as &T
    }
}