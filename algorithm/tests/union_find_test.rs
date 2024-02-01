use algorithm::UnionFind;

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);

    // Make sure all elements belong to different groups by default
    assert!(!uf.same(0, 1));
    assert!(!uf.same(0, 2));
    assert!(!uf.same(1, 2));

    /*
        Combine elements using the union method and check
        that they belong to the same group using the same method
    */
    uf.union(0, 1);
    assert!(uf.same(0, 1));
    assert!(!uf.same(0, 2));
    assert!(!uf.same(1, 2));

    // Combine more elements and make sure the group structure is correct
    uf.union(1, 2);
    assert!(uf.same(0, 1));
    assert!(uf.same(0, 2));
    assert!(uf.same(1, 2));
}
