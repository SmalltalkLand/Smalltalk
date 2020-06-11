use core::ops::*;
use alloc::vec::*;
use super::super::monad::*;
use typenum::{Sum, Exp, Integer, Z0,P1,P2, P3, P4, N1, P1000000000000000000, True,False};
pub struct Cons<A,B>{
    a: A,
    b: B,
}
pub struct TypeCons<A,B> = ();
pub type TypeConsLoop<T,N = P1000000000000000000> = TypeCons<T,ConsLoop<Sum<N,N1>>>;
pub type If<True,A,B> = A;
pub type If<False,A,B> = B;
pub type ForCons<T,N> = If<Eq<N,Z0>,Z0,Cons<T,ForCons<T,Sum<N,N1>>>>;
pub type ForTypeCons<T,N> = If<Eq<N,Z0>,Z0,TypeCons<T,ForTypeCons<T,Sum<N,N1>>>>;
pub fn eval<Type>{Type::to_bool()}
pub type A<Cons<A,B>> = A;
pub type B<Cons<A,B>> = B;
pub type A<TypeCons<A,B>> = A;
pub type B<TypeCons<A,B>> = B;
pub type MCompose<Cons<A,B>> = ComposeMonad<A,B>;
pub type MCompose<TypeCons<A,B>> = ComposeMonad<A,B>;
pub type MBoth<Cons<A,B>> = BothMonad<A,B>;
pub type MBoth<TypeCons<A,B>> = BothMonad<A,B>;
pub type ConsList<T> = Cons<T,Option<ConsList<T>>>;

pub mod sf;

pub fn cons_to_vec<T>(v: ConsList<T>) -> Vec<T>{
    let ConsList<T> {a,b} = v;
    let v = vec![a];
    match b{
        None => v,
        Some(c) => {
            let v2 = cons_to_vec(c);
            v2.append(v);
            v2
        },
    }
}