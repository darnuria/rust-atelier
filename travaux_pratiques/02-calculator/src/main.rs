// -----------------------------------------------------------------------------
// Atelier Rust
// 2020-10-13 - 16h30 -> 18h
// Animateur: Axel Viala (darnuria)
//
// travaux pratiques
// 02-calculator
//
// Dans cet exercice on va écrire un programme qui lis une expression arithmetique
// en position prefixée et réalise cette operation.
//
//
// Objectifs pédagogiques:
//
// - match  + enum
// - parsing basique
// - gerer les erreurs
// - S'initier a editer du rust
//
// -----------------------------------------------------------------------------
// Étapes:
//
// - Ajouter la division
// - Documenter les fonctions avec des exemples avec /// et faire compiler les test de doc.
// - Remplacer les appels a unwrap hors test par une gestion propre des erreurs
// - Deplacer Instruction::parse par dans une implementation du trait `std::convert::TryFrom<&str>`
// - Deplacer Instruction::Display par dans une implementation du trait `std::fmt::Display`
// - Ajouter une pile ou un arbre d'expression a executer
// - lire une expression depuis argv plutôt que la stdin
// - Implementer les expressions comme un vrai arbre de syntaxe abstrait
// - Gérer les expressions infixes et les parentheses!

#[derive(Debug, PartialEq)]
enum Expression {
    Mul(i32, i32),
    Add(i32, i32),
    Sub(i32, i32),
}

// Tuples: Ocaml, Python, Swift, Kotlin?, Haskell, Scala, Typescript
// (&str, i32, i32) ~= (i32, i32, &str) ~- struct sans nom
impl Expression {
    /// Represente des expressions en notation infixe.
    /// ```rust
    /// assert_eq!(Expression::Add(1, 1), "1 + 1")
    /// ```
    fn display(&self) -> String {
        use Expression as E;
        let (op, x, y) = match self {
            E::Mul(x, y) => ('*', x, y),
            E::Add(x, y) => ('+', x, y),
            E::Sub(x, y) => ('-', x, y),
        };
        format!("{} {} {}", x, op, y)
    }

    fn execute(self) -> i32 {
        use Expression as E;
        match self {
            E::Mul(x, y) => x * y,
            E::Add(x, y) => x + y,
            E::Sub(x, y) => x - y,
        }
    }

    /// Parse an simple calculation expression in prefix format
    /// manage + - *
    /// ```rust
    /// assert_eq!(parse("+ 3 3"), Instruction::Add(3, 3));
    /// ```
    fn parse(input: &str) -> Result<Expression, std::num::ParseIntError> {
        let expr: Vec<&str> = input.split_whitespace().collect();
        if expr.len() != 3 {
            // Peut-être il faut faire un erreur plus generique
            // plutôt que de crasher! :) indice: std::Error
            panic!("Unable to handle expression not enougth parameter");
        }
        let op = expr[0];
        let x = expr[1].parse()?;
        let y = expr[2].parse()?;
        let expr = match (op, x, y) {
            ("+", x, y) => Expression::Add(x, y),
            ("*", x, y) => Expression::Mul(x, y),
            ("-", x, y) => Expression::Sub(x, y),
            // TODO: trouver une solution pour faire une erreur generique :)
            (_, _, _) => panic!("Unexpected operator >_<"),
        };
        Ok(expr)
    }
}

fn main() {
    let mut expr = String::new();
    let stdio = io::stdin();
    stdio.read_line(&mut expr).expect("Failed to read!");
    let expr = Expression::parse(expr.trim()).expect("Something wrong happened!");
    println!("{}", expr.execute());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        // Addition
        let add = Expression::Add(2, 2);
        assert_eq!("2 + 2", add.display());
        // Substraction
        let sub = Expression::Sub(2, 2);
        assert_eq!("2 - 2", sub.display());
        // Multiply
        assert_eq!("2 * 2", Expression::Mul(2, 2).display());
    }

    #[test]
    fn test_execute() {
        // Multiply
        assert_eq!(2 * 2, Expression::Mul(2, 2).execute());
        // Substraction
        assert_eq!(2 - 2, Expression::Sub(2, 2).execute());
        // Addition
        assert_eq!(2 + 2, Expression::Add(2, 2).execute());
        // Addition
        assert_eq!(1 + 0, Expression::Add(1, 0).execute());
    }

    #[test]
    fn test_parse() {
        // Multiply
        assert_eq!(Expression::Mul(2, 2), Expression::parse("* 2 2").unwrap());
        assert_eq!(Expression::Add(2, 2), Expression::parse("+ 2 2").unwrap());
        assert_eq!(Expression::Sub(2, 2), Expression::parse("- 2 2").unwrap());
    }
}
