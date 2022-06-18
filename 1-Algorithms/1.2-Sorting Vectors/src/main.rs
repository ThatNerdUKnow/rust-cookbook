#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    println!("Sort a Vector of Integers");

    let mut vec = vec![1, 5, 10, 2, 15];

    println!("{:?}", vec);
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec);

    println!("Sort a vector of floats");

    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    println!("Sort a Vector of Structs");

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    println!("{:?}", people);
    people.sort();
    println!("{:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25)
        ]
    );

    // Sort people by age

    people.sort_by(|a,b|b.age.cmp(&a.age));
    println!("{:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(),60),
            Person::new("Zoe".to_string(),25),
            Person::new("John".to_string(),1)
        ]
    );
}
