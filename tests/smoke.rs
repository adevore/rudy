extern crate rudy;

#[test]
fn smoke_test() {
    use rudy::rudymap::RudyMap;

    let mut map: RudyMap<u32, u32> = RudyMap::new();
    let n = 10_000;

    for i in 0..n {
        assert!(map.insert(i, i).is_none());
    }

    for i in 0..n {
        assert_eq!(map.remove(i), Some(i));
    }
}
