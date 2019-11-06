## Merkle-Patricia-Trie

Rust implementation of the Modified Patricia Tree (aka Trie),

The implementation is strongly inspired by [cita trie](https://github.com/cryptape/cita-trie)

## Features

- Implementation of the Modified Patricia Tree.
- Support `no_std` build.

## Example

```rust
use std::rc::Rc;
use merkle_patricia_trie::{build_trie, Trie};

fn main() {
    let key = b"test-key".to_vec();
    let value = b"test-value".to_vec();
        
    let mut trie = build_trie(vec![(key.clone(), value.clone())]).unwrap();
    let exists = trie.contains(&key).unwrap();
    assert_eq!(exists, true);
    let removed = trie.remove(&key).unwrap();
    assert_eq!(removed, true);
    let new_root = trie.root().unwrap();
    println!("new root = {:?}", new_root);
}

```