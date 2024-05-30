use xnor_macros::generate_rank_structs;
struct Rank0 {}
struct Rank1<const D0: usize> {}
struct Rank2<const D0: usize, const D1: usize> {}
struct Rank3<const D0: usize, const D1: usize, const D2: usize> {}
