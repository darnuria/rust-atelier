// -----------------------------------------------------------------------------
// Programation avancée - exercice 2020
// Intervenant: Axel Viala (darnuria) axel@darnuria.eu
//
// struct.rs
//
// Dans cet exercice on va écrire notre première structure et ses methodes,
// fonctions associées, dans un second temps on vera comment faire du passage
// par référence
//
// Objectifs pédagogiques:
//
// - Découverte des structures
// - Découverte de la syntaxe pour faire des méthodes
// - self
// - passage par référence(borrowing), copie et semantiques de mouvement (move)
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// 0. On complête la structure Point2D pour la faire compiler
// 1. On ajoute une fonction `new` pour creer un Point2D
// 2. On associe la fonction `new` avec un bloc `impl Point2D {}`
// 2. On crée une fonction `add` qui fait l'addition de 2 `Points2D`, `self` et  `p`
// 2.0 Observer ce qui se passe si on tente de réutiliser un des deux arguments de `add`!
// 2.1 On reprend `add` pour prendre par référence `self` et `p`
// 2.2 Écrire quelques tests pour nos fonctions

/// Structure pour représenter un point 2D dans l'espace
struct Point2D {
    x: i32,
    y: i32,
}

// impl {
fn new(x: i32, y: i32) -> Point2D {
    Point2D { x, y }
}

// Il faudra modifier la signature pour utiliser self
// et passer par référence! ;)
fn add(a: Point2D, p: Point2D) -> Point2D {
    new(???, ???)
}
// }

fn main() {
    
}

// Pour commencer à écrire les tests il faudra décomenter le bloc ci-dessous:
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
    }
}
*/