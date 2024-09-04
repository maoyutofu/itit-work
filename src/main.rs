use leptos::*;
use leptos_router::*;

mod common;
mod view;

use common::Footer;
use common::Header;
use view::Base64;
use view::Index;
use view::Json;
use view::Md5;
use view::Uuid;
#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <div>
            <header>
                <Header />
            </header>
            <main>
                <Routes>
                    <Route path="/" view=Index />
                    <Route path="/base64" view=Base64 />
                    <Route path="/json" view=Json />
                    <Route path="/md5" view=Md5 />
                    <Route path="/uuid" view=Uuid />
                </Routes>
            </main>
            <footer>
                <Footer />
            </footer>
        </div>
      </Router>
    }
}
pub fn main() {
    mount_to_body(|| view! { <App /> })
}
