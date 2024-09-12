use chrono::{DateTime, FixedOffset, Utc};
use leptos::*;
use leptos_meta::*;

fn utc_datetime_format(milliseconds: i64, format: &str) -> Result<String, String> {
    let seconds = milliseconds / 1000;
    let nanoseconds = (milliseconds % 1000) * 1_000_000;
    if let Some(datatime) = DateTime::from_timestamp(seconds, nanoseconds as u32) {
        let utc_datetime = DateTime::<Utc>::from_naive_utc_and_offset(datatime.naive_utc(), Utc);
        Ok(format!("{}", utc_datetime.format(format)))
    } else {
        Err("Timestamp parsing error".to_string())
    }
}

fn datetime_format(milliseconds: i64, offset: f32, format: &str) -> Result<String, String> {
    let offset_seconds = (offset * 3600_f32) as i32;
    if let Some(target_offset) = FixedOffset::east_opt(offset_seconds) {
        let seconds = milliseconds / 1000;
        let nanoseconds = (milliseconds % 1000) * 1_000_000;
        if let Some(datatime) = DateTime::from_timestamp(seconds, nanoseconds as u32) {
            let utc_datetime =
                DateTime::<Utc>::from_naive_utc_and_offset(datatime.naive_utc(), Utc);
            let target_datetime = utc_datetime.with_timezone(&target_offset);
            Ok(format!("{}", target_datetime.format(format)))
        } else {
            Err("Timestamp parsing error".to_string())
        }
    } else {
        Err("Time zone parsing error".to_string())
    }
}

