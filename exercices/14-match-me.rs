// -----------------------------------------------------------------------------
// Atelier Rust
// 2022-04-03 - 11h
// Animateur: Axel (darnuria) && Aurelia
//
// 14-match-me.rs
//
// Objectifs pédagogiques:
//
// - jouer avec le pattern matching: `match`
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Etapes:
//
// 1. Observer l'erreur de compilation
// 2. Corriger autant que neccessaire le code pour faire compiler
// le programme.

// Questions:
// -1.0: Axel adore "trop" les compilateurs et en met partout.
// 0.0: Pourquoi ce code ne peut pas compiler?
// 1.0: Comment réussir a avoir un code qui permet d'afficher l'element 1 après le remove?
// 2.0: Comment faire si le contenu du vector implemente pas le trait `Copy`?

// Dans cette enumeration on va essayer de representer un langage tres simple de calcul,
// Ce langage va faire des Additions, Soustractions, Divisions et Multiplications
// c'est déjà pas mal!
// Inspiré de l'exercism sur le langage Forth:
// https://exercism.org/tracks/rust/exercises/forth

/// Represente nos operations de pile
/// On va ranger dans un `Vec<Instr>`.
///
/// Comment ça va fonctionner? Imaginons une pile d'assiette,
/// on dois toujours depiler pour faire une action (sauf les bonus),
/// Par exemple additionner dois enlever avec Vec::pop() deux fois le sommet de pile
/// et on additione et push le resultat dans la pile!
enum Instr {
    // Consome les deux elements en sommets de pile et fait une multiplication
    Mult,
    // Consome les deux elements en sommets de pile et fait une division
    Div,
    // Consome les deux elements en sommets de pile et fait une subtraction
    Sub,
    // Consome les deux elements en sommets de pile et fait une addition
    Add,
    // Pousse une valeur dans notre pile
    Push(u32),
    // Bonus! :)
    // Swap,
    // Pop,
    // Print la valeur sur le sommet de pile
    // Print
    // Met 1 dans le sommet si les valeurs en sommet sont égales sinon 0
    //Eq,
}

enum EvalError {
    /// Dans un monde ideal on renvoie une erreur si on Add sans sommet de pile assez rempli!
    StackUnderflow,
    /// Pas utilisé dans cet intro mais si vous voulez faire la lecture depuis un fichier (parsing)
    /// Ça sera utile!
    UnknownInstruction,
}

struct Machine {
    code: Vec<Instr>,
    stack: Vec<u32>,
}

impl Machine {
    fn new(code: Vec<Instr>) -> Machine {
        Machine {
            code,
            stack: Vec::new(),
        }
    }

    fn evaluate(self) -> Result<(), EvalError> {
        for instr in self.code.iter() {
            match instr {
                Instr::Push(i) => {
                    // Voir Vec::push()
                    unimplemented!("Push une valeur");
                }
                Instr::Add => {
                    unimplemented!("Addition deux valuers en sommets de pile");
                }
                Instr::Mult => {
                    unimplemented!("Multiplication deux valuers en sommets de pile");
                }
                Instr::Div => {
                    unimplemented!("Division deux valuers en sommets de pile");
                }
                Instr::Sub => {
                    unimplemented!("Soustraction deux valuers en sommets de pile");
                }
            }
        }
        // Par défaut on renvoie OK mais en bonus gerer les erreurs avec ? et des retours de
        // Err(EvalError) c'est mieux :)
        Ok(())
    }
}

fn main() {
    let code = vec![
        Instr::Push(1),
        Instr::Push(1),
        Instr::Add,
        Instr::Push(2),
        Instr::Sub,
    ];
    let machine = Machine::new(code);
    machine.evaluate();
}
