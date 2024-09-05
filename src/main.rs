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
use view::Qrcode;
use view::Toml;
use view::Uuid;
use view::Yaml;

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
                    <Route path="/qrcode" view=Qrcode />
                    <Route path="/yaml" view=Yaml />
                    <Route path="/toml" view=Toml />
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
