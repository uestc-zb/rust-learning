fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}",THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    // let spaces = "  ";
    // let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("{} {}",x,y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    println!("{} {} {} {} {} {}",sum, difference, product, quotient, floored, remainder);

    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '🔛';
    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let months = ["January", "Feb", "Mar", "Apr", "May", "June", "July", "Aug",
                                // "Sep", "Oct", "Nov", "Dec"];
    // let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);
    another_function(5);
    print_lab(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("{}",y);
    let x = five();
    println!("The value of x is: {}", x);

    let number = 5;
    if number < 5{
        println!("condition was true");
    } else if number == 5 {
        println!("condition was 5");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}",number);

    loop{
        println!("again!");
        break;
    }

    let mut count = 0;
    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop{
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}",count);

    let mut counter = 0;
    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!",number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("{}", number);
    }
    println!("LIFEOFF!!!!")
}

fn another_function(x: i32){
    println!("Another function {}", x);
}

fn print_lab(value: i32, unit_label: char){
    println!("The measurement is {} {}",value, unit_label);
}

fn five() -> i32{
    5
}