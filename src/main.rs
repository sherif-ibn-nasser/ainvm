fn print(s:&String){


    println!("{}",s);
    
}

fn change(s:&mut String){

    *s=String::from("value");

}

fn main() {
    let mut s: String=String::from("this is string");
    let mut s1: String=s;

    change(&mut s1);

    println!("{}",s1);


    println!("Hello, world!");
}
