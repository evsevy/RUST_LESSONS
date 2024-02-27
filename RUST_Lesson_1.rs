
//*******************************RUST_SYNTAX************************************//

fn main() {
    let a: i32 = 5;
    let b: f64 = 3.14;
    let c: char = 'a';
    let d: bool = true;
    let e: &str = "Hello, Rust!";
    
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
}

fn main() {
    // Объявление переменных
    let integer_number: i32 = 10;
    let float_number: f64 = 3.14;
    let character: char = 'A';
    let boolean: bool = true;
    
    // Объявление массива
    let numbers: [i32; 3] = [1, 2, 3];
    
    // Индексирование массива
    println!("First element of the array: {}", numbers[0]);
    println!("Second element of the array: {}", numbers[1]);
    println!("Third element of the array: {}", numbers[2]);
}

fn main() {
    let a: u8 = 15;
    let b: u8 = 7;

    // bitwise AND
    let result_and = a & b;
    println!("Bitwise AND: {}", result_and);

    // bitwise OR
    let result_or = a | b;
    println!("Bitwise OR: {}", result_or);

    // bitwise XOR
    let result_xor = a ^ b;
    println!("Bitwise XOR: {}", result_xor);

    // bitwise left shift
    let result_left_shift = a << 2;
    println!("Bitwise Left Shift: {}", result_left_shift);

    // bitwise right shift
    let result_right_shift = a >> 2;
    println!("Bitwise Right Shift: {}", result_right_shift);
}


// В Rust есть несколько разрядов типов данных, которые могут быть использованы в программировании:

1. Примитивные типы данных:
   - Целочисленные типы данных: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
   - Вещественные типы данных: `f32`, `f64`
   - Логический тип данных: `bool`
   - Символьный тип данных: `char`

2. Структурированные типы данных:
   - Кортежи: `()` и `(T1, T2, ...)`
   - Массивы: `[T; N]`
   - Срезы: `&[T]` и `&mut [T]`

3. Указатели и смарт-указатели:
   - Ссылки: `&T`
   - Мутабельные ссылки: `&mut T`
   - Умные указатели: `Box<T>`, `Rc<T>`, `Arc<T>`

4. Структуры и перечисления:
   - Структуры: `struct`
   - Перечисления: `enum`

5. Функции и замыкания:
   - Функции с определенной сигнатурой: `fn`
   - Замыкания: `|param1, param2, ...| { /* implementation */ }`

6. Типы данных для работы с потоками:
   - Типы данных для параллельных вычислений: `Thread`, `Channel`, `Mutex`, `Barrier`, `Arc`, `Atomic<Type>`

7. Другие специализированные типы данных:
   - Строки: `String`, `&str`
   - Векторы: `Vec<T>`
   - Хэш-таблицы: `HashMap<K, V>`, `HashSet<T>`

//Это основные разряды типов данных в Rust, которые помогут вам создавать разнообразные структуры данных и определять их поведение в вашей программе.

1. Целочисленные типы данных:

let a: i8 = 10;
let b: i16 = 10000;
let c: i32 = 1000000000;
let d: i64 = 1000000000000;
let e: i128


2. Вещественные типы данных:

let x: f32 = 3.14;
let y: f64 = 3.14159265359;


3. Булевый тип данных:

let is_true: bool = true;
let is_false: bool = false;


4. Строковый тип данных:

let hello: &str = "Hello, world!";


5. Составные типы данных:

struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
/*
Битовая арифметика в Rust предоставляет возможность работать с битами напрямую, 
без использования стандартных операций над числами. В Rust для работы с битами используются операторы &, |, ^, <<, >> и !.
*/

//Примеры использования битовых операторов в Rust:

//1. Побитовое И (AND):

let result = 0b1010 & 0b1100; // результат: 0b1000


//2. Побитовое ИЛИ (OR):


let result = 0b1010 | 0b1100; // результат: 0b1110


//3. Побитовое исключающее ИЛИ (XOR):


let result = 0b1010 ^ 0b1100; // результат: 0b0110


//4. Побитовый сдвиг влево:


let result = 0b1010 << 2; // результат: 0b0100


//5. Побитовый сдвиг вправо:


let result = 0b1010 >> 1; // результат: 0b0101


//6. Побитовая инверсия (NOT):


let result = !0b1010; // результат: 0b0101

/*
Битовая арифметика в Rust может быть полезна при работе с масками, флагами, 
битовыми полями структур и другими подобными задачами. 
Но следует быть осторожным при использовании битовых операторов, 
так как неправильное использование может привести к ошибкам и неочевидным результатам.
*/
// БИТОВЫЕ ВЫРАЖЕНИЯ:
1. Побитовое И (AND):

fn main() {
    let a: u8 = 5; // 00000101
    let b: u8 = 3; // 00000011
    
    let result = a & b; // 00000001

    println!("Result of bitwise AND operation: {}", result);
}

2. Побитовое ИЛИ (OR):

