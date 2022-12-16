fn main() {
 try_variables();

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