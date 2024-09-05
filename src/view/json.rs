use std::collections::HashMap;

use leptos::*;
use serde_json::Value;

#[component]
pub fn Json() -> impl IntoView {
    let (data, set_data) = create_signal("".to_string());
    let (msg, set_msg) = create_signal(None);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let format = move |_| {
        match serde_json::from_str::<HashMap<String, Value>>(data.get().as_str()) {
            Ok(value) => match serde_json::to_string_pretty(&value) {
                Ok(format_str) => {
                    set_data.set(format_str);
                    set_msg.set(None);
                }
                Err(e) => set_msg.set(Some(e.to_string())),
            },
            Err(e) => set_msg.set(Some(e.to_string())),
        };
    };

    let compress = move |_| {
        let origin_str = data.get();
        let format_str = origin_str.as_str().replace("\n", "").replace("\t", "");
        set_data.set(format_str);
        set_msg.set(None);
    };

    let transfer = move |_| {
        let origin_str = data.get();
        let format_str = origin_str
            .as_str()
            .replace("\\", "\\\\")
            .replace("\"", "\\\"");
        set_data.set(format_str);
        set_msg.set(None);
    };

    let detransfer = move |_| {
        let origin_str = data.get();
        let format_str = origin_str
            .as_str()
            .replace("\\\\", "\\")
            .replace("\\\"", "\"");
        set_data.set(format_str);
        set_msg.set(None);
    };

    view! {
        <section class="my-5">
        <h2 class="my-5">JSON 在线工具</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data prop:value=data id="data" rows="21" class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=format type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">格式化</button>
            <button on:click=compress type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">压缩</button>
            <button on:click=transfer type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转义</button>
            <button on:click=detransfer type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">去转义</button>
        </div>
        <Show
            when=move || { msg.get().is_some() }
            fallback=|| view! { }
        >
             <div class="p-4 mt-5 bg-yellow-100 text-yellow-800">
                <p>{msg}</p>
            </div>
        </Show>
        </section>
    }
}
