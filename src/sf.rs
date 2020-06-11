use enforcer::base::*;
use typenum::{False,True,Integer,Sub,P1,N1,Z0};
type F = False;
type T = True;
type_operators! {
    [ZZ, ZZZ, ZZZZ, ZZZZZ, ZZZZZZ]

    concrete List => BitVec {
        Nil => BitVec::new(),
        Cons(B: Bit, L: List = Nil) => { let mut tail = L; tail.push(B); tail },
    }
    concrete ProgramTy => Program {
        Empty => Program::Empty,
        Left(P: ProgramTy = Empty) => Program::Left(Box::new(P)),
        Right(P: ProgramTy = Empty) => Program::Right(Box::new(P)),
        Flip(P: ProgramTy = Empty) => Program::Flip(Box::new(P)),
        Loop(P: ProgramTy = Empty, Q: ProgramTy = Empty) => Program::Loop(Box::new((P, Q))),
        Hook(H: TypeCons<T,ProgramTy = Empty>) => T<H>,
    }
    
    concrete StateTy => StateTyOut {
        St(L: List, C: Bit, R: List) => {
            let mut bits = L;
            let loc = bits.len();
            bits.push(C);
            bits.extend(R.into_iter().rev());
    
            StateTyOut {
                loc: loc,
                bits: bits,
            }
        },
    }
    (Run) Running(ProgramTy, StateTy): StateTy {
        forall (P: ProgramTy, C: Bit, R: List) {
            [(Left P), (St Nil C R)] => (# P (St Nil F (Cons C R)))
        }
        forall (P: ProgramTy, L: List, C: Bit) {
            [(Right P), (St L C Nil)] => (# P (St (Cons C L) F Nil))
        }
        forall (P: ProgramTy, L: List, C: Bit, N: Bit, R: List) {
            [(Left P), (St (Cons N L) C R)] => (# P (St L N (Cons C R)))
            [(Right P), (St L C (Cons N R))] => (# P (St (Cons C L) N R))
        }
        forall (P: ProgramTy, L: List, R: List) {
            [(Flip P), (St L F R)] => (# P (St L T R))
            [(Flip P), (St L T R)] => (# P (St L F R))
        }
        forall (P: ProgramTy, Q: ProgramTy, L: List, R: List) {
            [(Loop P Q), (St L F R)] => (# Q (St L F R))
            [(Loop P Q), (St L T R)] => (# (Loop P Q) (# P (St L T R)))
        }
        forall (S: StateTy) {
            [Empty, S] => S
        }
    }

}
pub type Blank = St<Nil, F, Nil>;
macro_rules! sf {
    (< $($prog:tt)*) => { Left<sf!($($prog)*)> };
    (> $($prog:tt)*) => { Right<sf!($($prog)*)> };
    (* $($prog:tt)*) => { Flip<sf!($($prog)*)> };
    ([$($inside:tt)*] $($outside:tt)*) => { Loop<sf!($($inside)*), sf!($($outside)*)> };
    (($($progr:tt)*) : $($progs:tt)) => {Hook<TypeCons<$($progr)*,sf!($($progs)*)>>};
    () => { Empty };
}
