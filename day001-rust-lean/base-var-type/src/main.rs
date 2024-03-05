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

    // #########################################
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

    // #########################################
    // 1.3.3 进阶数据类型

    // #########################################
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

    // #########################################
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
    println!("{}", ptr.0); // Rust

    let mut arr = ["Rust", "Go", "C++"];

    let arr_ptr = &mut arr;

    arr_ptr[2] = "Java"; // Change C++ To Java

    println!("{:?}", arr_ptr); // ["Rust", "Go", "Java"]

    // #########################################
    // 集合
    // 两个重要的集合 Vec 和 HashMap，这里的集合指的是它们都聚集了多个同类型的元素

    // 1 Vec
    // Vec是动态大小，相比起数组来说，它更加常用
    // Vec中的元素必须相同
    // 需要注意的是，如果没有指定类型，那么 vec 需要通过上下文去推测，所以必须在接下来的代码总使用，否则会提示
    // an explicit type, where the type for type parameter `T` is specified

    // let mut vec1 = Vec::<String>::new();
    let mut vec1 = Vec::new();
    let mut vec2 = vec![];
    // let mut vec1_1 = Vec::<String>::new();
    // let mut v3c2_1: Vec<String> = vec![];

    // vec 支持一系列的操作

    // 添加元素
    vec1.push("Rust");
    vec2.push("Go");

    // 当作栈
    vec1.pop();

    // 修改数据
    vec2[0] = "Rust";

    // Vec 和 String一样，数据存放在堆上

    println!("{}", vec2.len()); // 1
    println!("{}", vec2.capacity()); // 4
    println!("{:?}", vec2.as_ptr()); // 0x7fafc9f05b70
    println!("{}", vec2[0]);

    // let mut vec3 = vec![];
    // vec3.push("ve3");

    // 2 HashMap

    // HashMap并不是预导入的，需要手动引入当前作用域
    use std::collections::HashMap;

    // 使用new方法创建
    let mut scores = HashMap::new();

    // 插入数据
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".into(), 10);
    scores.insert(String::from("Yellow"), 50);

    // 修改数据,修改和插入数据是同一个api
    scores.insert(String::from("Blue"), 100);

    // 访问数据,注意访问的key传递的是引用
    let key = String::from("Blue");
    println!("{:?}", scores[&key]);

    // 1 结构体
    // Rust中的结构体有三种

    // 1.1 常规结构体
    struct Language {
        name: String,
        birth: u32,
        is_popular: bool,
    }

    // 1.2 元组结构体
    struct Rust(String);

    // 1.3 单元结构体
    struct Go;

    // 2 为结构体实现方法
    impl Rust {
        // Self 代表结构体本身
        fn new() -> Self {
            Rust(String::from("Rust"))
        }

        fn print(&self) {
            println!("{:?}", self.0);
        }
    }

    // 3 方法调用
    let r = Rust::new();

    r.print();

    Rust::print(&r);
    Rust::print(&r);

    // 4 访问结构体成员
    println!("{:?}", r.0);

    //枚举

    // 枚举在形式上和结构体较为相似
    enum Subject {
        Math,
        Chinese,
        English(String),
    }

    // 初始化

    let subject = Subject::English(String::from("English"));

    //标准库中两个比较重要的枚举 Option和 Result

    // Result 用于一些处操作可能遇到错误的场景，比如打开文件时，如果成功，返回文件，遇到错误时返回一个Error
    use std::fs::File;

    let file: Result<File, std::io::Error> = File::open("tmp.txt");

    // Option 用于一些值可能为空的情况
    // 如尝试获取哈希表中某个key所对应的value，当值存在时，返回值，当不存在时返回None

    let map: HashMap<&str, u32> = HashMap::new();
    let v: Option<&u32> = map.get("rust");

    // 函数

    // 1 函数定义
    // 1.1 没有参数和返回值的函数
    fn foo() {
        println!("foo")
    }

    // 1.2 有参数和返回值的函数

    fn bar(s: &str) -> String {
        String::from(s)
    }

    // 1.3 参数类型必须显式声明，比如引用或者可变性

    fn foobar(mut s: &str) -> &str {
        s = "rust";
        s
    }

    // 2 函数调用

    foo();
    bar("Rust");
    let fr = foobar("go");
    println!("{}", fr);

    // 3 函数作为参数

    fn af(f: fn() -> u32) -> u32 {
        let value = f();

        value
    }

    fn bf() -> u32 {
        42
    }

    // 把函数作为参传给另一个函数

    af(bf);

    // 闭包
    // 1 闭包定义

    // 闭包可以捕获环境变量,并且根据其对环境变量的操作可以分为以下三类

    let c1 = || println!("未捕获环境变量");

    let v = "rust";
    let c2 = || println!("捕获环境变量但不修改 {}", v);

    let mut s0 = String::from("hello");

    // 闭包的参数写在 ｜｜ 中

    let mut c3 = |s: String| {
        s0 = s + v;
        println!("捕获并修改环境变量 {:?}", s0)
    };

    // 2 闭包的调用

    // 闭包的调用同函数一样

    c1();
    c2();
    c3(String::from("rust"));

    // 1 泛型参数的表示

    // 泛型参数一般用大写字母`T`表示,多个泛型参数可以使用多个大写字母

    // 学习泛型时可以把泛型当作自定义类型，它必须先声明才能使用

    // 2 泛型如何使用

    // 2.1 集合 Vec<T>
    // 集合vector就是由泛型提供支持的,它允许我们使用某个具体类型时再指定

    let v1: Vec<u8> = Vec::new();
    let v2: Vec<String> = Vec::new();
    let v3: Vec<bool> = Vec::new();

    // 2.2 泛型结构体

    // 可以声明一个泛型结构体，然后再使用的时候在指定成员的具体类型
    // 注意：必须先在` <> `中声明泛型参数，然后才能使用

    struct Type<T>(T);
    struct Point<A, B> {
        a: A,
        b: B,
    }

    let t1 = Type(42);
    let t2 = Type("rust");

    let p1 = Point { a: 42, b: 42 };
    let p2 = Point { a: 42.1, b: 42.1 };

    // 为泛型结构体实现方法
    // 注意：为泛型结构体实现方法时，impl和结构体后面的泛型声明要保持一致
    impl<A, B> Point<A, B> {
        fn new(a: A, b: B) -> Self {
            Point { a, b }
        }
    }

    // 2.3 泛型枚举

    // 同样，可以定义泛型枚举

    enum Area<A, B, C> {
        Rectangle(A),
        Square(B),
        Circle(C),
    }

    let a1: Area<f64, u32, &str> = Area::Rectangle(42f64);
    let a2: Area<f32, u64, &str> = Area::Square(42u64);
    let a3: Area<f64, u32, &str> = Area::Circle("100 cm^2");

    // 2.4 泛型函数

    // 函数参数也可以是泛型, 当然泛型也需要在 `<>` 中先声明

    fn generics<T, B>(a: T, b: B) -> T {
        a
    }
    generics(32, "rust");
    generics("rust", 32);

    // Rust 中的模式匹配指的是结构上的匹配，最常用有 match、while let 、let 、if let

    // 1 match
    // match 是最长用的模式匹配，主要和枚举搭配使用，以匹配不同的枚举成员

    match std::fs::File::open("rust.txtr") {
        Ok(file) => println!("{:?}", file),
        Err(err) => println!("{}", err),
    }

    // 2 if let
    // if let 可以让我们只关注我们想要的结果

    if let Ok(file) = std::fs::File::open("rust.txtr") {
        println!("{:?}", file);
    }

    // 3 while let
    // 和 if let 类似，只处理正确的结果

    while let Ok(file) = std::fs::File::open("rust.txt") {
        println!("{:?}", file);
    }

    // 4 let
    // let 本身也是一种模式匹配
    // 使用 let 匹配元组中的元素

    let tuple = (42, true, "rust");

    let (x, y, z) = tuple;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
