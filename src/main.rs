extern crate core;

use std::collections::HashMap;
use crate::checked::{MathError, MathResult};

// use core::num::fmt::Part::Copy;

// 19.标准库类型
fn main() {
    //region 19.1.箱子、栈和堆
    println!("\n\n*****19.1.箱子、栈和堆*****");
    // 在Rust中，所有值默认都是栈分配的。
    // 通过创建 Box<T> ，可以把值 装箱(boxed) 来使它在堆上分配。
    // 箱子 (box，即 Box<T> 类型的实例）是一个智能指针，指向堆分配的 T 类型的值。
    // 当箱子离开作用域时，它的析构函数会被调用，内部的对象会被销毁，堆上分配的内存也会被释放。
    // 被装箱的值可以作用 * 运算符进行解引用，这会移除一层装箱。
    // 栈分配的变量
    use std::mem;
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug)]
    struct Point{
        x: f64,
        y: f64,
    }
    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn boxed_origin() -> Box<Point> {
        // 在堆上分配这个点，并返回一个指向它的指针
        // Box::new(Point { x:0.0, y: 0.0})
        Box::new(
            Point { x: 0.0, y: 0.0 }
        )
    }
    let point: Point = origin();
    // 栈分配
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };
    // 堆分配
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });
    // 函数的输出可以装箱
    let boxed_point: Box<Point> = Box::new(origin());
    // 两层装箱
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack", mem::size_of_val(&rectangle));
    // box 的宽度就是指针的宽度，什么意思？ 2023年2月4日20时9分57秒
    // 确切地说，是 box 占用的内存，就是指针的大小，以下这几项输入的内存占用均是8个字节！！！
    println!("Boxed point occupies {} bytes in the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&box_in_a_box));
    // 将包含在 'boxed_point' 中的数据复制到 'unboxed_point'
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_point));
    //endregion

    //region 19.2.动态数组
    println!("\n\n*****=====19.2.动态数组=====*****");
    // vector 是大小可变的数组。
    // 和 slice（切片） 类似，它们的大小在编译时是未知的，可以随时扩大或缩小
    // 一个 vector 用3个词来表示：一个指向数据的指针、vector的、还有其容量
    // 此容量指明要为这个 vector 保留多少内存
    // vector 的长度只要小于该容量，就可以随意增长；当需要超过这个阈值时，会
    // 给 vector 重新分配一段更大的容量。

    // 迭代器可以被收集到 vector 之中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
    // vec! 宏可用来初始化一个 vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 在 vector 的尾部插入一个新元素
    println!("Push 4 into the xs");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // 报错，不可变的 vector 不可增长
    // 代码编辑的时候，IDE 就已经报错了！！！ 2023年2月4日20时25分21秒
    // collected_iterator.push(0);

    // len 方法获取一个 vector 的当前大小
    println!("Vector size: {}", xs.len());

    // 下标使用中括号表示（从 0 开始）
    println!("Second element: {}", xs[1]);

    // pop 移除 vector 最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());

    // 超出下标范围将抛出一个 panic
    // println!("Fourth element: {:?}", xs[3]);

    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i,x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
    // 多亏了 iter_mut ，可变的 vector 在迭代的，其中每个值都能被修改！
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updateed vector: {:?}", xs);
    //endregion

    //region 19.3.字符串
    println!("\n\n*****=====19.3.字符串=====*****");
    // Rust 中有两种字符串类型： String 和 &str。
    // String 被存储为由字节组成的 vector(Vec<u8>)，但保证其一定是一个有效的 UTF-8 序列。
    // String 是堆分配的，可增长的，且不是零结尾的(null terminated)。
    // &str 是一个总是指向有效 UTF-8 序列的切片（&[u8]），并可用来查看 String 的内容
    // 就如同 &[T] 是 Vec[T]的全部或部分引用。
    // 一个对吟诗内存中分配的字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并未分配新字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符到一个 vector ，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("chars的内容是: {:?}", chars);

    // 创建一个空的且可增长的 String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
    //region

    //region 19.3.字面量与转义字符
    println!("\n\n*****=====19.3.字面量与转义字符=====*****");
    // 书写含有特殊字符的字符串字面量有很多种方法，都会产生类似的 &str ，所以最好选择最方便的
    // 写法。类似地，字节串(byte string) 字面量也有多种写法，它们都会产生 &[u8;N] 类型。
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means?) {}", byte_escape);
    // 也可以用 Unicode 码位表示。
    let unicode_codepoint = "\u{2110}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+2110) is called {}", unicode_codepoint, character_name);
    let long_string = "String literals \
        can span multiple lines. \
        The linebreak and indentation here are ->\
        <- can be escaped too!";
    println!("{}", long_string);
    // 如果需要大量转义字符，可以直接使用原始字符串(raw string)。
      let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
    // 可使用的 # 的数目没有限制。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 想要非UTF-8字符串（注意，&str 和 String 都必须是合法的 UTF-8 序列），或者需要一个字节数组，
    // 其中大部分是文本，请使用 字节串
    use std::str;
    // 注意这并不是一个 &str
    let bytestring: &[u8; 20] = b"this is a bytestring";
    // 字节串没有实现 Display
    println!("A bytestring is: {:?}", bytestring);
    // 字节串可以使用单字节的转义字符...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 把字节串转换为 &str 可能失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"

    // 但这样的话它们就无法转换成 &str 了
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
    //endregion

    //region 19.4.选项Option
    println!("\n\n*****=====19.4.选项Option=====*****");
    // 有时候想要捕捉到程序某部分的失败信息，而不是调用 panic!，可用 Option 枚举类型来实现。
    // Option<T>，有两个变量：
    //      None：表里失败或缺少值
    //      Some(value)：元组结构体，封装了一个 T 类型的值 value
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }
    fn try_division(dividend: i32, divisor: i32) {
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed!", dividend, divisor),
            Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
        }
    }
    try_division(4, 2);
    try_division(1, 0);
    // 绑定 None 到一个变量需要类型标注
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);
    // 解包 `Some` 将取出被包装的值
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
    // 解包 `None` 将会引发 'panic!'
    // println!("{:?} unwraps to {:?}", none, none.unwrap());
    //endregion

    //region 19.5.选项Result
    println!("\n\n*****=====19.5.选项Result=====*****");
    // 我们已经看到 Option 枚举类型可以用途可能失败的函数的返回值 ，其中返回 None 可以表明失败。
    // 但是有时要强调 为什么 一个操作会失败，这做到这一点，提供了 Result 枚举类型。
    // Result<T, E> 类型拥有两个取值：
    //      Ok(value)：表示成功，并包装操作返回的 value(value拥有 T 类型)
    //      Err（why）：表示失败，并包装 why,它能够解释失败的原因( whyr拥有Ｅ　类型）
    // op(x, y) == sqrt(ln(x / y))
    // fn op(x:f64, y:f64) -> f64 {
    //     match checked::div(x, y) {
    //         Err(why) => panic!("{:?}", why),
    //         Ok(radio) => match checked::ln(radio) {
    //             Err(why) => panic!("{:?}", why),
    //             Ok(ln) => match checked::sqrt(ln) {
    //                 Err(why) => panic!("{:?}", why),
    //                 Ok(sqrt) => sqrt,
    //             }
    //         }
    //     }
    // }
    // println!("{}", op(1.0, 10.0));
    //endregion

    //region 19.5.？运算符
    println!("\n\n*****=====19.5.？运算符=====*****");
    // 把 result 用 match 连接起来会显得很难看；幸运的是，？运算符可以把这种逻辑变得干净漂亮
    // ？运算符用在返回值为 Result 的表达式后面，它等同于这样一个匹配表达式：其中 Err(err)
    // 分支展开成提前返回的 return Err(err)，而 Ok(ok) 分支展开成 ok 表达式。

    // panic! 即中止了当前程序的运行
    // checked_v1::op(1.0, 10.0);
    //endregion

    //region 19.6.panic!
    println!("\n\n*****=====19.6.panic!=====*****");
    // panic! 宏可用于产生一个 panic,并开始回退(unwind)它的栈。
    // 在回退栈的同时，运行时将会释放该 线程所拥有的所有资源，这是通过调用线程中所有对象的析构函数
    // 实现的。因为我们正在处理的程序只有一个线程，panic! 将会引发程序报告 panic! 消息并退出。
    fn division_int(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("division by zero");
        } else {
            dividend / divisor
        }
    }
    let _x = Box::new(0i32);
    // division_int(3, 0);
    println!("This point won't be reached");

    //region 19.7.散列表HashMap
    println!("\n\n*****=====19.7.散列表HashMap=====*****");
    // vector 通过整形下标来存储值，而 HasMap（散列表）通过键（key）来存储值。
    // HashMap 的键可以是布尔型、整形、字符串，或任意实现了 Eq 和 Hask trait
    // 的其他类型。
    // 和 vector 类似，HashMap 也是可增长的，但 HashMap 在占据了多余空间时还
    // 可以缩小自己。可以使用 HashMap::with_capacity(unit) 创建具有一定初始
    // 容量的 HashMap，也可以使用 HashMap::new() 来获得一个带有默认初始容量的
    // HashMap。
    use std::collections::HashMap;
    fn call(number: &str) -> &str {
        match number {
            "798-1364" => "We're sorry, the call cannot be completed\
            Please hang up and try again.",
            "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name\
            is Fred. What can I get for you today?",
            _ => "Hi, Who is this again?"
        }
    }
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 接受一个引用并返回 Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }





















}
mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }
    pub type MathResult = Result<f64, MathError>;
    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }
}
mod checked_v1 {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }
    pub type MathResult = Result<f64, MathError>;
    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 div 失败了，那么返回 DivisionByZero
        let ratio = div(x, y)?;
        // 如果 ln 失败了，那么返回 NegativeLogarithm
        let ln = ln(ratio)?;
        sqrt(ln)
    }
    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NegativeLogarithm => "logarithm of negative number",
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}