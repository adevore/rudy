// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rudy;

#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn hashmap_new_drop(b: &mut Bencher) {
    use std::collections::HashMap;

    b.iter(|| {
        let m: HashMap<u32, u32> = HashMap::new();
        assert_eq!(m.len(), 0);
    })
}

fn hashmap_new_insert_drop(b: &mut Bencher) {
    use std::collections::HashMap;

    b.iter(|| {
        let mut m = HashMap::new();
        m.insert(0u32, 0);
        assert_eq!(m.len(), 1);
    })
}

fn hashmap_grow_by_insertion(b: &mut Bencher) {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1001;

    b.iter(|| {
        m.insert(k, k);
        k += 1;
    });
}

fn hashmap_find_existing(b: &mut Bencher) {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    b.iter(|| {
        for i in 1u32..1001 {
            m.contains_key(&i);
        }
    });
}

fn hashmap_find_nonexisting(b: &mut Bencher) {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    b.iter(|| {
        for i in 1001..2001 {
            m.contains_key(&i);
        }
    });
}

fn hashmap_as_queue(b: &mut Bencher) {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1;

    b.iter(|| {
        m.remove(&k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    });
}

fn hashmap_get_remove_insert(b: &mut Bencher) {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1;

    b.iter(|| {
        m.get(&(k + 400));
        m.get(&(k + 2000));
        m.remove(&k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    })
}

fn rudymap_new_drop(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    b.iter(|| {
        let m: RudyMap<u32, u32> = RudyMap::new();
        assert_eq!(m.len(), 0);
    })
}

fn rudymap_new_insert_drop(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    b.iter(|| {
        let mut m = RudyMap::new();
        m.insert(0u32, 0);
        assert_eq!(m.len(), 1);
    })
}

fn rudymap_grow_by_insertion(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    let mut m = RudyMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1001;

    b.iter(|| {
        m.insert(k, k);
        k += 1;
    });
}

fn rudymap_find_existing(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    let mut m = RudyMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    b.iter(|| {
        for i in 1u32..1001 {
            m.contains_key(i);
        }
    });
}

fn rudymap_find_nonexisting(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    let mut m = RudyMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    b.iter(|| {
        for i in 1001u32..2001 {
            m.contains_key(i);
        }
    });
}

fn rudymap_as_queue(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    let mut m = RudyMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1;

    b.iter(|| {
        m.remove(k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    });
}

fn rudymap_get_remove_insert(b: &mut Bencher) {
    use rudy::rudymap::RudyMap;

    let mut m = RudyMap::new();

    for i in 1u32..1001 {
        m.insert(i, i);
    }

    let mut k: u32 = 1;

    b.iter(|| {
        m.get(k + 400);
        m.get(k + 2000);
        m.remove(k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    })
}

benchmark_group!(benches,
    hashmap_get_remove_insert, hashmap_as_queue, hashmap_find_nonexisting,
    hashmap_find_existing, hashmap_grow_by_insertion, hashmap_new_insert_drop,
    hashmap_new_drop,
    rudymap_get_remove_insert, rudymap_as_queue, rudymap_find_nonexisting,
    rudymap_find_existing, rudymap_grow_by_insertion, rudymap_new_insert_drop,
    rudymap_new_drop
);
benchmark_main!(benches);
