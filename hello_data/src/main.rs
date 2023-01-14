use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::vec;

fn main() {
    let mut vector:Vec<i32> = vec![1, 2, 3];  // same type; variable size
    let array:[i32; 3] = [1,2,3];  // same type; fixed size
    let tuple: (i32, f64, &str) = (1,2.2,"3");
    // max-heap
    let heap: BinaryHeap<i32> = BinaryHeap::from([6, 7, 5]);
    
    // unnordered hash
    let hash_map: HashMap<&str, bool> = HashMap::from([
        ("hi", true),
        ("hello", false),
        ("has", true),
    ]);

    // unnordered set
    let hash_set = HashSet::from([7,6,5]);
    
    // key-ordered hash using btree
    let mut tree_map = BTreeMap::new();
    tree_map.insert("hi", true);
    tree_map.insert("hi", true);
    tree_map.insert("hello", false);
    tree_map.insert("has", true);
    
    // value-ordered set using btree
    let mut tree_set = BTreeSet::new();
    tree_set.insert(7);
    tree_set.insert(6);
    tree_set.insert(5);

    println!("array: {:?}", array);
    println!("tuple: {:?}", tuple);
    println!("heap: {:?}", heap);
    println!("hash_set: {:?}", hash_set);
    println!("hash_map: {:?}", hash_map);
    println!("tree_map: {:?}", tree_map);
    println!("tree_set: {:?}", tree_set);
    
    println!("\nVECTOR");
    println!("========");
    
    println!("vector: {:?}", &vector);
    
    vector.push(10);
    println!("push 10: {:?}", &vector);
    
    let mut tmp_vec = vec![20,30];
    vector.append(&mut tmp_vec);
    println!("append 20,30: {:?}", &vector);

    vector.pop();
    println!("pop (tail): {:?}", &vector);

    // 0-indexed
    println!("get(2): {:?}", vector.get(2));
    println!("get(10): {:?}", vector.get(10));
    // unsafe; will panic at runtime
    // println!("[10]: {:?}", vector[10]);
    
    // WIP
}
