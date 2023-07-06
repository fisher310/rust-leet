#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

#[test]
fn test_name() {
    let bag = Bag { food: Food::Cake };
    match bag.food {
        Food::Cake => println!("I got cake."),
        ref a => println!("I got {:?}", a),
    }

    println!("{:?}", bag);
}
