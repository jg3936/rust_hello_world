fn main() {
    try_variables();
    solo_heap_ownership();
}

fn solo_heap_ownership() {
    let s1 = String::from("first");
    println!("{}",s1);
    let s2 = s1;
    //println!("{}",s1); //compilation error --> single ownership.
    println!("{}",s2);//cannot use s1 here, it has gone out of scope since only single ownership is allowed.

}

fn try_variables() {
    let total_vacay_days = 20;
    let mut days_used = 15;
    println!("Hello, you have used {} out of {} days of vacation", days_used, total_vacay_days);

    println!("You have {} days left", total_vacay_days - days_used);

    let mut days_left = total_vacay_days - days_used;

    println!("You have {} days left", days_left);

    days_used+=1;
    days_left-=1;

    println!("Now you have used {} days", days_used);

    println!("You have {} days left", days_left);
}