#[component]
pub fn Timestamp() -> impl IntoView {
    let ctime = Utc::now().timestamp();
    let (data, set_data) = create_signal(ctime.to_string());
    let (result0, set_result0) = create_signal("".to_string());
    let (result1, set_result1) = create_signal("".to_string());
    let (msg, set_msg) = create_signal(None::<String>);
    let (unit, set_unit) = create_signal("s".to_string());
    let (tz, set_tz) = create_signal("8".to_string());

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let change_unit = move |ev| {
        set_unit.set(event_target_value(&ev));
    };

    let change_tz = move |ev| {
        set_tz.set(event_target_value(&ev));
    };

    let transform = move |_| {
        let unit = unit.get();
        match tz.get().parse::<f32>() {
            Ok(tz) => {
                match data.get().parse::<i64>() {
                    Ok(data) => {
                        let mut new_data = data;
                        if unit == "s" {
                            new_data = data * 1000;
                        }
                        match utc_datetime_format(new_data, "%Y-%m-%d %H:%M:%S") {
                            Ok(date_str) => {
                                set_result0.set(date_str);
                                match datetime_format(new_data, tz, "%Y-%m-%d %H:%M:%S") {
                                    Ok(date_str) => {
                                        set_result1.set(date_str);
                                        set_msg.set(None);
                                    }
                                    Err(e) => set_msg.set(Some(e.to_string())),
                                }
                            }
                            Err(e) => set_msg.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => set_msg.set(Some(e.to_string())),
                };
            }
            Err(e) => set_msg.set(Some(e.to_string())),
        }
    };

    view! {
            <Title text="时间戳在线转换 - itit.work" />
            <section class="my-5">
                <h2 class="my-5">时间戳在线转换</h2>
                <label for="default-input" class="block mb-2 mt-5 text-sm font-medium text-gray-900 dark:text-white">时间戳</label>
                <div class="flex">
                    <input on:input=input_data prop:value=data type="text" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5" />
                </div>
                <div class="flex justify-between gap-5 mt-5">
                    <select on:change=change_unit id="mode" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5">
                        <option value="s">秒</option>
                        <option value="ms">毫秒</option>
                    </select>
                    <select on:change=change_tz id="mode" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5">
                        <option value="-10">-10时区  [美国] Honolulu 夏威夷檀香山</option>
                        <option value="-9">-9时区  [美国] Anchorage 阿拉斯加安克雷奇</option>
                        <option value="-8">-8时区  [加拿大] Vancouver 温哥华</option>
                        <option value="-8">-8时区  [美国] San Francisco 旧金山</option>
                        <option value="-8">-8时区  [美国] Seattle 西雅图</option>
                        <option value="-8">-8时区  [美国] Los Angeles 洛杉矶</option>
                        <option value="-7">-7时区  [加拿大] Aklavik 阿克拉维克</option>
                        <option value="-7">-7时区  [加拿大] Edmonton 艾德蒙顿</option>
                        <option value="-7">-7时区  [美国] Phoenix 凤凰城</option>
                        <option value="-7">-7时区  [美国] Denver 丹佛</option>
                        <option value="-6">-6时区  [墨西哥] Mexico City 墨西哥城</option>
                        <option value="-6">-6时区  [加拿大] Winnipeg 温尼伯</option>
                        <option value="-6">-6时区  [美国] Houston 休斯敦</option>
                        <option value="-6">-6时区  [美国] Minneapolis 明尼亚波利斯</option>
                        <option value="-6">-6时区  [美国] St. Paul 圣保罗</option>
                        <option value="-6">-6时区  [美国] New Orleans 新奥尔良</option>
                        <option value="-6">-6时区  [美国] Chicago 芝加哥</option>
                        <option value="-6">-6时区  [美国] Montgomery 蒙哥马利</option>
                        <option value="-6">-6时区  [危地马拉] Guatemala 危地马拉</option>
                        <option value="-6">-6时区  [萨尔瓦多] San Salvador 圣萨尔瓦多</option>
                        <option value="-6">-6时区  [洪都拉斯] Tegucigalpa 特古西加尔巴</option>
                        <option value="-6">-6时区  [尼加拉瓜] Managua 马那瓜</option>
                        <option value="-5">-5时区  [古巴] Havana 哈瓦那</option>
                        <option value="-5">-5时区  [美国] Indianapolis 印地安纳波利斯</option>
                        <option value="-5">-5时区  [美国] Atlanta 亚特兰大</option>
                        <option value="-5">-5时区  [美国] Detroit 底特律</option>
                        <option value="-5">-5时区  [美国] Washington DC 华盛顿哥伦比亚特区</option>
                        <option value="-5">-5时区  [美国] Philadelphia 费城</option>
                        <option value="-5">-5时区  [加拿大] Toronto 多伦多</option>
                        <option value="-5">-5时区  [加拿大] Ottawa 渥太华</option>
                        <option value="-5">-5时区  [巴哈马] Nassau 拿骚</option>
                        <option value="-5">-5时区  [秘鲁] Lima 利马</option>
                        <option value="-5">-5时区  [牙买加] Kingston 金斯敦</option>
                        <option value="-5">-5时区  [哥伦比亚] Bogota 波哥大</option>
                        <option value="-5">-5时区  [美国] New York 纽约</option>
                        <option value="-5">-5时区  [加拿大] Montreal 蒙特利尔</option>
                        <option value="-5">-5时区  [美国] Boston 波士顿</option>
                        <option value="-5">-5时区  [多米尼加] Santo Domingo 圣多明各</option>
                        <option value="-5">-5时区  [玻利维亚] La Paz 拉帕兹</option>
                        <option value="-5">-5时区  [委内瑞拉] Caracas 加拉加斯</option>
                        <option value="-5">-5时区  [波多黎各] San Juan 圣胡安</option>
                        <option value="-5">-5时区  [加拿大] Halifax 哈里法克斯</option>
                        <option value="-4">-4时区  [智利] Santiago 圣地亚哥</option>
                        <option value="-4">-4时区  [巴拉圭] Asuncion 亚松森</option>
                        <option value="-3.5" inner_html="-3.5时区  [加拿大] St. John's 圣约翰斯"></option>
                        <option value="-3">-3时区  [阿根廷] Buenos Aires 布宜诺斯艾利斯</option>
                        <option value="-3">-3时区  [乌拉圭] Montevideo 蒙特维的亚</option>
                        <option value="-3">-3时区  [巴西] Brasilia 巴西利亚</option>
                        <option value="-3">-3时区  [巴西] Sao Paulo 圣保罗</option>
                        <option value="-3">-3时区  [巴西] Rio de Janeiro 里约热内卢</option>
                        <option value="0">0时区  [冰岛] Reykjavik 雷克雅未克</option>
                        <option value="0">0时区  [葡萄牙] Lisbon 里斯本</option>
                        <option value="0">0时区  [摩洛哥] Casablanca 卡萨布兰卡</option>
                        <option value="0">0时区  [爱尔兰] Dublin 都柏林</option>
                        <option value="0">0时区  [英国] London 伦敦</option>
                        <option value="1">1时区  [西班牙] Madrid 马德里</option>
                        <option value="1">1时区  [西班牙] Barcelona 巴塞罗那</option>
                        <option value="1">1时区  [法国] Paris 巴黎</option>
                        <option value="1">1时区  [尼日利亚] Lagos 拉各斯</option>
                        <option value="1">1时区  [阿尔及利亚] Algiers 阿尔及尔</option>
                        <option value="1">1时区  [比利时] Brussels 布鲁塞尔</option>
                        <option value="1">1时区  [荷兰] Amsterdam 阿姆斯特丹</option>
                        <option value="1">1时区  [瑞士] Geneva 日内瓦</option>
                        <option value="1">1时区  [瑞士] Zurich 苏黎世</option>
                        <option value="1">1时区  [德国] Frankfurt 法兰克福</option>
                        <option value="1">1时区  [挪威] Oslo 奥斯陆</option>
                        <option value="1">1时区  [丹麦] Copenhagen 哥本哈根</option>
                        <option value="1">1时区  [意大利] Rome 罗马</option>
                        <option value="1">1时区  [德国] Berlin 柏林</option>
                        <option value="1">1时区  [捷克] Prague 布拉格</option>
                        <option value="1">1时区  [克罗地亚] Zagreb 萨格雷布</option>
                        <option value="1">1时区  [奥地利] Vienna 维也纳</option>
                        <option value="1">1时区  [瑞典] Stockholm 斯德哥尔摩</option>
                        <option value="1">1时区  [匈牙利] Budapest 布达佩斯</option>
                        <option value="1">1时区  [塞尔维亚与蒙特内哥罗] Belgrade 贝尔格莱德</option>
                        <option value="1">1时区  [波兰] Warsaw 华沙</option>
                        <option value="2">2时区  [南非] Cape Town 开普敦</option>
                        <option value="2">2时区  [保加利亚] Sofia 索非亚</option>
                        <option value="2">2时区  [希腊] Athens 雅典城</option>
                        <option value="2">2时区  [爱沙尼亚] Tallinn 塔林</option>
                        <option value="2">2时区  [芬兰] Helsinki 赫尔辛基</option>
                        <option value="2">2时区  [罗马尼亚] Bucharest 布加勒斯特</option>
                        <option value="2">2时区  [白俄罗斯] Minsk 明斯克</option>
                        <option value="2">2时区  [南非] Johannesburg 约翰尼斯堡</option>
                        <option value="2">2时区  [土耳其] Istanbul 伊斯坦布尔</option>
                        <option value="2">2时区  [乌克兰] Kyiv 基辅</option>
                        <option value="2">2时区  [乌克兰] Odesa 敖德萨</option>
                        <option value="2">2时区  [津巴布韦] Harare 哈拉雷</option>
                        <option value="2">2时区  [埃及] Cairo 开罗</option>
                        <option value="2">2时区  [土耳其] Ankara 安卡拉</option>
                        <option value="2">2时区  [以色列] Jerusalem 耶路撒冷</option>
                        <option value="2">2时区  [黎巴嫩] Beirut 贝鲁特</option>
                        <option value="2">2时区  [约旦] Amman 安曼</option>
                        <option value="3">3时区  [苏丹] Khartoum 喀土穆</option>
                        <option value="3">3时区  [肯尼亚] Nairobi 内罗毕</option>
                        <option value="3">3时区  [俄罗斯] Moscow 莫斯科</option>
                        <option value="3">3时区  [埃塞俄比亚] Addis Ababa 亚的斯亚贝巴</option>
                        <option value="3">3时区  [伊拉克] Baghdad 巴格达</option>
                        <option value="3">3时区  [也门] Aden 亚丁</option>
                        <option value="3">3时区  [沙特阿拉伯] Riyadh 利雅得</option>
                        <option value="3">3时区  [马达加斯加] Antananarivo 安塔那那利佛</option>
                        <option value="3">3时区  [科威特] Kuwait City 科威特城</option>
                        <option value="3.5">3.5时区  [伊朗] Tehran 德黑兰</option>
                        <option value="4">4时区  [阿拉伯联合酋长国] Abu Dhabi 阿布扎比</option>
                        <option value="4.5">4.5时区  [阿富汗] Kabul 喀布尔</option>
                        <option value="5">5时区  [巴基斯坦] Karachi 卡拉奇</option>
                        <option value="5">5时区  [乌兹别克斯坦] Tashkent 塔什干</option>
                        <option value="5">5时区  [巴基斯坦] Islamabad 伊斯兰堡</option>
                        <option value="5">5时区  [巴基斯坦] Lahore 拉合尔</option>
                        <option value="5.5">5.5时区  [印度] Mumbai 孟买</option>
                        <option value="5.5">5.5时区  [印度] New Delhi 新德里</option>
                        <option value="5.5">5.5时区  [印度] Kolkata 柯尔喀塔</option>
                        <option value="5.75">5.75时区  [尼泊尔] Kathmandu 加德满都</option>
                        <option value="6">6时区  [孟加拉] Dhaka 达卡</option>
                        <option value="6.5">6.5时区  [缅甸] Yangon 仰光</option>
                        <option value="7">7时区  [柬埔寨] Phnom Penh 金边</option>
                        <option value="7">7时区  [泰国] Bangkok 曼谷</option>
                        <option value="7">7时区  [越南] Hanoi 河内</option>
                        <option value="7">7时区  [印尼] Jakarta 雅加达</option>
                        <option value="8" selected>8时区  [中国] Beijing 北京</option>
                        <option value="8">8时区  [马来西亚] Kuala Lumpur 吉隆坡</option>
                        <option value="8">8时区  [新加坡] Singapore 新加坡</option>
                        <option value="8">8时区  [中国] Hong Kong 香港</option>
                        <option value="8">8时区  [澳大利亚] Perth 珀斯</option>
                        <option value="8">8时区  [菲律宾] Manila 马尼拉</option>
                        <option value="8">8时区  [中国] Shanghai 上海</option>
                        <option value="8">8时区  [中国] Taipei 台北</option>
                        <option value="9">9时区  [韩国] Seoul 首尔(汉城)</option>
                        <option value="9">9时区  [日本] Tokyo 东京</option>
                        <option value="9.5">9.5时区  [澳大利亚] Darwin 达尔文</option>
                        <option value="10">10时区  [俄罗斯] Vladivostok 符拉迪沃斯托克(海参崴)</option>
                        <option value="10">10时区  [澳大利亚] Brisbane 布里斯班</option>
                        <option value="11">11时区  [澳大利亚] Melbourne 墨尔本(*)</option>
                        <option value="11">11时区  [澳大利亚] Canberra 堪培拉(*)</option>
                        <option value="11">11时区  [澳大利亚] Sydney 悉尼(*)</option>
                        <option value="9.5">9.5时区  [澳大利亚] Adelaide 亚特雷德</option>
                        <option value="12">12时区  [俄罗斯] Kamchatka 堪察加</option>
                        <option value="12">12时区  [俄罗斯] Anadyr 阿纳德尔</option>
                        <option value="12">12时区  [斐济] Suva 苏瓦</option>
                        <option value="13">13时区  [新西兰] Wellington 惠灵顿(*)</option>
                        <option value="13.75">13.75时区  [新西兰] Chatham Island 查塔姆群岛(*)</option>
                        <option value="14">14时区  [基里巴斯] Kiritimati 圣诞岛</option>
                    </select>
                </div>
                <div class="flex mt-5 justify-end gap-1">
                    <button on:click=transform type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转换</button>
                </div>
                <label for="default-input" class="block mb-2 mt-5 text-sm font-medium text-gray-900 dark:text-white">零时区</label>
                <div class="flex">
                    <input readonly prop:value=result0 type="text" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5" />
                </div>
                <label for="default-input" class="block mb-2 mt-5 text-sm font-medium text-gray-900 dark:text-white">目标时区</label>
                <div class="flex">
                    <input readonly prop:value=result1 type="text" class="w-full bg-gray-50 border-none text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block p-2.5" />
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
