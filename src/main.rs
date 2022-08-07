use testing_rs::coucou;

fn main() {
    let hello = coucou();
    println!("Hello, world: {}!", hello);
}


#[test]
fn test_coucou() {
    assert_eq!(coucou(), "coucou les tests");
}
