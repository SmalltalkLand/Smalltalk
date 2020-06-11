pub trait Monad{
    type ThenType;
    fn then(&self,v: dyn FnMut(ThenType));
}
pub struct ComposeMonad<A,B>{
a: A,
b: dyn FnMut(A::ThenType) -> B,
}
pub impl Monad for ComposeMonad<A,B>{
    ThenType = B::ThenType;
    fn then(&self,vf: dyn FnMut(B::ThenType)){
        self.a.then(|v|{
            self.b(v).then(vf);
        })
    }
}
pub struct Continuation<T>{
    fnn: &dyn FnMut(dyn FnMut(T)),
}
pub impl Monad for Continuation<T>{
    ThenType = T;
    fn then(&self,vf: dyn FnMut(T)){
        self.fnn(vf);
    }
}
pub struct BothMonad<T,A,B> where A: Monad, B: Monad, A::ThenType == T, B::ThenType == T{
    ThenType = T;
    a: A,
    b: B,
}
pub impl Monad for BothMonad<T,A,B>{
    fn then(&self,vf: dyn FnMut(T)){
        self.a.then(vf);
        self.b.then(vf);
    }
}
pub fn waitM<T>(m: Box<dyn Monad<ThenType = T>>) -> T{
    let v: Option<T> = None;
    m.then(|vv|{v = Some(vv)});
    while (match v{Some(_) => false,None => true}){};
    return v.unwrap();
}