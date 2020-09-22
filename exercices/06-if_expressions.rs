// -----------------------------------------------------------------------------
// Programation avancée - exercice 2020
// Intervenant: Axel Viala (darnuria) axel@darnuria.eu
//
// 06-if_expressions.rs
//
// Dans cet exerice, on va découvrir le if en Rust qui permet
// d'exprimer des conditions, il existe en deux versions principales
// Le if-expressionnel qui renvoie une valeur et imperatif
// comme vous avez vu en C.
//
// Si vous avez déjà fait du Ocaml rien de nouveau!
//
// Objectifs pédagogiques:
//
// - Retour sur les expressions
// - différences Expression / Instruction
// - Retourner une valeur d'une fonction
// - Cadriciel de test intégré `mod test`
// - macro `assert_eq!` et `unimplemented!`
//
// -----------------------------------------------------------------------------

// Inspiré et traduit de l'exercice proposée sur Exercism.io "Leap Year"
// https://github.com/exercism/rust/tree/master/exercises/leap
// Attribution: Exercism contributors (MIT License)

/// Cette fonction envoie `true` si l'année passée en paramètre est bissextile
/// sinon `false`.
/// 
/// ## Algorithme
/// 
/// Une année est bisextile si elle est:
///
/// * Divisible par 4
/// * Mais Pas divisible par 100, 
///     * Sauf si elle est aussi divisible par 400
/// 
/// ## Arguments
/// 
/// * `year`: represente une année en décimal comme `2020`.
/// 
/// ## Exemples
/// ```rust
/// assert_eq!(is_leap_year(1, false))
/// assert_eq!(is_leap_year(4, true))
/// assert_eq!(is_leap_year(2400, true))
/// ```
fn is_leap_year(year: u32) -> bool { 
    // Cette macro crash le programme si appellée pratique pour écrire
    // un skelette de fonction et ensuite programmer.
    unimplemented!("C'est à vous de jouer! ;)")
}

fn main() {
    let year = 1900;
    println!("{} est une année: {}", year, if is_leap_year(year) { "bissextile" } else { "commune" });
    println!("As tu pensé à executer les test avec rustc --test main? :)")
}


// Partie réservée au tests!
// Editez si vous le désirez mais attention a pas tout casser. ;)
mod test {
    use super::is_leap_year;
    fn help_leapyear_case(year: u32, expected: bool) {
        assert_eq!(is_leap_year(year), expected);
    }

    #[test]
    fn test_annee_non_divisible_par_4() {
        help_leapyear_case(2015, false);
    }

    #[test]
    fn test_annee_divisible_par_2_mais_pas_4() {
        help_leapyear_case(1970, false);
    }

    #[test]
    fn test_anne_divisible_par_4_mais_pas_100() {
        help_leapyear_case(1996, true);
    }

    #[test]
    fn test_annee_divisible_par_4_et_5() {
        help_leapyear_case(1960, true);
    }

    #[test]
    fn test_annee_divisible_par_100_mais_pas_400() {
        help_leapyear_case(2100, false);
    }

    #[test]
    fn test_annee_divisible_par_100_mais_pas_400_du_coup_non() {
        help_leapyear_case(1900, false);
    }

    #[test]
    fn test_annee_divisible_par_400_bisextile() {
        help_leapyear_case(2000, true);
    }

    #[test]
    fn test_annee_divisible_par_400_mais_pas_125() {
        help_leapyear_case(2400, true);
    }

    #[test]
    fn test_annee_divisible_par_200_mais_pas_400() {
        help_leapyear_case(1800, false);
    }

    #[test]
    fn test_1997() {
        help_leapyear_case(1997, false);
    }

    #[test]
    fn test_cas_simples() {
        help_leapyear_case(1, false);
        help_leapyear_case(4, true);
        help_leapyear_case(100, false);
        help_leapyear_case(400, true);
        help_leapyear_case(900, false);
    }

    #[test]
    fn test_annee_non_bisextiles() {
        help_leapyear_case(1700, false);
        help_leapyear_case(1800, false);
        help_leapyear_case(1900, false);
    }

    #[test]
    fn test_annees_divisibles_par_100_et_400() {
        help_leapyear_case(1600, true);
        help_leapyear_case(2000, true);
        help_leapyear_case(2400, true);
    }

    #[test]
    fn test_annees_1600_a_1699_exclue() {
        let incorrect_years = (1600..1700)
            .filter(|&year| is_leap_year(year) != (year % 4 == 0))
            .collect::<Vec<_>>();

        if !incorrect_years.is_empty() {
            panic!("incorrect result for years: {:?}", incorrect_years);
        }
    }
}