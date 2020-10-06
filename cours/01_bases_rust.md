# Bases de Rust 🦀

> Conventions: Lorsque vous verrez le symbole: 📖, il s'aggira
> d'un lien vers la documentation Rust associée au mot précédent ce symbole.
> Exemple: Index de la documentation de la bibliotheque standard de Rust[📖](https://doc.rust-lang.org/std/)

Le [livre de Rust](https://doc.rust-lang.org/book/) n'est pas en option,
on vera ici surtout des rappels et des points essentiel en français !

Pour se souvenir de la syntaxe je vous recommande la feuille de rappel interractive
disponnible sur <https://cheats.rs/>.

## Variables et mutabilité

### Déclaration de variables `let` et `mut`

Pour déclarer une variable on utilse le mot-clef `let` :

```rust
let x = 5;
```

Par défaut, en rust les variables sont **immutables**.

> Note: On dit qu'une variable est **immutable** car elle ne peux pas changer de
> **valeur** au cours de l'exécution.

Notre ami le compilateur 🤖 risque de vous sortir:
`cannot assign twice to immutable variable`.

```rust
let x = 5;
x = 7;
```

### Mutabilité sur une variable

Pour rendre une variable mutable vous pouvez utiliser le mot-clef `mut` :

```rust
let mut x = 5;
// Attention on change la valeur de x en mémoire!
x = 7;
```

**⚠ Attention ⚠:**
> Un grand pouvoir vient avec de grandes responsabilités,
>changer la valeur des choses peut apporter beaucoup de bugs mais parfois
>c'est néccessaire en Rust!

**Pour la culture:**

> Il existe des langages fonctionnels sans mutabilité comme
> [Haskell](https://en.wikipedia.org/wiki/Haskell_(programming_language))
> On laissera au compilateur le soin de transformer du code immutable en
> instructions et transformations d'état sur la mémoire. Ocaml est partiellement
> immutable.

### Expressions

On peut lier à un nom avec `let` une valeur qui contient n'importe quelle
expression.

**Par exemple :**

```rust
let c_pas_faux = (42 - 10) == 32;
```

En rust énormement de constructions sont des expressions! Le `if`, le `match`,
les appels de fonctions que nous verrons plus tard!

Et aussi les blocs de code entre `{ }`.

Par exemple le code suivant est une expression de type `i32` associée au nom
`super_calcul_complexe`.

```rust
let super_calcul_complexe = {
    let c = 20 + 2;
    c + 20
};
```

Ce qui n'est pas une expression est une instrution *statement* en anglais,
les déclarations avec `let` sont des instructions, on enchaîne les instructions
avec `;`.

> ⚠ Attention ⚠: Avec `;` on peut se retrouver à faire `{ 5 + 5; }` la valeur de
> ce block sera le tuple vide: `()` et son type `()` dit `unit` type!

### Shadowing

On peut déclarer une nouvelle variable avec un nom d'une variable déjà utilisée.

```rust
// on définit que x = 5 pour le reste du bloc de code
let x = 5;
// et en fait on se dit finalement x c'est 5 + 5;
// Mais en mémoire le premier x n'a pas changé!
let x = 5 + x;
```

Dans cet exemple: `x` n'est pas mutable, la mémoire ne peut pas être modifiée sans
le mot-clef `mut`.

Ce procédé est utile lorsqu'on à une variable qui au cours du programme,
représente quelque chose et que on ne s'intéresse pas à l'ancienne variable mais
que le sens est le même par exemple:

```rust
// Imaginez une application web
// Ou vous avez une fonction app.get('age') qui vous donne une string
// puis vous voulez transformer age en entier avec le code suivant:
let age = app.get("age"); // age est du type String
let age: i32 = age.parse().expect("Bebop impossible de convertir en i32");
// le premier age est un peu «une variable jetable».
```

Attention à ne pas faire la même chose avec `mut`.
Le compilateur va de vous dire : `mismatched types` si les types sont différents.

Si les types ne sont pas différents vous risquez d'avoir un bug! Attention mut
implique de grandes responsabilitées! Conclusion il faut utiliser `mut` avec
modération et préférer faire du `shadowing` en Rust quand on ne s'importe pas
de l'ancien état d'une variable.

```rust
let mut espaces = "    "
// on essaye d'affecter un entier à une variable qui est un string.
// len sert a récupérer la taille d'une chaine de caractères.
espaces = espaces.len();
```

Mais vous pouvez faire:

```rust
let espaces = "     ";
let espaces = espaces.len();
```

### `const`

On peut définir des [constantes](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html), toujours nommées en majuscules.
Elles sont utiles pour partager une valeur avec l'ensemble d'un programme.
Nous y reviendrons plus tard.

```rust
const VITESSE_SON: u32 = 340;
```

## Essentiel à retenir sur les variables

Avoir des variables immutables par défaut évite pas mal de bugs courants et
permet pleins d'optimisations. Et on peut se concentrer lors de la lecture
sur les variables mutables! Si vous devez partager une valeur avec tout le
programme faites une constante avec `const`.

A voir plus tard: On a pas encore vu les variables notées `static` c'est pour plus
tard! ;)

## Types de données primitifs

Voici quelques types de bases que nous avons manipulé pendant la première séance
vous trouverez des liens vers la documentation Rust.

On va dire que ce sont des types dit **primitifs** du langage rust car ils
sont défini dans le compilateur.

La plupart du temps, le compilateur peut inférer le type des données sauf pour
les types des fonctions et pour les structures et quelques cas particuliers.

Quelques types de bases:

### Entiers signés - signed

Dans la vie de tout les jours on utilisera principalement les i32!

* `i8`[📖](https://doc.rust-lang.org/std/primitive.i8.html)
* `i16`[📖](https://doc.rust-lang.org/std/primitive.i16.html)
* `i32`[📖](https://doc.rust-lang.org/std/primitive.i32.html)
* `i64`[📖](https://doc.rust-lang.org/std/primitive.i64.html)
* `i128`[📖](https://doc.rust-lang.org/std/primitive.i128.html)

> Rappel: Le nombre maximum encodable sur un nombre de `n` bits est `−2^(n − 1)`
> dans les valeurs negatives et dans les valeurs positives: `2 n ^ (n - 1)`.

Par exemple pour un `i8` l'encodage possible va de `-128` à `127` en hexadecimal
respectivement `0x80` et `0x7f`.

#### Erreurs et overflow

En Rust par défaut le compilateur vous protége des dépassements de capacité
à la compilation et à l'execution en mode *debug*.

```rust
fn main () {
    let a:i8 = 128;
    println!("{:#X}")
}
```

Va vous donner l'erreur suivante:

```rust
error: literal out of range for `i8`
 --> test.rs:2:16
  |
2 |     let a:i8 = 128;
  |                ^^^
  |
  = note: `#[deny(overflowing_literals)]` on by default
  = note: the literal `128` does not fit into the type `i8` whose range is `-128..=127`

error: aborting due to previous error
```

Alors que le code C compilé avec `gcc -Wall -Wextra` compilera joyeusement et
exploitera tristement un comportement indéfini du standard C dit
*undefined behaviour* en anglais. 👻🖥

> Note si vous êtes curieux voici un lien pour obtenir le standard C en version
> dernier brouillon pour le C18 :
> [Standard C2018 last draft edition](https://web.archive.org/web/20200929114427/http://www.open-std.org/jtc1/sc22/wg14/www/docs/n2454.pdf)
> Pour les autres versions du standard tout les derniers brouillons sont
> disponnibles gratuitement ici: <http://www.iso-9899.info/wiki/The_Standard>

```C
#include <stdint.h> // Definitions de types standards comme int32_t
#include <stdio.h>  // printf

int main(void) {
  int8_t a = 128; // int8_t Entier sur 8bits
  printf("0x%X\n", a); // %X formatage hexadecimal
  return 0;
}
```

Seule une compilation avec `-Wconversion` du code çi dessus vous donnera
un message de **warning** pas très explicite:

```C
gcc -Wall -Wextra -Wstrict-overflow -Wconversion test.c
test.c: In function ‘main’:
test.c:5:14: warning: conversion from ‘int’ to ‘int8_t’ {aka ‘signed char’} changes value from ‘128’ to ‘-128’ [-Wconversion]
    5 |   int8_t a = 128;
      |              ^~~
```

Le compilateur C ici va auto-convertir 128 (`int32_t`) en -128 (`int8_t`) !

Conclusion en C, on peut avoir des surprises! Référence populaire des internets :
le meme connu sous le nom *"This is fine"*

### Entiers non signées - unsigned

Utiles si vous ne projetez pas d'utiliser des nombres négatifs. Cependant
attention convertir un unsigned en signed peut apporter des bugs! ;)

* `u8`[📖](https://doc.rust-lang.org/std/primitive.u8.html)
* `u16`[📖](https://doc.rust-lang.org/std/primitive.u16.html)
* `u32`[📖](https://doc.rust-lang.org/std/primitive.u32.html)
* `u64`[📖](https://doc.rust-lang.org/std/primitive.u64.html)
* `u128`[📖](https://doc.rust-lang.org/std/primitive.u128.html)

Rappel: la valeur maximum encodable en non signé pour un nombre de bit `n` est
`2 ^ n`, par exemple pour un `u8` on peut encoder de `0` à `256`.

### Booleens - bool

En Rust il existe un type pour les valeurs booleenne `bool`[📖](https://doc.rust-lang.org/std/primitive.bool.html),
ce type est utilisée par le `if` les boucles `while` pour determiner si l'itération
dois continuer.

Il n'admet que deux valeurs `true` et `false`.

```rust
/// Returns `true` si `temperature` est inférieur ou égal à 30°C et
/// supérieure a 19°C.
fn is_temperature_ok_for_working(temperature: i32) -> bool {
    temperature < 30 && temperature >= 19
}
```

Vous pourrez utiliser les opérateurs `&&` *et* et `||` *ou inclusif* pour faire
des expressions booleenes: pour plus d'information hésitez pas à consulter ce
chapitre du livre de Rust[]

Attention en rust rien n'est converti automatiquement comme en C en `bool`:

```rust
let a = if 5 { '👻' } else { '🦀' };
```

Donne l'erreur de compilation:

```rust
error[E0308]: mismatched types
 --> test.rs:2:16
  |
2 |     let a = if 5 { '👻' } else { '🦀' };
  |                ^ expected `bool`, found integer

error: aborting due to previous error
For more information about this error, try `rustc --explain E0308`.
```

### Caractères - `char`

En Rust les caractères type `char`[📖](https://doc.rust-lang.org/std/primitive.char.html)
 sont encodées sur *32bits* car ils representent des [valeurs scalaires unicode](https://www.unicode.org/glossary/#unicode_scalar_value) valides.

> Note avancée à propos d'utf-8: `👻 U+1F47B: Ghost` est un caractère valide mais `é` peut s'encoder
> de plusieurs façons! Par exemple `é (U+00E9): Latin Small Letter E with Acute`
> qui est un `char` valide ou une combinaison elements unicode: `U+0065: 'latin small letter e`
> et `U+0301: 'combining acute accent`.

Dans la vie de tout les jours vous devriez pas avoir trop de problèmes! Mais
quand on vera les chaines de caractères vous verrez que c'est compliqué et itérer
sur une chaine ne peux pas se faire n'importe comment. ;)

### Tuples - ()

> TODO: *A complêter en ébauche*

En Rust il existe un type pour décrire la conjonction de plusieurs types,
les tuples, c'est très pratique quand on ne veux pas nommer via une `struct`ure
chacun des champs ou faire une fonction qui renvoie plusieurs valeurs.

```rust
let a = ('🤖', 3i32); // tuple de type (char, i32)
let c = (); // tuple de type () dit unit
```

### References - `&`

En Rust vous avez du le voir, tout n'est pas directement une valeur, parfois
on va utiliser une référence, c'est comme un pointeur en C sauf que son usage
est vérifié par le compilateur Rust et une référence ne peux pas être nulle,
elle dois toujours pointer sur quelque chose de valide.

L'opérateur `&` sert à prendre une référence.

```rust
let a = 5;
let r = &a; // On prends une référence sur a
println!("{}", a);
```

Grace à de la [magie](https://doc.rust-lang.org/book/ch15-02-deref.html) du typage
souvent les références sont utilisable comme des valeurs. Mais parfois il faut
utiliser l'opérateur `*` qui sert à accéder à la valeur référencée.

#### Emprunt / Copie / Déplacement  - Borrow / Copy / Move

En Rust par défaut tout sauf *(c'est une simplification)* les types primitifs,
sera déplacé. C'est nouveaux pour vous et pour la plupart du monde de la programmation!

Habituellement, si on dit rien on prends soit en **Copie** soit en **Référence**
par exemple en Java tout les Objets sont pris par référence par défaut.

En C tout est pris par **Copie** par défaut.

En Rust c'est totalement différent par exemple imagions le vecteur de caractères
de type : `Vec<char>`

> Note: `Vec<T>`[📖](https://doc.rust-lang.org/std/vec/struct.Vec.html) est un
> type super utile pour ranger des choses aggrandisable ça sera souvent votre choix
> par défaut. ;)

```rust
/// Attention si v ne contiens rien ça plante!
fn first_move(v: Vec<char>) -> char { v[0] }
fn main() {
    let v = vec!['🦀', '🤔', '🤖'];
    println!("{}", first_move(v));
    println!("{:#?}", v); // {:#?} #? est le formateur de debuggage
}
```

Vous allez avoir la fameuse erreur:

```rust
error[E0382]: borrow of moved value: `v`
 --> test.rs:5:23
  |
3 |     let v = vec!['🦀', '🤔', '🤖'];
  |         - move occurs because `v` has type `std::vec::Vec<char>`, which does not implement the `Copy` trait
4 |     println!("{}", first_move(v));
  |                               - value moved here
5 |     println!("{:#?}", v);
  |                       ^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
```

Pour corriger cette erreur il faut réecrire notre fonction qui prends le premier
element du tableau afin que elle emprunte une référence sur le `Vec<char>`
avec `&`.

```rust
/// Attention si v ne contiens rien ça plante!
fn first_borrow(v: &Vec<char>) -> char { v[0] }
fn main() {
    let v = vec!['🦀', '🤔', '🤖'];
    println!("{}", first_move(&v));
    println!("{:#?}", v);
}
```

Les références ne peuvent pas être invalides par exemple en Rust on ne peux pas
faire

```rust
fn reference_infondee() -> &i32 {
    let a = 42;
    &a
}
```

Ce genre d'erreur en C/C++ s'appelle un *dangling pointer/reference* c'est un
comportement indéfini du standard C/C++ 👻.

#### Lifetimes - Cannot infers lifetime

Parfois il faudra ajouter des annotations de durées de vie d'une référence
dite **lifetime** comme pour:

```rust
fn max_by_ref(a: &i32, b: &i32) -> &i32 {
    if *a > *b { a } else { b }
}
```

Rust ne sachant inférer quelle durée de vie pour la référence de retour prendre.

```rust
Error[E0106]: missing lifetime specifier
 --> test.rs:1:36
  |
1 | fn max_by_ref(a: &i32, b: &i32) -> &i32 {
  |                  ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn max_by_ref<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
  |              ^^^^    ^^^^^^^     ^^^^^^^     ^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
```

Il faudra spécifier avec la syntaxe suivante la *lifetime* : `&'a`, vous pouvez nommer
la lifetime comme vous voulez par exemple `'ma_duree_de_vie`.

```rust
fn max_by_ref<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a > *b { a } else { b }
}
```

Souvent le compilateur va deviner pour vous ce mecanisme s'appelle la *lifetime ellision*
mais parfois vous devrez vous confronter aux lifetimes!

Ce mecanisme est ce qui permet d'eviter d'avoir des `malloc` et des `free` comme
en C.

### Les slices ou tranches &[]

À venir
* slices (tranches)

* fonctions
-->

### Fonctions - fn

En Rust pour définir une fonction, il faut utiliser le mot-clé `fn` par exemple
voici la fomnction la plus simple, la fonction qui ne fait rien.

```rust
fn ma_fonction_qui_fait_rien() { }
```

Plus intérressant, voici une fonction qui renvoie quelque chose
sans prendre d'argument:

```rust
fn good_number() -> i32 { 42 }
```

Une fonction qui prends un argument et renvoie une valeur ici `a` qui est un
entier sur 32 bits sera pris par **copie** nous alons revenir dessus plus tard! 🤔

```rust
fn identity(a: i32) -> i32 { a }
```

A ce stade on peut noter deux choses, le mot-clé `return` est optionnel, si vous
ne mettez pas de `;` alors la dernière expression est retournée.

Plus complexe on va faire une fonction qui prends deux arguments et en fait
la somme, les arguments seront de types `i32` et le retour sur `i32` aussi.

```rust
fn addition_en_plus_long(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
```

## Contrôle de flux d'execution

En rust pour gérer un flux d'instruction on peut utiliser le mot clef `if` et `else` et les combinner en `else if`,
la particularité est en Rust est qu'il n'y a pas de ternaires comme en C ou Javascript vous pouvez directement utiliser
le `if` comme une expression.

```rust
let bonnes_conditions_d_etudes = if temperature <= 25 { true } else { false };
```

Vous pouvez aussi avoir un `if` imperatif comme en C ou JavaScript comme çi dessous sa valeur de retour sera `()`.
```rust
if expression {
  println!("Ceci est un message a caractère imperatif");
}
```
La règle a respecter est que chaque branche dois renvoyer le même type `T` par exemple si le `if` renvoie `i32` alors le `else` dois renvoyer `i32` aussi impossible de renvoyer `i32` ou `()` sans utiliser une `enum`. ;)

## match

(a venir)

## Structures - struct

### Structures avec champs nommées dites classiques

En Rust on peut définir des structures par exemple:

```rust
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  // On construit un Point avec x = 0 y = 1
  let p = Point { x: 0, y: 1 };
  println!("{}", p.x);
}
```

Par défaut les champs seront immutables pour les avoir mutables il faudra les annoter `mut` comme
avec le mot-clé `let`. Vous remarquerez qu'on dois indiquer les types sur les champs.

### Structures avec champs non nommées - tuple struct / opaque struct

On peut aussi representer des structures sans nommer les champs par exemple:

```rust
struct Point(i32, i32);
fn main() {
  let p = Point(0, 5);
  println!("{}". p.0);
}
```

L'usage entre une structure classique ou tuple struct dépendra du besoin,
en général on utilisera des structures à champs nommées.

### Structures sans champs nommées - unit struct

Il existe des structures sans champs cela s'appelle des unit-struct. Ça sert à avoir un type pour quelque chose, mais
qui n'a pas besoin de valeur. C'est utilisé par exemple dans le framework de jeu vidéo [amethyst.rs](https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-04.html) les systèmes comme le gestionnaire de mouvement des balles jeu pong proposé en tutoriel sont representé par une unit struct.

```rust
struct UnitStruct;
fn main() {
  let u = UnitStruct; // On peut construire une instance de cette unit struct.
}
```

### Associer des fonctions aux structures - impl

En rust on peut associer des fonctions aux types comme les structures par exemple pour
créer une fonction `new` on appellera ces fonctions des méthodes.

> Note: Pas vraiment de classes en Rust comme vu dans les langages Objets.

```rust
/// Un point en deux dimensions.
struct Point { x: i32, y: i32 }

impl Point {
  /// Construit un `Point` dans un espace en deux dimensions:
  /// `x` represente sa position en abscisse
  /// `y` represente sa position en ordonnées
  fn new(x: i32, y: i32) {
    Point { x, y }
  }
}

fn main() {
  let p = Point::new(0, 5);
}
```

Ici `new` sera une méthode dite statique car accessible sans avoir un `Point`
construit avec la syntaxe `Point::new(0, 5)`, il peut y avoir autant de variantes de «constructeur»
que vous le désirez car new n'est pas un concept particulier du langage contrairement à d'autres langages.

D'autres methodes seront liée a une instance construite par example:

```rust
struct Point { x: i32, y: i32 }
impl Point {
  fn new(x: i32, y: i32) -> Self { Point { x, y } }
  // self se référe à l'instance sur laquelle on appelle add.
  // par exemple p.add(Point { x: 5, y: 5}) ici self c'est `p`.
  fn add(self, other: Self) -> Self { // Self se référe a Point ;)
    Point::new(
      self.x + other.x,
      self.y + other.y
    )
  }
}

fn main() {
  let p = Point::new(0, 0);
  let a = Point::new(1, 1);
  let o = p.add(a);
  println("x: {}; y: {}", o.x, o.y);
}
```

En Rust `self` est un sucre syntaxique pour écrire `self: Self` et `Self` se référe au type écrit après le mot
clef `impl`. `self` sera toujours en lien avec la structure à gauche du `.` dans la syntaxe `p.add(o)`.

On peut aussi écrire en syntaxe préfixée: `Point::add(p, o)` c'est equivalent.

On va voir par la suite qu'il est possible d'implementer et definir des comportements contractualisées via
les `traits`. Vous en avez peut-être manipulées sans le savoir en utilisant `#[derive(Eq, Debug)]` devant
un type. Qui demande au compilateur de générer pour nous par exemple l'égalité avec `==`.

Par exemple je peux avoir l'égalité facilement et l'affichage de débug en ajoutant juste:

```rust
#[derive(Eq, Debug)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 5, y: 5 };
  assert_eq!(p, Point { x: 5, y: 5 });
}
```

Automatiquement avec `#[derive(Eq, Debug)]` le compilateur va implementer (générer le code) pour nous des traits:

* Egalité totale:`std::cmp::Eq`[📖doc](https://doc.rust-lang.org/std/cmp/trait.Eq.html) permet d'utiliser `==`
* Affichage de Debug `std::fmt::Debug`[📖doc](https://doc.rust-lang.org/std/fmt/trait.Debug.html) formatage dans `println!`: `{:#?}`

On va revenir sur les traits plus tard. ;)


### Avancé implementé soit même un trait comme Add

Par exemple on peut définir l'addition entre deux point selon le `trait` `std::ops::Add`[📖doc](https://doc.rust-lang.org/std/ops/trait.Add.html)

```rust
// a lire On implemente Add entre Point et Point.
impl Add for Point {
      // Le trait Add neccessite d'indiquer un type associée au résultat de l'addition
    // On vera pourquoi c'est la magie derrière les itérateurs.
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

Ce qui nous donne maintenant le droit d'additionner des `Point` avec `+`!

## Types options - enum

En Rust on peut definir des `enum`erations dit types sommes ou Algebraic Data Types.

```rust
enum Colors {
  Purple,
  Red,
  Black,
}
```

Et souhaite l'utiliser!

```rust
impl Colors {
  fn to_hex(&self) -> &str {
    match self {
      Colors::Purple => "#EE82EE",
      Colors::Red => "#D90416",
      Colors::Black => "#000000",
    }
  }
}
```
