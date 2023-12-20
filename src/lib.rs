/* --------- Combinators --------- */

#[inline]
pub fn b<F, G, X, Y, Z>(f: F, g: G) -> impl Fn(X) -> Z
where
    G: Fn(X) -> Y,
    F: Fn(Y) -> Z,
{
    move |x: X| f(g(x))
}

#[inline]
pub fn bb<F, G, H, W, X, Y, Z>(f: F, g: G, h: H) -> impl Fn(W) -> Z
where
    H: Fn(W) -> X,
    G: Fn(X) -> Y,
    F: Fn(Y) -> Z,
{
    move |w: W| f(g(h(w)))
}

#[inline]
pub fn bbb<F, G, H, I, V, W, X, Y, Z>(f: F, g: G, h: H, i: I) -> impl Fn(V) -> Z
where
    I: Fn(V) -> W,
    H: Fn(W) -> X,
    G: Fn(X) -> Y,
    F: Fn(Y) -> Z,
{
    move |v: V| f(g(h(i(v))))
}

#[inline]
pub fn b1<F, G, W, X, Y, Z>(f: F, g: G) -> impl Fn(W, X) -> Z
where
    G: Fn(W, X) -> Y,
    F: Fn(Y) -> Z,
{
    move |w, y| f(g(w, y))
}

#[inline]
pub fn c<F, X, Y, Z>(f: F) -> impl Fn(Y, X) -> Z
where
    F: Fn(X, Y) -> Z,
{
    move |y, x| f(x, y)
}

#[inline]
pub fn k<X, Y>() -> impl Fn(X, Y) -> X {
    move |x, _y| x
}

#[inline]
pub fn ki<X, Y>() -> impl Fn(X, Y) -> Y {
    move |_x, y| y
}

#[inline]
pub fn phi<F, G, H, W, X, Y, Z>(f: F, g: G, h: H) -> impl Fn(W) -> Z
where
    W: Clone,
    F: Fn(W) -> X,
    H: Fn(W) -> Y,
    G: Fn(X, Y) -> Z,
{
    move |w| g(f(w.clone()), h(w))
}

#[inline]
pub fn phi1<F, G, H, V, W, X, Y, Z>(f: F, g: G, h: H) -> impl Fn(V, W) -> Z
where
    V: Clone,
    W: Clone,
    F: Fn(V, W) -> X,
    H: Fn(V, W) -> Y,
    G: Fn(X, Y) -> Z,
{
    move |v, w| g(f(v.clone(), w.clone()), h(v, w))
}

#[inline]
pub fn psi<F, G, X, Y, Z>(f: F, g: G) -> impl Fn(X, X) -> Z
where
    G: Clone + Fn(X) -> Y,
    F: Fn(Y, Y) -> Z,
{
    move |a, b| f(g.clone()(a), g(b))
}

#[inline]
pub fn w<F, X, Y>(f: F) -> impl Fn(X) -> Y
where
    X: Clone,
    F: Fn(X, X) -> Y,
{
    move |x| f(x.clone(), x)
}
