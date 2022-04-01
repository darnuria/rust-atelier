// -----------------------------------------------------------------------------
<<<<<<< HEAD:exercices/09-struct-point-copy.rs
// Programation avancée - exercice 2020
// Intervenant: Axel Viala (darnuria) axel@darnuria.eu
// Animateur: Axel Viala (darnuria)
=======
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel (darnuria) && Aurelia
>>>>>>> 806f69d (mise a jour JDLL 2022 authors):07-struct-point-copy.rs
//
// struct.rs
//
// Dans cet exercice on reviens sur la structure Point décrite dans l'exercice 7.
// Sauf que cette fois on va lui faire dériver le Trait `Copy` car au fond c'est une petite
// structure, pourquoi s'embêter avec des références.
//
// Objectifs pédagogiques:
//
// - Retour sur les structures
// - Découverte de la syntaxe pour faire des méthodes sur des structures implementant `Copy`
// - «Passage par copie»
// - Initiation aux Traits
//
// Point sur le trait Copy, lorsque on implemente Copy, on ne passe plus par référence, ni par
// semantiques de mouvement, la structure est passée comme par exemple un entier, elle est copiée
// bit à bit. C'est problématique si votre structure est grosse, cependant lorsque elle est petite
// en gros moins de 128 bits cela peut être avantageux.
// Un autre cas avantageux c'est dans un contexte de programmation concurente ou de redondance.
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// 0. On complête la structure Point2D pour la faire compiler
// 1. On ajoute une fonction `new` pour creer un Point2D
// 1.1 Dériver `PartialEq` et `Debug` avec la directive de macro: `#[derive(...)`
// 1.2 Dériver le trait `Copy` and `Clone`.
// 2. On associe la fonction `new` avec un bloc `impl Point2D {}`
// 2. On crée une fonction `add` qui fait l'addition de 2 `Points2D`, `self` et  `p`
// 2.0 Observer ce qui se passe si on tente de réutiliser un des deux arguments de `add`!
// 2.1 Reprendre `add`, pour que ce soit une implementation du trait `std::ops::Add`
//   - https://doc.rust-lang.org/std/ops/trait.Add.html
// 2.2 Écrire une fonction pour réaliser l'addition scalaire entre un nombre et un Point2D.
//   -
// 2.3 Écrire quelques tests pour nos fonctions


// Ex: 2.1
// `use` permet d'importer du code d'autres modules dans la même crate ou non.
use std::ops::Add; // ici on importe le trait Add.

/// Structure pour représenter un point 2D dans l'espace
#[derive(???)] // Voir question 1.1, 1.2
struct Point2D {
    x: i32,
    y: i32,
}
// ⬆
// struct defini un reccord/enregistrement un pack de types

impl Point2D { // Le type Self vaut le type après `impl`

    /// Construct a point from 2 `i32`.
    /// ```
    ///     let a = Point2D::new(1, 2);
    /// ```
    fn new(x: i32, y: i32) -> Point2D {
        Point2D { x, y }
    }

<<<<<<< HEAD:exercices/09-struct-point-copy.rs
    /// Offre une representation textuelle pour un humain d'un point
    /// `Point2D { x: 3, y: 5 }
    fn show(self) -> String {
        unimplemented!("Whops pas encore implementé!")
        format!(???)
=======
    /// Renvoie la valeur pour `x` de la struct point courante
    fn x(self) -> i32 {
        self.x
    }

    fn y(self) -> i32 {
        unimplemented!("A vous de jouer!")
>>>>>>> 3466d58 (Petits changements sur points):07-struct-point-copy.rs
    }
}

// Ex: 2.1
impl Add for Point2D {
    type Output = Self;
    // `Output` est un type associé exigé par le trait `Add`.
    // doc: <https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types>

    // Self is Add here.
    fn add(self, other: Self) -> Self {
        //Self::new(???, ???)) // <- a corriger
        unimplemented!("A vous de jouer!")
    }
}



fn main() {}
// Pour commencer à écrire les tests il faudra décomenter le bloc ci-dessous:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Point2D::new(1, 2);
        unimplemented!("A vous de jouer!");
    }

    #[test]
    fn test_add() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(1, 2);
        let c = a.add(b);
        assert_eq!(c.x(), 2, "whoups l'addition ne corresponds pas!");
        assert_eq!(c.y(), 4);

        let e = Point2D::new(2, 4);
        assert_eq!(c, e);
    }
}
