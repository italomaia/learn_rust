use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::vec;

fn main() {
    let mut vector:Vec<i32> = vec![1, 2, 3];  // same type; variable size
    let mut array:[String; 3] = [String::from("1"), String::from("2"), String::from("3")];  // same type; fixed size
    let mut tuple: (i32, f64, &str) = (1,2.2,"3");
    // max-heap
    let mut heap: BinaryHeap<i32> = BinaryHeap::from([6, 7, 5]);
    
    // unnordered hash
    let mut hash_map: HashMap<&str, bool> = HashMap::from([
        ("hi", true),
        ("hello", false),
        ("has", true),
    ]);

    // unnordered set
    let mut hash_set = HashSet::from([7,6,5]);
    
    // key-ordered hash using btree
    let mut tree_map = BTreeMap::new();
    tree_map.insert("hi", true);
    tree_map.insert("hello", false);
    tree_map.insert("has", true);
    
    // value-ordered set using btree
    let mut tree_set = BTreeSet::new();
    tree_set.insert(7);
    tree_set.insert(6);
    tree_set.insert(5);

    print_header("VECTOR".to_string());
    
    println!("vector: {:?}", &vector);
    
    vector.push(10);
    println!("push 10: {:?}", &vector);

    let mut tmp_vec = vec![20,30];
    vector.append(&mut tmp_vec);
    println!("append 20,30: {:?}", &vector);

    // remove by index
    vector.remove(2);
    println!("remove(2): {:?}", &vector);

    vector.pop();
    println!("pop (tail): {:?}", &vector);

    // 0-indexed
    println!("get(2): {:?}", vector.get(2));
    println!("get(10): {:?}", vector.get(10));
    // unsafe; will panic at runtime
    // println!("[10]: {:?}", vector[10]);
    
    print_header("ARRAY".to_string());
    
    println!("array: {:?}", array);
    println!("concat: {:?}", array.concat());
    println!("join(\" \"): {}", array.join(" "));
    println!("last: {:?}", array.last());
    // we clone to avoid array being borrowed
    let array_clone:[String; 3] = array.clone().map(|v| { format!("({})", v) });
    // similar result, but with .iter()
    let array_vec: Vec<String> = array.iter().map(|v| { format!("({})", v) }).collect();

    println!("map(v): {:?}", array_clone);
    println!("map(v): {:?}", array_vec);
    println!("len: {:?}", array.len());
    println!("get op[1]: {:?}", array[1]);  // unsafe

    array[1] = "42".to_string();
    println!("set op[42]: {:?}", array);

    print_header("TUPLE".to_string());

    println!("tuple: {:?}", tuple);
    println!("get op[0]: {}", tuple.0);
    println!("get op[1]: {}", tuple.1);
    println!("get op[2]: {}", tuple.2);
    
    tuple.0 = 42;
    println!("set op: {:?}", tuple);

    print_header("HEAP".to_string());

    println!("heap: {:?}", heap);
    println!("peek: {:?}", heap.peek());
    println!(" pop: {:?}", heap.pop());
    
    heap.push(-1);
    println!("push(-1); peek: {:?}", heap.peek());

    heap.push(99);
    println!("push(99); peek: {:?}", heap.peek());

    print_header("HASH_SET".to_string());
    println!("hash_set: {:?}", hash_set);
    
    hash_set.insert(42);
    println!("insert(42):  {:?}", hash_set);

    hash_set.insert(42);
    println!("insert(42) again: {:?}", hash_set);
    println!("contains(&42): {:?}", hash_set.contains(&42));

    print_header("HASH_MAP".to_string());
    
    println!("hash_map: {:?}", hash_map);
    
    hash_map.insert("york", true);
    println!("insert(\"york\", true)): {:?}", hash_map);
    println!("get(\"york\"): {:?}", hash_map.get("york"));
    println!("get(\"texas\"): {:?}", hash_map.get("texas"));
    
    print_header("TREE_MAP".to_string());
    println!("tree_map: {:?}", tree_map);
    
    tree_map.insert("york", true);
    println!("insert(\"york\", true)): {:?}", tree_map);
    println!("get(\"york\"): {:?}", tree_map.get("york"));
    println!("get(\"texas\"): {:?}", tree_map.get("texas"));
    
    print_header("TREE_SET".to_string());
    println!("tree_set: {:?}", tree_set);

    tree_set.insert(42);
    println!("insert(42):  {:?}", tree_set);

    tree_set.insert(42);
    println!("insert(42) again: {:?}", tree_set);
    println!("contains(&42): {:?}", tree_set.contains(&42));
}

fn print_header (header: String) {
    let len = header.len();
    
    println!("\n{}", header);
    println!("{}", "=".repeat(len));
}
