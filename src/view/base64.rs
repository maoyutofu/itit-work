use base64::prelude::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Base64() -> impl IntoView {
    let (data, set_data) = create_signal("".to_string());
    let (result, set_result) = create_signal("".to_string());

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let encode = move |_| {
        let result_str = BASE64_STANDARD.encode(data.get());
        set_result.set(result_str);
    };
    let decode = move |_| {
        if let Ok(result_str) = BASE64_STANDARD.decode(data.get()) {
            if let Ok(result_str) = String::from_utf8(result_str) {
                set_result.set(result_str);
            }
        }
    };

    view! {
        <Title text="Base64 在线编码解码 - ITIT.Work" />
        <section class="my-5">
        <h2 class="my-5">Base64 在线编码解码</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data id="data" rows="11" class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=encode type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">编码</button>
            <button on:click=decode type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">解码</button>
        </div>
        <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
        </section>
    }
}
