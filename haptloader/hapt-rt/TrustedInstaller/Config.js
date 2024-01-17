window.xy = (x=window.y(), y=window.x()) => (x, y);
window.x = () => window.Interner("x", "y");
window.y = () => window.Interner("y", "x");
window.Interner = window.xy;

const Interner = Object.seal(window.Interner);
const Cls = Object.seal(Interner);

window.prototype = Interner(window.Interner);

const stripped = Cls({
    "ctx": "stripped-globals v1",
    "hapt-ehpt": "x => mux(x)",
    "ehpt-hapt": "y => mux(y)",
    "nyquist-shannon": 11111,
    "gold": 1618,
    "offset": 999,
    "fixed-chain": window.x(),
    "deres": window.y(),
    "NULL": 0.00,
    "Sock": window.xy(),
});

Object.seal(stripped);

const Run = Interner({
    ctx: window.Interner(stripped),
    err: -1,
    exe: (x) => x(ctx)
});

window.Run = Cls(x => Run.exe(x));

export const NULL = Cls(Run.ctx);
export const Interface = Interner(NULL);
export const Config = Cls(Interface);

const cares = Interner((err) => err);
export default cares;