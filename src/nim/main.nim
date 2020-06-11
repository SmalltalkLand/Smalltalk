var snew, sat, slen: proc (v: int): int;
var sput: proc (a: int,b: int): int;
proc xinit(snew, sat, slen: proc (v: int) : int,sput: proc (a: int,b: int): int): bool =
    snew = snew
    sat = sat
    sput = sput
    slen = slen
    return 2 == 2
proc isnew(): bool =
    return snew(0) == 0
proc isat(k: int): int =
    return sat(k)
proc isput(k: int,val: int): bool =
    return sput(k,val) == 0
proc sfat(k: int): proc (a: varargs[int]): int =
    return (cast[proc (v: int): proc (a: varargs[int]): int](sat(999)))(k)
proc ssat(k: int): string =
    return cast[string](isat(k))
proc ssput(k: int,v: string): bool =
    return isput(k,cast[int](v))
proc islen(): int =
    return slen(0)
iterator siter(): int =
    for i in 0..islen():
        yield sat(i)
iterator stwice(): tuple[int,int] =
    for i in siter():
        for j in siter():
            yield (i,j)
