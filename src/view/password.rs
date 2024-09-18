use leptos::*;
use leptos_meta::*;
use rand::prelude::SliceRandom;

const CAPITAL_CHAR: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];
const SMALL_CHAR: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];
const DIGITAL_CHAR: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
const SYMBOLE_CHAR: [char; 17] = ['~','!','@','#','$','%','^','&','*','(', ')', '-', '+', '_', '=', ',', '.'];

fn generate_password(random_char_list: Vec<char>, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let selected: Vec<&char> = random_char_list.choose_multiple(&mut rng, length).collect();

    let password: String = String::from_iter(selected);
    password
}

#[component]
pub fn Password() -> impl IntoView {
    let (capital, set_capital) = create_signal(true);
    let (small, set_small) = create_signal(true);
    let (digital, set_digital) = create_signal(true);
    let (symbol, set_symbol) = create_signal(false);
    let (password_length, set_password_length) = create_signal(8);
    let (password_count, set_password_count) = create_signal(10);
    let (result, set_result) = create_signal("".to_string());

    let capital_change = move |_| {
        set_capital.set(!capital.get())
    };

    let small_change = move |_| {
        set_small.set(!small.get())
    };

    let digital_change = move |_| {
        set_digital.set(!digital.get())
    };

    let symbol_change = move |_| {
        set_symbol.set(!symbol.get())
    };

    let password_length_input = move |ev| {
        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
            set_password_length.set(val);
        }
    };

    let password_count_input = move |ev| {
        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
            set_password_count.set(val);
        }
    };

    let generate = move |_| {
        let mut random_char_list = vec![];

        if capital.get() {
            random_char_list.extend(CAPITAL_CHAR);
        }

        if small.get() {
            random_char_list.extend(SMALL_CHAR);
        }

        if digital.get() {
            random_char_list.extend(DIGITAL_CHAR);
        }

        if symbol.get() {
            random_char_list.extend(SYMBOLE_CHAR);
        }

        let mut password_list = vec![];
        for _ in 0..password_count.get() {
            let password = generate_password(random_char_list.clone(), password_length.get() as usize);
            password_list.push(password);
        }

        let password_str = password_list.join("\n");
        set_result.set(password_str);
    };

    view!(
        <Title text="在线随机密码生成器 - itit.work" />

        <section class="my-5">
            <h2 class="my-5">在线随机密码生成器</h2>
            <div class="flex items-center flex-wrap gap-5 mb-4">
                <div>
                    <input id="capital" type="checkbox" on:change=capital_change prop:checked=capital value="" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500" />
                    <label for="capital" class="ms-2 text-sm font-medium text-gray-900 ml-2">大写字母(A-Z)</label>
                </div>
                <div>
                    <input id="small" type="checkbox"  on:change=small_change prop:checked=small value="" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500" />
                    <label for="small" class="ms-2 text-sm font-medium text-gray-900 ml-2">小写字母(a-z)</label>
                </div>
                <div>
                    <input id="digital" type="checkbox" on:change=digital_change prop:checked=digital value="" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500" />
                    <label for="digital" class="ms-2 text-sm font-medium text-gray-900 ml-2">数字(0-9)</label>
                </div>
                <div>
                    <input id="symbol" type="checkbox" on:change=symbol_change prop:checked=symbol value="" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500" />
                    <label for="symbol" class="ms-2 text-sm font-medium text-gray-900 ml-2">其他符号(~!@#$%^&*()-+_=,.)</label>
                </div>
            </div>
            <div class="flex items-center mb-4">
                <label for="password-length" class="w-24 text-sm font-medium text-gray-900">密码长度</label>
                <input id="password-length" type="number" on:input=password_length_input prop:value=password_length class="bg-gray-50 border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" />
            </div>
            <div class="flex items-center mb-4">
                <label for="password-count" class="w-24 text-sm font-medium text-gray-900">密码个数</label>
                <input id="password-count" type="number" on:input=password_count_input prop:value=password_count class="bg-gray-50 border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" />
            </div>
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=generate type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">生成</button>
            </div>
            <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
            <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500">{result}</textarea>
        </section>
    )
}