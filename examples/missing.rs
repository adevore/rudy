extern crate rudy;

use rudy::rudymap::RudyMap;

fn main() {
    let mut map: RudyMap<u32, u32> = RudyMap::new();
    let low = 0;
    let high = 100_000_000;
    for i in low..high {
        match map.insert(i, i + 1) {
            Some(evicted) => println!("Evicted: {}", evicted),
            None => {}
        }
    }
    for i in low..high {
        if map.get(i) != Some(&(i + 1)) {
            println!("map({}) -> {:?} != {}", i, map.get(i), i + 1);
        }
    }
}
