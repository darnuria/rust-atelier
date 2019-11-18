// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Animateur: Axel Viala (darnuria)
//
// nombres.rs
//
// Dans cet exercice vous allez réaliser un convertiseur de degrés Fahrenheit
// vers les degrés Celsius.
//
// Objectifs pédagogiques:
//
// - Manipulation des nombres
// - Approfondissement variables
// - Découverte du formattage
// - Concept d'expression
// - Opérations entre types
//
// Bonus écrire une fonction pour assurer la conversion
// - Découverte des Fonctions
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------


/*
 * Notre programme va réaliser une conversion d'une temperature en degrés
 * Fahrenheit vers les degrés Celsius.
 *
 * Pour convertir entre ces deux unités on peut utiliser l'équivalence que -40°C
 * est égal à -40°F.
 * Avec la formule suivante:
 *
 *  (f + 40)
 *  -------- - 40 = c
 *    1.8
 * Ou `f` est une température en degrés Fahrenheit et `c` est une température en degrés celsius.
 */
fn main() {
    // On associe, avec let, -40 à la variable fahrenheit.
    let fahrenheit = -40;

    //-------------------------------------------------
    // Ci-dessous on va faire notre conversion en Rust:

    // Mémo des opérateurs d'arithmétique sur les nombres courants
    // ---------------------------------
    // | Opérateur | Fonction          |
    // ---------------------------------
    // | a + b     | Addition          |
    // | a - b     | Soustraction      |
    // | a * b     | Multiplication    |
    // | a / b     | Division          |
    // ---------------------------------
    // Petite découverte du typage:
    // a et b sont des nombres ici! En Rust une expression telle que 5 a un type
    // ici le type `i32` et on ne peut pas additionner un `i32` avec n'importe
    // quel autre type, tel que une chaine de caratère.

    let celsius = ???;
    //-------------------------------------------------

    // Comme fait dans l'exercice avec le «hello-world» avec votre prénom,
    // on va afficher dans le terminal une chaine de caractère formaté.
    // Les symboles: `{}` indiquent un emplacement de formatage ou sera
    // inséré votre expression à afficher.
    // Cela permet de faire des affichages riches c'est très utilisé en Rust.
    println!("{}°F équivaut à {}°C", ???, ???);
    // Fin de notre programme de conversion.
}



































// Indice: Il faut vraiment traduire la formule mathématique décrite dans le commentaire en code rust
// le tableau récapitulatif des opérateurs est la pour vous aider! :))
// Si vous êtes vraiment bloqué venez me demander!!!! :D
