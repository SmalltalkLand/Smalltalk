#![no_std]
extern crate stdweb;
use stdweb;
use super::super::monad::*;
use super::super::task::{executor::*, *};
use super::*;
use super::super::util::*;
fn jscall(f: stdweb::Reference,v: Vec<dyn Any>) -> Option<stdweb::Reference>{
    js!{return @{f}(...@{v})}
}
fn jsget(j: stdweb::Reference,k: String) -> Option<stdweb::Reference>{
    js!{return @{j}[@{k}]}
}
fn jspromise<T>(f: dyn FnMut (stdweb::Reference)) -> stdweb::Reference{
js!{return new Promise(@{f})}
}
fn jsmonadpromise<T>(f: dyn Monad<ThenType = T>) -> stdweb::Reference{
    jspromise(|ff: stdweb::Reference|{
        f.then(|v|{
            jscall(ff,vec![v]);
        })
    })
}
struct BaseWeb{

};
impl Web for BaseWeb{
    fn request_animation_frame(f: dyn FnMut ()){
        js!{requestAnimationFrame(@{f})};
    }
    fn mark_function<T>(f: T) -> T{
        js!{self._temp = @{f}};
        js!{return self._temp}
    }
}
async fn mainTask(s: Spawner,w: &mut BaseWeb){

}
fn main(){

    let mut executor = Executor::new();
    let mut s = Spawner {ex: &executor};
    let mut bw = BaseWeb {};
    let mut task = Task::new(bw.mark_function(mainTask)(s,unsafe{mutt(&bw)}));
    task.web = &bw;
    executor.spawn(task);
    executor.web_run(|f|{
        js!{requestAnimationFrame(function(){
            let f = @{f};
            f();
        })}
    });
}