use actix_multipart::Multipart;
use actix_web::{
    web::{post, resource, Data},
    App, HttpResponse, HttpServer,
};
use form_data::{handle_multipart, Error, Field, Form};
use futures::Future;

fn upload((mp, state): (Multipart, Data<Form>)) -> Box<Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        handle_multipart(mp, state.get_ref().clone()).map(|uploaded_content| {
            let mut map = uploaded_content.map().unwrap();
            let text = map.remove("text").unwrap().text().unwrap();
            let file = map.remove("file").unwrap().bytes().unwrap();
            println!("text: {:?}, file: {:?}", &text, &file);
            HttpResponse::Created().finish()
        }),
    )
}

fn main() -> Result<(), failure::Error> {
    let form = Form::new()
        .field("text", Field::text())
        .field("file", Field::bytes());

    println!("{:?}", form);

    HttpServer::new(move || {
        App::new()
            .data(form.clone())
            .service(resource("/upload").route(post().to(upload)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()?;

    Ok(())
}
