use leptos::*;
use leptos_meta::*;

#[component]
pub fn Hex() -> impl IntoView {
    let (decimalism, set_decimalism) = create_signal("".to_string());
    let (hex, set_hex) = create_signal("".to_string());
    let (bin, set_bin) = create_signal("".to_string());
    let (r#type, set_type) = create_signal("1".to_string());
    let (msg, set_msg) = create_signal(None::<String>);

    let input_decimalism = move |ev| {
        set_decimalism.set(event_target_value(&ev));
    };

    let input_hex = move |ev| {
        set_hex.set(event_target_value(&ev));
    };

    let input_bin = move |ev| {
        set_bin.set(event_target_value(&ev));
    };

    let decimal_to = move || {
        if let Ok(v) = decimalism.get().parse::<i64>() {
            set_hex.set(format!("{:x}", v));
            set_bin.set(format!("{:b}", v));
        }
    };


    let hex_to = move || {
        match i64::from_str_radix(&hex.get(), 16) {
            Ok(val) => {
                set_decimalism.set(val.to_string());
                set_bin.set(format!("{:b}", val));
            },
            Err(e) => set_msg.set(Some(e.to_string()))
        }
    };

    let bin_to = move || {
        match i64::from_str_radix(&bin.get(), 2) {
            Ok(val) => {
                set_decimalism.set(val.to_string());
                set_hex.set(format!("{:x}", val));
            },
            Err(e) => set_msg.set(Some(e.to_string()))
        }
    };

    let change_input = move |ev| {
        set_type.set(event_target_value(&ev));
    };

    let convert = move |_| {
        let type_to = r#type.get();
        let type_str = type_to.as_str();
        match type_str {
            "1" => decimal_to(),
            "2" => hex_to(),
            "3" => bin_to(),
            _=> {}
        };
    };

    view! {
        <Title text="进制转换 - itit.work" />
        <section class="my-5">
            <h2 class="my-5">进制转换</h2>
            <label for="number-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">十进制</label>
            <input on:input=input_decimalism prop:value=move || decimalism.get()  type="number" id="number-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <label for="hex-input" class="block mb-2 text-sm font-medium text-gray-900 mt-5">十六进制</label>
            <input on:input=input_hex prop:value=move || hex.get() type="text" id="hex-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <label for="bin-input" class="block mb-2 text-sm font-medium text-gray-900 mt-5">二进制</label>
            <input on:input=input_bin prop:value=move || bin.get() type="number" id="bin-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <div class="flex mt-5 justify-end gap-1">
                <select id="countries" on:change=change_input class="bg-gray-50 border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 px-5 py-2.5 me-2 mb-2">
                    <option value="1">十进制</option>
                    <option value="2">十六进制</option>
                    <option value="3">二进制</option>
                </select>
                <button on:click=convert type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转换</button>
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