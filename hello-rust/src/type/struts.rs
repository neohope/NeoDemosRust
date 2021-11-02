// 结构体

#[derive(Debug)]
struct Marker;

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main(){
    let marker = Marker {};

    let color = Color(128, 128, 128);

    let person = Person {
        name: "Tyr".into(),
        age: 18,
    };

    println!(
        "marker: {:?}, color: {:?}, person: {:?}",
        marker, color, person
    );
}