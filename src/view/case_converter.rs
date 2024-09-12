use leptos::*;
use leptos_meta::*;

#[component]
pub fn CaseConverter() -> impl IntoView {
    let (data, set_data) = create_signal("".to_string());

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let to_uppercase = move |_| {
        set_data.set(data.get().to_uppercase())
    };
    let to_lowercase = move |_| {
        set_data.set(data.get().to_lowercase())
    };

    view! {
        <Title text="大小写在线转换 - itit.work" />
        <section class="my-5">
        <h2 class="my-5">大小写在线转换</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data prop:value=data id="data" rows="11" class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=to_uppercase type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转大写</button>
            <button on:click=to_lowercase type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转小写</button>
        </div>
        </section>
    }
}