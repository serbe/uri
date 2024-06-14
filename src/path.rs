pub enum Path {
    abempty,
    absolute(Option<RangeUsize>),
    noscheme(RangeUsize),
    rootless(RangeUsize),
    empty,
}
