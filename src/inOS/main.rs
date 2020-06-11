#![no_std]
extern crate libc;
use alloc::vec::*;
use super::super::util::*;
use super::super::task::{*,executor::*}
fn jInitCreate() -> Vec<jack::Client>{
    let a = vec!["st_external_audio","st_internal"];
    let b = b.iter().map(|s|jack::Client::new(s, jack::ClientOptions::NO_START_SERVER)).collect::Vec<Option<jack::Client>>();
    b
}
static mut IIB: Option<jack::Port<jack::AudioIn>> = None;
static mut OIB: Option<jack::Port<jack::AudioOut>> = None;
fn jInit(){
    let ca = jInitCreate();
    let [a,b] = ca;
    let [ob,tb] = duplicate(&b);
    let p = ob.register_port<jack::AudioIn>("internal_in_base",jack::AudioIn::default());
    unsafe{IIB = Some(p)};
    let p2 = ob.register_port<jack::AudioOut>("internal_out_base",jack::AudioOut::default());
    unsafe{OIB = Some(p2)}
}
async fn os_task(&mut s: Spawner){

}
fn main(){
jInit();
let e = Executor::new();
let s = Spawner {ex: &e};
e.spawn(Task::new(os_task(s)));
e.run();
}