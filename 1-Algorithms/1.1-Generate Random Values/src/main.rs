use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Standard};

#[derive(Debug)]
struct Point {
    _x: i32,
    _y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            _x: rand_x,
            _y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

  
    println!("Generate Random Numbers");

    println!("Random u8:    {}", n1);
    println!("Random u16:   {}", n2);
    println!("Random u32:   {}", rng.gen::<u32>());
    println!("Random i32:   {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    println!("Generate random numbers within a range");

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float:    {}", rng.gen_range(0.0..10.0));

    println!("Generate using uniform distribution");

    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die:    {}", throw);
        if throw == 6 {
            break;
        }
    }

    let normal: Normal<f64> = <Normal<f64>>::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2,9) distribution", v);

    println!("Generate random values of a custom type");

    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    println!("Create random passwords from a set of alphanumeric characters");

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

    println!("Create random passwords from a set of user-defined characters");

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
