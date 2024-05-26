fn main() {
    let age: i32 = 21; //(nicht mutable) setzen den Variablen "age" mit dem Wert "21" und dem Typ "int32"
    println!("My age is {}", age); // wird alles, was in geschweiften Klammern steht, als eine Variable ausgegeben, die ginter dem Komma steht
    println!("{}", age); // nur die "age" drucken (21)

    let mut name: &str = "Andrii";
    println!("{}", name);
    name = "Andr";
    println!("{}", name);

}
