use leptos::*;

#[component]
pub fn Decimal() -> impl IntoView {
    let (data, set_data) = create_signal("".to_string());
    let (result, set_result) = create_signal("".to_string());
    let (msg, set_msg) = create_signal(None);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let to_decimal = move |_| {
        match i64::from_str_radix(&data.get(), 16) {
            Ok(val) => set_result.set(val.to_string()),
            Err(e) => set_msg.set(Some(e.to_string()))
        }
    };

    view! {
        <section class="my-5">
            <h2 class="my-5">十六进制转十进制</h2>
            <label for="hex-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">十六进制</label>
            <input on:input=input_data type="text" id="hex-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=to_decimal type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转换</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900 mt-5">十进制</label>
            <input prop:value=move || result.get() type="text" id="result" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
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