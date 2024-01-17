use std :: { sync::Arc, ffi::c_void, marker::{PhantomData, PhantomPinned}, ops::Div, borrow::Borrow };

macro_rules! xy {
    ($unsafe: ident $x: expr) => { $unsafe { std::mem::transmute($x) }};
    ($unsafe: ident $x: expr; $T1: ident => $T2: ident) => { $unsafe { std::mem::transmute::<$T1, $T2>($x) }};
}

static mut LOCAL_STORAGE: [I16;167_772_16] = [I16(-1,1);167_772_16];
static RESET_0: [I16;2] = xy!(unsafe [LOCAL_STORAGE[0], LOCAL_STORAGE[1]]);
static RESET_1: [I16;2] = xy!(unsafe [LOCAL_STORAGE[1], LOCAL_STORAGE[2]]);
static RESET_2: [I16;2] = xy!(unsafe [LOCAL_STORAGE[2], LOCAL_STORAGE[1]]);
static RESET_3: [I16;3] = xy!(unsafe [LOCAL_STORAGE[3], LOCAL_STORAGE[3], LOCAL_STORAGE[3]]);
static RESET_VECTORS: [[I16;3];2] = [[RESET_0[0], RESET_1[0], RESET_2[0]], RESET_3];

static RESET_A: u8 = RESET_VECTORS[0][0].0 as u8;
static RESET_B: u8 = RESET_VECTORS[0][0].1 as u8;
static RESET_C: u8 = RESET_VECTORS[0][1].0 as u8;
static RESET_D: u8 = RESET_VECTORS[0][1].1 as u8;

static JUMPER_A: [u8; 4] = [RESET_A, RESET_B, RESET_C, RESET_D];
static JUMPER_B: [u8; 4] = [RESET_D, RESET_C, RESET_B, RESET_A];

static INT: [u8; 4] = JUMPER_A;
static LONG: [u8; 8] = [RESET_A, RESET_B, RESET_C, RESET_D, RESET_D, RESET_C, RESET_B, RESET_A];

static U16: u16 = xy!(unsafe LOCAL_STORAGE[0]);
static U32: u32 = xy!(unsafe INT);
static U64: u64 = xy!(unsafe LONG);

macro_rules! global_proxy {
    ($x: ident => $y: ident) => {{
        global_proxy!($x);
        global_proxy!($y)
    }};

    (haptloader) => { Metaloader(0) };
    (v2) => { Metaloader(42) };

    (haptloader/v1) => {
        global_proxy!(haptloader => v2)
    }
}

static LOADER: Metaloader = global_proxy!(haptloader/v1);

fn hapt_storage() -> Interner {
    (LOADER.0.max(usize::MAX as i128).try_into().expect("[server] god left the game."), PhantomData)
}

fn cleanup() {
    dbg!("[cleanup] called");
}

#[derive(Copy, Clone, Debug)]
struct Metaloader(i128);

static META: [Metaloader; 3] = [Metaloader(42), Metaloader(0), Metaloader(1)];
static PROXY: Metaloader = Metaloader(42);

impl Metaloader {
    fn get_hashcode(port: &Proxy) {
        META.get(0).map_or_else(|| eprintln!("get failure"), |x| Proxy::send(port));
    }

    fn init(name: &str, id: i128) -> Metaloader {
        dbg!("metaloader: ".to_owned() + name);
        Self { 0: id }
    }

    fn get(hash: usize) -> Self {
        META.get(hash).expect("failed to get metaloader").clone()
    }
}

impl AsRef<Metaloader> for Metaloader {
    fn as_ref(&self) -> &Metaloader {
        Arc::new(self).as_ref()
    }
}
trait Superclass {
    fn new_super() -> Metaloader;
    fn call_super(x: Interner) -> Metaloader;
}

impl Superclass for Metaloader {
    fn new_super() -> Metaloader {
        global_proxy!(haptloader/v1)
    }

    fn call_super(x: Interner) -> Metaloader {
        META.get(2).expect("[hapt-storage] failed to get storage device").0;
        global_proxy!(haptloader)
    }
}

trait Fn: Superclass + Sized {
    fn new() -> Metaloader {
        Self::new_super()
    }

    fn call(x: Gc) -> Metaloader {
        Self::call_super((x.0.0, x.0.1))
    }
}

