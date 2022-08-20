fn main() {
    /*
        ### Règle de nommage des variables :
            - Uniquement des lettres, des chiffres et "_"
            - Commence obligatoirement par une lettre ou un "_"
            - Les noms sont sensibles à la casse
            - Il ne faut pas utiliser des noms réservés par le langqage Rust (ex : let, mut, if, while, etc ...)

            let -> déclare une variable immutable (qui ne peut pas être modifiée après affectation)
            let mut -> déclare une variable mutable (qui peut être modifiée après affectation)

        SHADOWING =
            La variable exite encore
            let mut age = 34;
            age = age + 1;

            La variable n'exite plus et la valeur est la valeur de la dernière affectation
            let age = 20;
            let age = age + 1;
    */

    // Déclaration d'une variable
    let mut age = 34;
    println!("age = {}", age);

    age = age + 1;

    // Affichage de la variable
    println!("age = {}", age);
}
