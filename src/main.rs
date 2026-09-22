
// небезопасная функция
unsafe fn dangeres_operation(v: *mut i32) {
    *v += 1 ;
}

// изменяемая статическая переменная
static mut COUNTER: i32 = 1 ;

unsafe trait Dangeres {
    fn do_it(&self) ;
}

struct MyStruct ;

// реализация unsafe trait
unsafe impl Dangeres for MyStruct {
    fn do_it(&self) {
        println!("Doing something dangerous!") ;
    }
}

// Определение union
// Поле должно реализовывать `Copy` или быть обернуто в `ManuallyDrop<...>`, чтобы его 
// можно было использовать в объединении (`union`).
union MyUnion {
    i:  i32,
    f:  f32,
}

fn main() {

    // 1) разименовывание сырого указателя

    let mut m = 1 ;
    // получение из &mut i32 -> *mut i32 (изменяемая ссылка -> сырой изменяемый указатель)
    //let rp_m = &mut m as *mut i32;
    // или  так:
    let rp_m = &mut m as *mut _ ;   // автоматическое выведение типа

    unsafe {
        *rp_m += 1 ;    // изменить значение m через *mut i32
    }

    // Прочитать изменённое значение из m
    println!("{}", m) ; // Out: 2

    println!("-------------------------------------\n") ;

    // 2) Вызов небезопасной функции

    unsafe {
        dangeres_operation(rp_m);
    }

    println!("{}", m) ; // Out: 3

    println!("-------------------------------------\n") ;

    // 3) Доступ или изменение к mutable static variable

    unsafe  {
        COUNTER += 1 ;
    }

    println!("{}", 
        unsafe {
            COUNTER
        }
    ) ; // Out: 2

    println!("-------------------------------------\n") ;

    // 4) Реализация unsafe trait

    let my_s = MyStruct ;

    my_s.do_it();   // Out: Doing something dangerous!

    println!("-------------------------------------\n") ;

    // 5) Обращение к union

    let mut u = MyUnion{i: 1} ;
    unsafe {
        println!("i: {}", u.i) ;    // i: 1
        u.f = 10.2 ;
        println!("f: {}", u.f) ;    // f: 10.2
    }

}
