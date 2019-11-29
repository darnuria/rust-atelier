// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel Viala (darnuria)
//
// 10-lifetimes-basic.rs
//
// Objectifs pédagogiques:
//
// - Découverte du concept de lifetime
//
// Inspiré par l'exemple de *programming rust* de Jim Blandy et
// Jason Orendorff.
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Etapes:
// 
// 1. Observer l'erreur de lifetime à la compilation
// 2. Corriger autant que neccessaire le code pour faire compiler
// le programme.

/// Prends une slice en parametre et retourne une référence
/// vers le minimum.
/// ```
///     let v = [3, 1, 2];
///     let s = smallest(&w);
///     assert_eq!(s == 1);
/// ```
fn smallest(w: &[i32]) -> &i32 {
    // Souvent en rust on «omet» les lifetime car le
    // compilateur peut les deviner (lifetime elision)
    let mut s = &w[0];
    for r in &w[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

/// Comme `smallest_lifetime` mais avec des lifetime explicites
/// Utilisez les erreurs de compilation pour corriger la signature
/// de type.
fn smallest_lifetime<'a>(w: &[i32]) -> &'a i32 {
    // Mais souvent on dois les écrires soit même,
    // Ici ce n'était aps obligé mais pour s'habituer
    // nous allons le faire.
    let mut s = &w[0];
    for r in &w[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    let a = [5, 2, 1, 6, 7, 3, 1, 3, 0, 1];
    let s_ellided = smallest(&a);
    let s_lifetime = smallest_lifetime(&a);
    assert_eq!(*s_ellided, *s_lifetime);
}