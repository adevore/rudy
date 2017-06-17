#![feature(inclusive_range_syntax)]
extern crate rudy;
use rudy::rudymap::RudyMap;

fn main() {
    let mut map = RudyMap::new();
    let low = 0u32;
    let high = 10000u32;
    for i in low..high {
        map.insert(i, i + 1);
    }
    for i in low..high {
        assert_eq!(map.get(i).cloned(), Some(i + 1));
    }
    println!("{}", map.len());
    for i in low..high {
        assert_eq!(map.remove(i), Some(i + 1));
    }
    for i in low..high {
        assert_eq!(map.get(i), None);
    }
    for i in 5...7 {
        println!("{} => {:?}", i, map.get(i));
    }
    println!("{}", map.len());
}
