//#[macro_use] extern crate nickel;

// Projet: 
// Écrire un guess the number avec un mini serveur web et un client HTML;
// avec un framework web.


// Vous devez faire un choix Gotham ou Nickel pour le framework web
// https://gotham.rs/

// https://nickel-org.github.io/getting-started.html


fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767").unwrap();
}