fn main() {
    let a: u8 = 5; // 00000101
    let b: u8 = 3; // 00000011
    
    let result = a | b; // 00000111

    println!("Result of bitwise OR operation: {}", result);
}

3. Побитовое исключающее ИЛИ (XOR):

fn main() {
    let a: u8 = 5; // 00000101
    let b: u8 = 3; // 00000011
    
    let result = a ^ b; // 00000110

    println!("Result of bitwise XOR operation: {}", result);
}


4. Побитовый сдвиг влево:

fn main() {
    let a: u8 = 5; // 00000101
    
    let result = a << 2; // 00010100

    println!("Result of bitwise left shift operation: {}", result);
}


5. Побитовый сдвиг вправо:

fn main() {
    let a: u8 = 5; // 00000101
    
    let result = a >> 2; // 00000001

    println!("Result of bitwise right shift operation: {}", result);
}

fn main() {
    let num1: i128 = 9223372036854775807; // maximum value for i128
    let num2: i128 = -9223372036854775808; // minimum value for i128

    println!("num1: {}", num1);
    println!("num2: {}", num2);
}

//1. Управление памятью с типами данных:

fn main() {
    let x: i32 = 5; // объявление переменной x типа i32
    let mut y: i32 = 10; // объявление изменяемой переменной y типа i32
    y = x; // присвоение значения переменной x переменной y
    println!("y = {}", y); // вывод значения переменной y
}

//2. Битовые операции:

fn main() {
    let a: u8 = 0b1010; // создание переменной a со значением 1010 в двоичном формате
    let b: u8 = 0b0011; // создание переменной b со значением 0011 в двоичном формате
    
    let result_and = a & b; // побитовое И
    let result_or = a | b; // побитовое ИЛИ
    let result_xor = a ^ b; // побитовое исключающее ИЛИ
    let result_not = !a; // побитовое НЕ
    
    println!("a & b = {:08b}", result_and); // вывод результата побитового И
    println!("a | b = {:08b}", result_or); // вывод результата побитового ИЛИ
    println!("a ^ b = {:08b}", result_xor); // вывод результата побитового исключающего ИЛИ
    println!("!a = {:08b}", result_not); // вывод результата побитового НЕ
}

//Управление памятью в языке программирования Rust осуществляется с помощью системы владения. 

//1. Владение и перемещение объекта:


fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 больше не действителен, т.к. его значение было перемещено в s2

    println!("{}", s2); // Печатает "hello"
    println!("{}", s1); // Ошибка компиляции: значение s1 уже не является действительным
}

//2. Клонирование объекта:

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Создает копию s1

    println!("{}", s1); // Печатает "hello"
    println!("{}", s2); // Печатает "hello"
}

//3. Владение с использованием ссылок:

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Передается ссылка на s1

    println!("Length of '{}' is {}", s1, len);
}


//Функция - это блок кода, который выполняет определенную задачу.
//В языке программирования Rust функции объявляются с использованием ключевого слова "fn". 

//1. Простая функция, которая выводит приветствие на экран:


fn say_hello() {
    println!("Hello, Rust!");
}

fn main() {
    say_hello();
}

//2. Функция с параметрами, которая складывает два числа и возвращает результат:


fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add_numbers(5, 10);
    println!("Sum: {}", result);
}

//3. Функция с возвращаемым значением и обработкой ошибки:


use std::fs::File;
use std::io::Error;

fn open_file(file_name: &str) -> Result<File, Error> {
    let file = File::open(file_name);
    file
}

fn main() {
    let result = open_file("example.txt");
    
    match result {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Failed to open file: {}", error),
    }
}

//Rust предоставляет богатый набор средств для работы с функциями, таких как передача параметров, возвращаемые значения, обработка ошибок и многое другое.

////////////////////////////////////ЦИКЛЫ//////////////////////////////////////////////////
//В Rust также есть циклы `while` и `for`, которые используются для повторения выполнения определенного блока кода. 

//Пример использования цикла `while`:

fn main() {
    let mut x = 5;
    
    while x > 0 {
        println!("x is now: {}", x);
        x -= 1;
    }
}

//Этот код будет выводить значения переменной `x` от 5 до 1.

Пример использования цикла `for`:

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}

//Этот код будет выводить все элементы массива `numbers`.

//Циклы `while` и `for` в Rust также поддерживают различные управляющие конструкции, такие как `break`, `continue` и `return`, которые позволяют изменять ход выполнения цикла.

//Код, использующий цикл `while` с управляющими конструкциями:

fn main() {
    let mut i = 0;

    while i < 5 {
        if i == 2 {
            i += 1;
            continue;
        }

        println!("{}", i);

        if i == 3 {
            break;
        }

        i += 1;
    }
}

//Код, использующий цикл `for` с управляющими конструкциями:

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers {
        if num == 2 {
            continue;
        }

        println!("{}", num);

        if num == 4 {
            break;
        }
    }
}

//Оба примера кода демонстрируют использование управляющих конструкций `break` и `continue` в циклах `while` и `for` в Rust.