## disjoint-set-union
<hr/>
This data structure provides the following possibilities. Initially there are several elements, each in a separate (own) set. Two sets can be combined in a single operation, and it is also possible to query which set the given element is currently in. The algorithmic complexity on average of both operations is O(1)

### Usage:
```rust
use dsu::Dsu;

fn main() {
    let mut dsu = Dsu::new(10);
    
    dsu.union(1, 2).unwrap();
    dsu.union(2, 3).unwrap();
    dsu.union(2, 7).unwrap();

    assert_eq!(dsu.lookup(2).unwrap(), dsu.lookup(7).unwrap());
    assert_eq!(dsu.lookup(1).unwrap(), dsu.lookup(3).unwrap());
    assert_ne!(dsu.lookup(1).unwrap(), dsu.lookup(8).unwrap());
    assert_eq!(dsu.lookup(9).unwrap(), 9);

}
```

### Cargo.toml
```bash
[dependencies]
dsu = {git = "https://github.com/kedess/disjoint-set-union.git", branch="master"}
```
