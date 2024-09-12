use leptos::*;
use leptos_meta::*;
use uuid::Uuid;

#[component]
pub fn Uuid() -> impl IntoView {
    let (count, set_count) = create_signal(5);
    let (result, set_result) = create_signal("".to_string());

    let input_data = move |ev| {
        if let Ok(v) = event_target_value(&ev).parse::<i32>() {
            set_count.set(v);
        }
    };

    let generate = move |_| {
        let mut ids = vec![];
        for _ in 0..count.get() {
            let id = Uuid::new_v4();
            ids.push(id.to_string());
        }
        set_result.set(ids.join("\n"));
    };

    view! {
        <Title text="UUID 在线生成 - itit.work" />
        <section class="my-5">
            <h2 class="my-5">UUID 在线生成</h2>
            <label for="number-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">数量</label>
            <input on:input=input_data  prop:value=move || count.get() type="number" id="number-input" aria-describedby="helper-text-explanation" class="bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" required />
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=generate type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">生成</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900 mt-5">结果</label>
            <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
        </section>
    }
}
