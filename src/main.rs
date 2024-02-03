use axum::{http::uri, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {

    let port = std::env::var("AXUM_PORT").unwrap_or("8080".to_string());
    let url = format!("0.0.0.0:{port}");

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind(url)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {

    let random_number = rand::random::<u32>();

    Html(format!("\
    <h1>Hello, World!  {random_number}</h1>
    "))

    //     let doc_html = format!("\
    //     <html lang=\"en\">\n    <head>\n  <title>Proccess: {random_number}</title>       <link rel=\"stylesheet\" href=\"stylesign.css\">\n        <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n        <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n        <link href=\"https://fonts.googleapis.com/css2?family=Arimo&family=Poppins&display=swap\" rel=\"stylesheet\">\n    </head>\n    <body>\n        <div class=\"sign-in-container\">\n            <div class=\"sign-column s1\">\n                <div class=\"sign-column-face s2\">\n                    <div class=\"s3\">\n                        <div class=\"sign-header-section\">\n                            <div class=\"sign-in-title\">\n                                Login Page\n                            </div>\n                            <div class=\"sign-in-title-alt\">\n                                Lorem ipsum dolor sit amet\n                            </div>\n                        </div>\n                        <div class=\"sign-buttons\">\n                            <a href=\"#\" class=\"login-w-button\">\n                                <img width=\"18\" height=\"18\" src=\"https://img.icons8.com/color/48/google-logo.png\" alt=\"google-logo\" />\n                                <span>Sign in with Google</span>\n                            </a>\n                            <a href=\"#\" class=\"login-w-button\">\n                                <img width=\"18\" height=\"18\" src=\"https://img.icons8.com/ios-filled/50/mac-os.png\" alt=\"mac-os\" />\n                                <span>Sign in with Apple</span>\n                            </a>\n                        </div>\n                        <div class=\"slice-container\">\n                            <div class=\"slice-text-c\">\n                                <div class=\"slicer\"></div>\n                                <div class=\"slice-text\">Or with email</div>\n                            </div>\n                        </div>\n                        <form class=\"input-container\">\n                            <input type=\"email\" required placeholder=\"Email\">\n                            <input type=\"password\" required placeholder=\"Passowrd\">\n                            <a href=\"#\" class=\"alt-f\">\n                                Forgot Password ?\n                            </a>\n                            <button>\n                                Sign in\n                            </button>\n                            <div href=\"#\" class=\"alt-f-full\">\n                                Not a Member yet ?\n                                <a href=\"sign-up.html\" class=\"alt-f\">\n                                    Sign up\n                                </a>\n                            </div>\n                        \n                        </form>\n                    </div>\n                    \n                </div>\n            </div>\n            \n            <div class=\"sign-column w2\">\n                <div class=\"intro-p\">\n                    <div class=\"canvas-logo\">\n                        <img src=\"https://images.unsplash.com/photo-1496200186974-4293800e2c20?q=80&w=1932&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D\" alt=\"logo\">\n                    </div>\n\n                    <div class=\"intro-content\">\n                        <div class=\"intro-title\">\n                            Lorem ipsum dolor sit amet\n                        </div>\n                        \n                    </div>\n                </div>\n            </div>\n        </div>\n    </body>\n    </html>
    // ");


    // Html::from(doc_html)

}