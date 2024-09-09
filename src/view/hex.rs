use leptos::*;

#[component]
pub fn Hex() -> impl IntoView {
    let (data, set_data) = create_signal(0);
    let (result, set_result) = create_signal("".to_string());

    let input_data = move |ev| {
        if let Ok(v) = event_target_value(&ev).parse::<i32>() {
            set_data.set(v);
        }
    };

    let to_hex = move |_| {
        set_result.set(format!("{:x}", data.get()));
    };

    view! {
        <section class="my-5">
            <h2 class="my-5">十进制转十六进制</h2>
            <label for="number-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">十进制</label>
            <input on:input=input_data type="number" id="number-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=to_hex type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转换</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900 mt-5">十六进制</label>
            <input prop:value=move || result.get() type="text" id="result" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
        </section>
    }
}