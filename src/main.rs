use leptos::*;
use leptos_router::*;
use leptos_meta::*;

mod common;
mod view;

use common::Footer;
use common::Header;
use view::Aes;
use view::Base64;
use view::CaseConverter;
use view::FileHash;
use view::Index;
use view::Json;
use view::Mac;
use view::Md5;
use view::Password;
use view::Qrcode;
use view::Timestamp;
use view::Toml;
use view::Uuid;
use view::Yaml;
use view::Hex;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
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
                    <Route path="/aes" view=Aes />
                    <Route path="/timestamp" view=Timestamp />
                    <Route path="/case-converter" view=CaseConverter />
                    <Route path="/hex" view=Hex />
                    <Route path="/password" view=Password />
                    <Route path="/mac" view=Mac />
                    <Route path="/file-hash" view=FileHash />
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