type Ident = Any<PhantomPinned>;

macro_rules! stdcall {
    ([haptloader/stdcall] $proxy:expr;$iters:expr) => {{
        let iters = $iters;
        iters.iter().map(|x| println!("{:?}", global_proxy!(haptloader/v1).0));
        global_proxy!(haptloader)
    }};
}

macro_rules! proxy {
    ([haptloader/Proxy]<=$iters:expr=>) => {{
        stdcall!([haptloader/stdcall] global_proxy!(haptloader/v1);$iters)
    }}
}

macro_rules! call {
    ([haptloader/Proxy]<=$expr:expr=>) => {{
        proxy!(
            [haptloader/Proxy] <= 
                vec![|x: &str|dbg!("proxy-pass".to_owned() + x)] 
        =>  )
    }}
}

#[derive(Copy, Clone, Debug)]
struct I16(i8, i8);

struct Proxy(Interner);

impl Proxy {
    fn send<T>(data: T) {
        dbg!("[io] send");
    }

    fn recv<T>(hook: &mut Interner) -> usize {
        dbg!("[io] recv hook");
        let unit: usize = usize::MAX;
        let identity = |x: Proxy| (x, unit).0;
        hook.0 = unsafe { std::mem::transmute(identity) };
        hook.0
    }
}

struct ProxyPass<T>(
    Interner, Interner, Interner, Interner,
    c_void, c_void, c_void, c_void,
    T, T, T, T
);

impl Superclass for ProxyPass<usize> {
    fn new_super() -> Metaloader {
        global_proxy!(haptloader)
    }

    fn call_super(x: Interner) -> Metaloader {
        println!("[proxy-pass] {}", x.0);
        global_proxy!(haptloader/v1)
    }
}

impl Fn for ProxyPass<usize> {
    fn new() -> Metaloader {
        global_proxy!(haptloader/v1)
    }

    fn call(x: Gc) -> Metaloader {
        x.borrow();
        call!([haptloader/Proxy] <= { cleanup(); x } =>)
    }
}

trait ProxyTrait: Superclass {
    fn merge<T>(x: T, ext: Proxy) -> isize {
        (global_proxy!(haptloader).0.max(isize::MAX.try_into().unwrap()).div(2) + (ext.0.0 as i128).div(2)) as isize
    }

    fn mux(&self, ext: Proxy) -> isize {
        println!("[hapt-muxer] muxing");
        Self::merge(self, ext)
    }

    fn call(&self, x: Proxy) -> Metaloader {
        self.mux(x);
        global_proxy!(haptloader/v1)
    }
}

type U16 = Proxy;

struct U24(U16, i8);
struct U48(U24, U16, i8);

type Any<Underlying> = (usize, PhantomData<Underlying>);
type Box: = Any<I16>;

trait Intern<T>: Send + Unpin {}
type Interner = Box;

struct Gc(Interner);
struct Rc(Gc, I16);

impl Rc {
    fn new(gc: Gc) -> Proxy {
        println!("[hapt-gc/{:?}]", std::any::Any::type_id(&gc));
        Proxy((gc.0.0, gc.0.1))
    }
}

impl Into<Gc> for Metaloader {
    fn into(self) -> Gc {
        todo!()
    }
}

macro_rules! map {
    (index $x: expr) => {{
        let ux: [u64;2] = xy!(unsafe $x);
        xy!(unsafe ux[0]; u64 => usize)
    }};

    (get $to: ident => $proxy: expr) => {{
        map!(index global_proxy!(haptloader/v1))
    }};

    (set $to: ident => $proxy: expr; $data: expr) => {
        let idx = map!(get $to => $proxy);
        println!("[hapt-storage/{}]: {:?}", idx, $data);
        unsafe { LOCAL_STORAGE[idx + 0xFF] = $data };
    };
}

fn main() -> () {
    let _rc = Rc::new(Gc((xy!(unsafe 42u64), PhantomData)));
    let mut idx: I16 = xy!(unsafe 0i16);
    loop {
        idx.0 = idx.0.wrapping_add(1);
        map!(set v2 => _rc; I16(idx.1 as i8, idx.0 as i8));

        idx.1 = idx.1.wrapping_add(2);
        map!(get v2 => _rc);
    }
}