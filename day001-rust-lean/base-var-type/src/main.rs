fn main() {
    // 1.3.2 基础数据类型
    // Rust 是强类型语言，每个值都有确切的类型

    // 标量类型
    // 1 整数类型

    // Rust 中整数类型分为有符号和无符号类型；长度分为8位，16位，32位，64位，128位
    // 特殊的整数类型: usize 和 isize，与系统架构相关，32位的系统则为32位，64位的系统为64位

    let integer: i32 = 42;
    let integer: i32 = 45;

    let s: usize = 100;
    let s: usize = 100;
    // 2 浮点类型
    // Rust 的浮点型是 f32 和 f64，大小分别为 32 位和 64 位。默认浮点类型是 f64
    // 浮点型都是有符号的

    let x = 2.0; // f64
    let x = 2.0;
    let y: f32 = 3.0; // f32
    let y: f32 = 2.0;
    // 3 布尔类型

    let t = true;
    let t = true;
    let f: bool = false;
    let f: bool = false;

    // 4 字符类型 char
    // Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值

    let c = 'z';
    let c = 'Z';
    let z = 'ℤ';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    let emoji = '🌽';
    let char_size = std::mem::size_of::<char>();
    let char_size = std::mem::size_of::<char>();

    println!("char_size={}", char_size);

    // 复合类型
    // Rust 中的复合类型主要有元组和数组

    // 1 元组
    // Rust中的元组可以将各种类型组合起来
    let types = (42, "Rust", true);

    // 可以通过下标索引访问
    println!("num is {}", types.0);

    // 单元类型 （）
    // 单元类型在Rust中是非常重要的类型，如果表达式不返回任何其他值，就隐式地返回单元值，
    // 如没有返回值的函数或者作用域

    let a: () = {};
    fn return_tuple() {}
    let func: () = return_tuple();
    assert_eq!(a, func);

    // 2 数组
    // 通过索引来访问或者修改数组中的元素

    let arr = [1, 2, 3, 4, 5];

    let mut arr1 = [0, 0, 0, 0, 0];
    arr1[0] = 100;
    println!("{:?}", arr1); // [100, 0, 0, 0, 0]

    // 1.3.3 进阶数据类型
    // 字符串
    // Rust 中的字符串比较复杂，有多种形式，适用于不同的场景。核心是需要掌握 &str 和 String

    // Rust 在编译代码时需要在编译期就能够确定类型的大小，而字符串 str 本身是动态大小的，因而日常中我们更多使用的是字符串的引用 &str 和 String

    // 1 &str：字符串字面量的引用
    // 字符串字面量实际上存放在程序的只读数据段中，在程序运行时会被加载到内存中读取
    let s = "Hello Rust Lang";
    let mut s1 = "Hello Go per";

    s1 = "Hello Rust";
    println!("{}", s1);

    // 2 String：字符串切片的引用
    // String 通过指针指向存放在堆上的字符串

    // 有多种方式可以在堆上创建字符串
    // let s2 = String::new();         // 空字符串
    // let s2 = "Hello Rust".to_string();
    // let s2: String = "Hello Rust".into();
    let s2 = String::from("Hello Rust");

    // 可以使用ptr、len、cap获取String的指针、长度和容量

    let s2 = String::from("Hello Rust");
    let cap = s2.capacity();
    let len = s2.len();
    let ptr = s2.as_ptr();

    println!("len {}", len); // 10
    println!("cap {}", cap); //10
    println!("pointer {:?}", ptr); //0x55da0e49bba0

    // 3 字符串切片
    // 字符串本质上一个u8序列，支持切片操作

    let s1 = String::from("Hello Rust");
    let s2 = "Hello Rust";

    let slice1 = &s1[0..5];
    let slice2 = &s2[6..10];

    println!("slice1: {}", slice1); // Hello
    println!("slice2: {}", slice2); // Rust

    // 引用
    // Rust 中的引用类型是一等公民，并且和借用指同一个概念。从可变性上可以分为可变引用和不可变引用

    // 1 不可变借用
    let num = 42;
    let immutable_s = &num;

    // 2 可变借用
    let mut num = 42;
    let mutable_s = &mut num;

    // 当类型占用空间比较大时，可以通过引用来访问或者修改数据,尤其是在传递数据的场景下

    let person_tuple = ("Rust", 13, true);

    let ptr = &person_tuple;
    println!("{}", ptr.0);

    let mut arr = ["Rust", "Go", "C++"];

    let arr_ptr = &mut arr;

    arr_ptr[2] = "Java"; // Change C++ To Java

    println!("{:?}", arr_ptr) // ["Rust", "Go", "Java"]
}
