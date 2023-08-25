use regex::Regex;

struct Name
{
    first: String,
    last:  String,
}

fn main() {
   let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
   println!("Regex grabbed: {}", re);
   println!("Did our date match? {}", re.is_match("2023-08-22"));

    
    let student1 = Name(first:string::From("Max"),last:string::From( "Mike"));

    println!("Person 1 {}", student1);
}
