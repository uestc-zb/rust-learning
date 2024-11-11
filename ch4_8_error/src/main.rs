// // // // // fn main() {
// // // // //     let s1 = Some("some1");
// // // // //     let s2 = Some("some2");
// // // // //     let n: Option<&str> = None;
  
// // // // //     let o1: Result<&str, &str> = Ok("ok1");
// // // // //     let o2: Result<&str, &str> = Ok("ok2");
// // // // //     let e1: Result<&str, &str> = Err("error1");
// // // // //     let e2: Result<&str, &str> = Err("error2");
  
// // // // //     assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
// // // // //     assert_eq!(s1.or(n), s1);  // Some or None = Some
// // // // //     assert_eq!(n.or(s1), s1);  // None or Some = Some
// // // // //     assert_eq!(n.or(n), n);    // None1 or None2 = None2
  
// // // // //     assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
// // // // //     assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
// // // // //     assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
// // // // //     assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2
  
// // // // //     assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
// // // // //     assert_eq!(s1.and(n), n);   // Some and None = None
// // // // //     assert_eq!(n.and(s1), n);   // None and Some = None
// // // // //     assert_eq!(n.and(n), n);    // None1 and None2 = None1
  
// // // // //     assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
// // // // //     assert_eq!(o1.and(e1), e1); // Ok and Err = Err
// // // // //     assert_eq!(e1.and(o1), e1); // Err and Ok = Err
// // // // //     assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
// // // // //   }

// // // // fn main() {
// // // //     // or_else with Option
// // // //     let s1 = Some("some1");
// // // //     let s2 = Some("some2");
// // // //     let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

// // // //     let n: Option<&str> = None;
// // // //     let fn_none = || None;

// // // //     assert_eq!(s1.or_else(fn_some), s1);  // Some1 or_else Some2 = Some1
// // // //     assert_eq!(s1.or_else(fn_none), s1);  // Some or_else None = Some
// // // //     assert_eq!(n.or_else(fn_some), s2);   // None or_else Some = Some
// // // //     assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

// // // //     // or_else with Result
// // // //     let o1: Result<&str, &str> = Ok("ok1");
// // // //     let o2: Result<&str, &str> = Ok("ok2");
// // // //     let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

// // // //     let e1: Result<&str, &str> = Err("error1");
// // // //     let e2: Result<&str, &str> = Err("error2");
// // // //     let fn_err = |_| Err("error2");

// // // //     assert_eq!(o1.or_else(fn_ok), o1);  // Ok1 or_else Ok2 = Ok1
// // // //     assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
// // // //     assert_eq!(e1.or_else(fn_ok), o2);  // Err or_else Ok = Ok
// // // //     assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
// // // // }

// // // fn main() {
// // //     // and_then with Option
// // //     let s1 = Some("some1");
// // //     let s2 = Some("some2");
// // //     let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

// // //     let n: Option<&str> = None;
// // //     let fn_none = |_| None;

// // //     assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
// // //     assert_eq!(s1.and_then(fn_none), n);  // Some and_then None = None
// // //     assert_eq!(n.and_then(fn_some), n);   // None and_then Some = None
// // //     assert_eq!(n.and_then(fn_none), n);   // None1 and_then None2 = None1

// // //     // and_then with Result
// // //     let o1: Result<&str, &str> = Ok("ok1");
// // //     let o2: Result<&str, &str> = Ok("ok2");
// // //     let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

// // //     let e1: Result<&str, &str> = Err("error1");
// // //     let e2: Result<&str, &str> = Err("error2");
// // //     let fn_err = |_| Err("error2");

// // //     assert_eq!(o1.and_then(fn_ok), o2);  // Ok1 and_then Ok2 = Ok2
// // //     assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
// // //     assert_eq!(e1.and_then(fn_ok), e1);  // Err and_then Ok = Err
// // //     assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
// // // }
// // fn main() {
// //     let s1 = Some(3);
// //     let s2 = Some(6);
// //     let n = None;

// //     let fn_is_even = |x: &i8| x % 2 == 0;

// //     assert_eq!(s1.filter(fn_is_even), n);  // Some(3) -> 3 is not even -> None
// //     assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
// //     assert_eq!(n.filter(fn_is_even), n);   // None -> no value -> None
// // }

// fn main() {
//     let o1: Result<&str, &str> = Ok("abcde");
//     let o2: Result<&str, isize> = Ok("abcde");

//     let e1: Result<&str, &str> = Err("404");
//     let e2: Result<&str, isize> = Err(404);

//     let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

//     assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
//     assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
// }

fn main() {
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
}