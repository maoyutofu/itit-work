use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="my-5">
            <h2>友情连接</h2>
            <div class="mt-5 px-5 py-5 bg-white">
                <ul class="flex items-start list-disc">
                    <li class="text-xs">
                        <a href="https://jizhong.plus/" target="_blank" inner_html="Jizhong's blog"></a>
                    </li>
                </ul>
            </div>
        </section>
        <div class="text-sm" inner_html="&copy;"> 2024 itit.work</div>
    }
}
