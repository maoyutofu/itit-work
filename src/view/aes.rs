use aes::Aes128;
use base64::prelude::*;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb, InvalidKeyIvLength};
use leptos::*;

type Aes128Ecb = Ecb<Aes128, Pkcs7>;

enum Mode {
    Aes128EbcPkcs7 = 0,
}

impl Mode {
    fn from(val: i32) -> Self {
        match val {
            0 => Mode::Aes128EbcPkcs7,
            _ => Mode::Aes128EbcPkcs7,
        }
    }
}

fn encrypt_aes128_ebc_pkcs7(key: String, data: String) -> Result<String, InvalidKeyIvLength> {
    let iv = [0u8; 16];
    let cipher = Aes128Ecb::new_from_slices(key.trim().as_bytes(), &iv)?;
    let enc = cipher.encrypt_vec(data.trim().as_bytes());
    Ok(BASE64_STANDARD.encode(&enc))
}

fn decrypt_aes128_ebc_pkcs7(
    key: String,
    data: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let iv = [0u8; 16];
    let cipher = Aes128Ecb::new_from_slices(key.trim().as_bytes(), &iv)?;
    let mut buf = BASE64_STANDARD.decode(data.trim())?;
    let decrypted_ciphertext = cipher.decrypt(&mut buf)?;
    let result = String::from_utf8(decrypted_ciphertext.to_vec())?;
    Ok(result)
}

#[component]
pub fn Aes() -> impl IntoView {
    let (data, set_data) = create_signal("".to_string());
    let (result, set_result) = create_signal("".to_string());
    let (msg, set_msg) = create_signal(None::<String>);
    let (key, set_key) = create_signal("".to_string());
    let (iv, set_iv) = create_signal("".to_string());
    let (mode, set_mode) = create_signal(0);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let input_key = move |ev| {
        set_key.set(event_target_value(&ev));
    };

    let input_iv = move |ev| {
        set_iv.set(event_target_value(&ev));
    };

    let change_mode = move |ev| {
        if let Ok(mode) = event_target_value(&ev).parse::<i32>() {
            set_mode.set(mode);
        }
    };

    let encrypt = move |_| {
        let key = key.get();
        let data = data.get();
        let mode = Mode::from(mode.get());
        let _iv = iv.get();
        let result = match mode {
            Mode::Aes128EbcPkcs7 => encrypt_aes128_ebc_pkcs7(key, data),
        };

        match result {
            Ok(result) => {
                set_result.set(result);
                set_msg.set(None);
            }
            Err(e) => set_msg.set(Some(e.to_string())),
        }
    };
    let decrypt = move |_| {
        let key = key.get();
        let data = data.get();
        let mode = Mode::from(mode.get());
        let result = match mode {
            Mode::Aes128EbcPkcs7 => decrypt_aes128_ebc_pkcs7(key, data),
        };

        match result {
            Ok(result) => {
                set_result.set(result);
                set_msg.set(None);
            }
            Err(e) => set_msg.set(Some(e.to_string())),
        }
    };

    view! {
        <section class="my-5">
            <h2 class="my-5">AES 在线加密解密</h2>
            <div class="p-4 bg-yellow-100 text-yellow-800">
                <p inner_html="说明：AES 数据块长度为 128 位，所以 IV 长度需要为 16 个字符（ECB 模式不用 IV），密钥根据指定密钥位数分别为 16、24、32 个字符，IV与密钥超过长度则截取，不足则在末尾填充 '\0' 补足"></p>
            </div>
            <label for="default-input" class="block mb-2 mt-5 text-sm font-medium text-gray-900 dark:text-white">密钥 & 偏移量</label>
            <div class="flex flex-col md:flex-row justify-between gap-5">
                <div class="flex flex-col md:flex-row w-full gap-5">
                    <input on:input=input_key type="text" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5" placeholder="key" />
                    <input on:input=input_iv type="text" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5" placeholder="iv" />
                </div>
                <select on:change=change_mode id="mode" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5">
                    // ebc 加密模式
                    // Pkcs7 填充模式
                    <option value="0">Aes128EbcPkcs7</option>
                </select>
            </div>
            <label for="data" class="block mb-2 text-sm font-medium text-gray-900 mt-5">数据</label>
            <textarea on:input=input_data id="data" rows="11" class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500"></textarea>
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=encrypt type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">加密</button>
                <button on:click=decrypt type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">解密</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
            <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
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
