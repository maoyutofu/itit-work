use ev::Event;
use leptos::*;
use leptos_meta::*;
use js_sys::{JsString, Object, Reflect, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

pub async fn md5_hash(f: File) -> Result<String, JsString> {
    let mut context = md5::Context::new();
    match f.stream().get_reader().dyn_into::<ReadableStreamDefaultReader>() {
        Ok(reader) => {
            // let mut data = Vec::new();
            loop {
                let chunk = match JsFuture::from(reader.read()).await {
                    Ok(chunk) => {
                        match chunk.dyn_into::<Object>() {
                            Ok(chunk) => chunk,
                            Err(_) => Err("dyn to Object")?
                        }
                    },
                    Err(_) => Err(JsString::from("read chunk"))?
                };
                // ReadableStreamReadResult is somehow wrong. So go by hand. Might be a web-sys bug.
                match Reflect::get(&chunk, &"done".into()) {
                    Ok(done) => {
                        if done.is_truthy() {
                            break;
                        }
                        let chunk = match Reflect::get(&chunk, &"value".into()) {
                            Ok(chunk) => {
                                match chunk.dyn_into::<Uint8Array>() {
                                    Ok(chunk) => chunk,
                                    Err(_) => Err(JsString::from("bytes are bytes"))?
                                }
                            },
                            Err(_) => Err(JsString::from("Get chunk"))?
                        };
                        context.consume(chunk.to_vec());
                        // let data_len = data.len();
                        // data.resize(data_len + chunk.length() as usize, 255);
                        // chunk.copy_to(&mut data[data_len..]);
                    },
                    Err(_) => Err(JsString::from("Get done"))?
                };
            }
            Ok(format!("{:x}", context.compute()))
        },
        Err(e) => Err(e.to_string())
    }
}


#[component]
pub fn FileHash() -> impl IntoView {
    // let (file, set_file) = create_signal("".to_string());
    let (result, set_result) = create_signal("".to_string());
    let (msg, set_msg) = create_signal(None::<String>);

    let input_file = move |ev: Event| {
        if let Some(target) = ev.target() {
            if let Some(files) = target.unchecked_ref::<web_sys::HtmlInputElement>().files() {
                if let Some(file) = files.get(0) {
                    wasm_bindgen_futures::spawn_local(async move {
                        match md5_hash(file).await {
                            Ok(hash_str) => {
                                let result_str = format!(
                                    "MD5 小写：{}\nMD5 大写：{}",
                                    hash_str,
                                    hash_str.to_uppercase()
                                );
                                set_result.set(result_str)
                            },
                            Err(e) => set_msg.set(e.as_string())
                        };
                    });
                }
            }
        }
    };

    view! {
        <Title text="文件 Hash 计算 - itit.work" />
        <section class="my-5">
        <h2 class="my-5">文件 Hash 计算</h2>
        <input on:change=input_file class="block w-full text-sm text-gray-900 border border-gray-300 cursor-pointer bg-gray-50" id="file_input" type="file" />
        <label for="result" class="block mt-5 mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
        </section>
        <Show
            when=move || { msg.get().is_some() }
            fallback=|| view! { }
        >
            <div class="p-4 mt-5 bg-yellow-100 text-yellow-800">
                <p>{msg}</p>
            </div>
        </Show>
    }
}