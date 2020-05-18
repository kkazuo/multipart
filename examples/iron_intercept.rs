extern crate iron;
extern crate multipart_any;

use iron::prelude::*;

use multipart_any::server::Entries;
use multipart_any::server::iron::Intercept;

fn main() {
    // We start with a basic request handler chain.
    let mut chain = Chain::new(|req: &mut Request|
        if let Some(entries) = req.extensions.get::<Entries>() {
            Ok(Response::with(format!("{:?}", entries)))
        } else {
            Ok(Response::with("Not a multipart request"))
        }
    );

    // `Intercept` will read out the entries and place them as an extension in the request.
    // It has various builder-style methods for changing how it will behave, but has sane settings
    // by default.
    chain.link_before(Intercept::default());

    Iron::new(chain).http("localhost:80").unwrap();
}
