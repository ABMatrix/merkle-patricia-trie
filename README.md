## Merkle-Patricia-Trie

Rust implementation of the Modified Patricia Tree (aka Trie),

The implementation is strongly inspired by [cita trie](https://github.com/cryptape/cita-trie)

## Features

- Implementation of the Modified Patricia Tree


## Example

```rust
use std::rc::Rc;
use merkle_patricia_trie::{MemoryDB, MerklePatriciaTrie, Trie};

fn main() {
        let memdb = Rc::new(MemoryDB::new());
        let key = b"test-key".to_vec();
        let value = b"test-value".to_vec();

        let root = {
            let mut trie = MerklePatriciaTrie::new(Rc::clone(&memdb));
            trie.insert(key.clone(), value.clone()).unwrap();

            let v = trie.get(&key).unwrap();
            assert_eq!(Some(value.to_vec()), v);
            trie.root().unwrap()
        };

        let mut trie = MerklePatriciaTrie::from(Rc::clone(&memdb), &root).unwrap();
        let exists = trie.contains(&key).unwrap();
        assert_eq!(exists, true);
        let removed = trie.remove(&key).unwrap();
        assert_eq!(removed, true);
        let new_root = trie.root().unwrap();
        println!("new root = {:?}", new_root);
}

```