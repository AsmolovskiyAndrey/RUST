fn main() {
    let age: i32 = 21; //(nicht mutable) setzen den Variablen "age" mit dem Wert "21" und dem Typ "int32"
    println!("My age is {}", age); // wird alles, was in geschweiften Klammern steht, als eine Variable ausgegeben, die ginter dem Komma steht
    println!("{}", age); // nur die "age" drucken (21)

    // let mut name: &str = "Andrii"; // ein möglicher, aber unsicherer Weg, eine Zeichenkette zuzuweisen
    let mut name: String = String::from("Andrii"); // ein gutes Praxis, eine String-Variable zuzuweisen
    println!("{}", name);
    name = String::from("ANDR"); // Methode 1: Verwendung der String::from
    println!("{}", name);
    name = "And".to_string(); // Methode 2: Verwendung der Methode to_string
    println!("{}", name);

}
