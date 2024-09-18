use leptos::*;
use leptos_meta::*;

use rand::{Rng, thread_rng};

fn generate_mac_address() -> String {
    let mut rng = thread_rng();
    let mac_address: String = (0..6)
        .map(|_| rng.gen_range(0..=255))
        .map(|num| format!("{:02x}", num))
        .collect::<Vec<String>>()
        .join(":");

    mac_address
}

#[component]
pub fn Mac() -> impl IntoView {
    let (count, set_count) = create_signal(10);
    let (result, set_result) = create_signal("".to_string());

    let count_input = move |ev| {
        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
            set_count.set(val);
        }
    };

    let generate = move |_| {
        let mut mac_list = vec![];

        for _ in 0..count.get() {
            mac_list.push(generate_mac_address());
        }

        let mac_str = mac_list.join("\n");
        set_result.set(mac_str);
    };

    view!(
        <Title text="在线 MAC 生成器 - itit.work" />

        <section class="my-5">
            <h2 class="my-5">在线 MAC 生成器</h2>
            <div class="flex flex-col mb-4">
                <label for="password-count" class="w-24 text-sm font-medium text-gray-900 mb-2">数量</label>
                <input id="password-count" type="number" on:input=count_input prop:value=count class="bg-gray-50 border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" />
            </div>
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=generate type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">生成</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
            <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
        </section>
    )
}