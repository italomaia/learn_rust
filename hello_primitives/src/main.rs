use std::any::type_name;

fn get_type<T>(_: &T) -> &str {
    return type_name::<T>();
}

fn main() {
    let def_bool = true;
    let def_int = 10;
    let def_float = 10.0;
    let def_char = 'a';
    let def_str = "hi";
    // element types can be different
    let def_tup = (1,2.3,3);
    // element types cannot be different
    let def_arr = [1,2,3];
    
    println!("Auto Types for Primitives");
    println!("==========");
    
    println!(" bool: {}", get_type(&def_bool));
    println!("  int: {}", get_type(&def_int));
    println!("float: {}", get_type(&def_float));
    println!("  str: {}", get_type(&def_str));
    println!(" char: {}", get_type(&def_char));
    println!("  tup: {}", get_type(&def_tup));
    println!("  arr: {}", get_type(&def_arr));
    
    println!("\nForce Var Type Like This:");
    let my_var: u16 = 12;

    println!("my_var: {}", &my_var);

    println!("\nDeclaring Vars with Types");
    println!("===========================");

    let u8_num:u8 = 100;

    println!("u8_num value: {}", &u8_num);
    println!("u8_num  type: {}", get_type(&u8_num));

    println!("\nOverflow and Primitive Type Conversion");
    println!("======================================");

    // this fails at COMPILE time because the compiler knows the result is not u8
    // let u8_a: u8 = 150;
    // let u8_res = u8_a + u8_a;

    // this fails at RUNTIME because the compiler cannot verify the number
    let str_num = "150";
    let unk_num: u8 = str_num.parse().unwrap();
    // let u8_res:u8 = unk_num + unk_num;
    // this fails because rust will attempt to put the sum result in a u8 before converting
    // let u16_res: u16 = (unk_num + unk_num).into();

    // this works because we convert before summing so rust places the result into a u16
    let u16_res = unk_num as u16 + unk_num as u16;
    println!(" u16_res: {}", u16_res);

    // this also works, but the result will be coerced to the max and min limits
    let u8_res = u8::saturating_add(unk_num, unk_num);
    println!(" u8_res value: {}", u8_res);
    println!(" u8_res  type: {}", get_type(&u8_res));

    println!("\nChar Types");
    println!("============");



    println!("\nString Types!");
    println!("=============");

    let str_ref: &str = "Hello";
    let str_ins: String = String::from(str_ref);
    // slices using range operator
    let str_slice_a: &str = &str_ins[0..];  // 0-indexed; last param is length by default
    let str_slice_b: &str = &str_ins[..str_ins.len()];  // first param is 0 by default
    let str_slice_c: &str = &str_ins[0..5];   // exclusive
    let str_slice_d: &str = &str_ins[0..=4];  // inclusive
    let str_slice_e: &str = &str_ins[..];

    println!("  ref: {}", str_ref);
    println!("  ins: {}", str_ins);
    println!("slice_a: {}", str_slice_a);
    println!("slice_b: {}", str_slice_b);
    println!("slice_c: {}", str_slice_c);
    println!("slice_d: {}", str_slice_d);
    println!("slice_e: {}", str_slice_e);
    // slice does not have some methods available with instances
    // println!("len: {}", str_ref.len());
    println!("len: {}", str_ins.len());
}
