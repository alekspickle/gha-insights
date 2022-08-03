use rocket::{
    catch,
    http::Status,
    response::{content, status},
    Request,
};

#[catch(404)]
pub(crate) fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <p>Hmm... What are you looking for?</p>
    "#,
    )
}

#[catch(default)]
pub(crate) fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}
