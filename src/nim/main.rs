extern "C" pub fn xinit(snew: *mut (),sat: *mut (),slen: *mut (),sput: *mut ()) -> bool;
static mut snew: *mut ();
static mut sat: *mut ();
static mut slen: *mut ();
static mut sput: *mut ();
pub fn set_snew(f: *mut ()) -> bool{unsafe{snew = f; xinit(snew,sat,slen,sput)}}
pub fn set_sat(f: *mut ()) -> bool{unsafe{sat = f; xinit(snew,sat,slen,sput)}}
pub fn set_slen(f: *mut ()) -> bool{unsafe{slen = f; xinit(snew,sat,slen,sput)}}
pub fn set_sput(f: *mut ()) -> bool{unsafe{sput = f; xinit(snew,sat,slen,sput)}}