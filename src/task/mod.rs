use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use super::monad::*;
use super::util::*;
use super::web::*;

pub mod executor;
pub mod keyboard;
//pub mod simple_executor;
pub struct Prediction<'task, 'supertask>{
    caller: &'task Task<'task, 'supertask>,
}
pub struct Task<'this, 'super_> {
    id: TaskId,
    future: Pin<Box<dyn 'this + Stream<Item = (Message)>>>,
    prediction: Option<(Box<dyn FnMut(&mut &mut Context) -> Poll<()>>,dyn Monad + 'this<dyn FnMut() -> Box<dyn Monad<Prediction<'this + 'super>>>)>,
    terminated: bool,
    parent: Option<&'super_ dyn Spawn>,
    web: Option<&mut 'this (&mut dyn Web + 'static)>,
    spawnOnDie: Option<dyn FnOnce () -> Task>,
    _marker: core::marker::PhantomData<&'this + 'super_ u8>,
}
pub trait Spawn{
    fn spawn(&mut self,task: &mut Task);
}
impl Spawn for Task<'_,'_>{
    fn spawn(&mut self,task: &mut Task){
        task.parent = Some(self);
        match self.parent{
            Some(p) => unsafe{mutt(p)}.spawn(task),
            None => return,
        };
        task.parent = Some(self);
    }
}
impl Drop for Task<'_,'_>{
    fn drop(&mut self){
        self.terminated = true;
        match self.spawnOnDie{
            Some(v) => {
                self.parent.unwrap().spawn(v);
                self.spawnOnDie = None;
            },
            None => {},
        }
    }
}
pub enum Message{
    Run(Option<u64>),
}
impl Stream for Task<'_,'_>{
    Item = Message,
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Message>>{
        self.poll()
    }
}
impl Task<'_,'_> {
    pub fn new<'a,'b>(future: dyn Stream<Output = Option<Message>> + 'static) -> Task<'a,'b> {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
            prediction: None,
            terminated: false,
            parent: None,
            web: None,
            spawnOnDie: None,
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<(Option<Message>)> {
        if self.terminated{return Poll::Ready(None);};
        match self.prediction{
            None => {
                self.future.as_mut().poll_next(context);
            },
            Some(v) => {
                let p = self.future.as_mut().poll_next(context);
                match v{
                    (a,b) => {
                        let pp = a(&mut context);
                        let bb = waitM(b);
                        match p{
                            Poll::Ready(Some(v)) => match pp{Poll::Ready(_) => Poll::Ready(Some(v)),Poll::Pending => Poll::Ready(Some(v))},
                            Poll::Ready(None) => match pp{Poll::Ready(_) => Poll::Ready(None),Poll::Pending => Poll::Pending},
                            Poll::Pending => Poll::Pending,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
