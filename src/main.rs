#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::http::{Cookie,CookieJar};
use rocket::response::Redirect;
use rocket::form::Form;

#[get("/")]
fn index(jar: &CookieJar<'_>) -> Template {
    match jar.get("logged_in") {
        Some(cookie) => {
            if cookie.value() == "true" {
                println!("{}",jar.get("logged_in").unwrap().value());
                println!("{}",jar.get("username").unwrap().value());
                Template::render("index", context! {
                    logged: true,
                    username: jar.get("username").unwrap().value()
                })
            } else {
                Template::render("index", context! {
                    logged: false
                })
            }
        },
        None => {
            Template::render("index", context! {
                logged: false
            })
        }
    }

}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! {

    })
}

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

#[post("/login", data="<datos>")]
fn post_login(jar: &CookieJar<'_>, datos:Form<Login>) -> Redirect {
    jar.add(Cookie::new("username", datos.username.clone()));
    jar.add(Cookie::new("password", datos.password.clone()));
    jar.add(Cookie::new("logged_in","true"));
    println!("{} {}", datos.username, datos.password);
    Redirect::to("/")
}

#[get("/register")]
fn register() -> Template {
    Template::render("register", context! {
        
    })
}


#[derive(FromForm)]
struct Register {
    nombre: String,
    username: String,
    email: String,
    password: String,
}

#[post("/register", data="<datos>")]
fn register_post(jar: &CookieJar<'_>, datos:Form<Register>) -> Redirect {
    jar.add(Cookie::new("username", datos.username.clone()));
    jar.add(Cookie::new("name", datos.nombre.clone()));
    jar.add(Cookie::new("email", datos.email.clone()));
    jar.add(Cookie::new("password", datos.password.clone()));
    jar.add(Cookie::new("logged_in","true"));
    println!("{} {} {} {}", datos.nombre, datos.username, datos.email, datos.password);
    Redirect::to("/")
}

#[get("/bicicletas")]
fn bicicletas() -> Template {
    Template::render("bicicletas", context! {
        
    })
}

#[get("/renta")]
fn renta() -> Template {
    Template::render("renta", context! {
        
    })
}

#[get("/reseña")]
fn reseña() -> Template {
    Template::render("reseña", context! {
        
    })
}

#[get("/faq")]
fn faq() -> Template {
    Template::render("faq", context! {
        
    })
}

#[get("/logout")]
fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove(Cookie::named("username"));
    jar.remove(Cookie::named("password"));
    jar.remove(Cookie::named("logged_in"));
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, register, register_post, login, bicicletas, renta, reseña, faq, post_login, logout])
    .attach(Template::fairing())
}
