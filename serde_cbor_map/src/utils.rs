pub(crate) fn list_to_tuple_2<A, B>(list: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut a: Vec<A> = Vec::new();
    let mut b: Vec<B> = Vec::new();

    for (ia, ib) in list {
        a.push(ia);
        b.push(ib);
    }

    (a, b)
}

pub(crate) fn list_to_tuple_4<A, B, C, D>(
    list: Vec<(A, B, C, D)>,
) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>) {
    let mut a: Vec<A> = Vec::new();
    let mut b: Vec<B> = Vec::new();
    let mut c: Vec<C> = Vec::new();
    let mut d: Vec<D> = Vec::new();

    for (ia, ib, ic, id) in list {
        a.push(ia);
        b.push(ib);
        c.push(ic);
        d.push(id);
    }

    (a, b, c, d)
}
