use leptos::*;
use leptos_router::A;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <section class="my-5">
            <h2>常用推荐</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://discord.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/discord.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Discord</h3>
                            <p class="text-xs text-gray-400 truncate">让您的群聊更有趣</p>
                        </div>
                    </div>
                </a>
                <a href="https://slack.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/slack.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Slack</h3>
                            <p class="text-xs text-gray-400 truncate">AI 工作管理和工作效率工具</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.reddit.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/reddit.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Reddit</h3>
                            <p class="text-xs text-gray-400 truncate">深入探索一切</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.instagram.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/instagram.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Instagram</h3>
                            <p class="text-xs text-gray-400 truncate">一款拍照分享的社交平台</p>
                        </div>
                    </div>
                </a>
                <a href="https://mp.toutiao.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/mp.toutiao.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">头条号</h3>
                            <p class="text-xs text-gray-400">今日头条媒体平台</p>
                        </div>
                    </div>
                </a>
                <a href="https://mp.weixin.qq.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/mp.weixin.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">微信公众平台</h3>
                            <p class="text-xs text-gray-400" inner_html="微信公众号、小程序开发者平台"></p>
                        </div>
                    </div>
                </a>
                <a href="https://weread.qq.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/weread.qq.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">微信读书</h3>
                            <p class="text-xs text-gray-400 truncate">记录美好生活</p>
                        </div>
                    </div>
                </a>
                <a href="https://stackoverflow.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/stackoverflow.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">StackOverflow</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="开发人员学习、分享和发展事业的地方"></p>
                        </div>
                    </div>
                </a>
                <a href="https://github.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/github.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">GitHub</h3>
                            <p class="text-xs text-gray-400 truncate">记录美好生活</p>
                        </div>
                    </div>
                </a>
                <a href="https://szfilehelper.weixin.qq.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/szfilehelper.weixin.qq.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">微信文件助手</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="手机、电脑互传文件"></p>
                        </div>
                    </div>
                </a>
            </div>
        </section>
        <section class="my-5">
            <h2>工具</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <A href="/json" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/json.webp" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">JSON</h3>
                            <p class="text-xs text-gray-400 truncate">JSON 格式化转义工具</p>
                        </div>
                    </div>
                </A>
                <A href="/md5" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/md5.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">MD5</h3>
                            <p class="text-xs text-gray-400 truncate">MD5计算工具</p>
                        </div>
                    </div>
                </A>
                <A href="/base64" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/base64.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Base64</h3>
                            <p class="text-xs text-gray-400 truncate">在线编码解码</p>
                        </div>
                    </div>
                </A>
                <A href="/uuid" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/uuid.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">UUID</h3>
                            <p class="text-xs text-gray-400 truncate">UUID 生成工具</p>
                        </div>
                    </div>
                </A>
            </div>
        </section>
        <section class="my-5">
            <h2>邮箱</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://mail.google.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/gmail.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Gmail</h3>
                            <p class="text-xs text-gray-400 truncate">Google 的邮件服务</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.microsoft.com/zh-cn/microsoft-365/outlook/web-email-login-for-outlook"
                    target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/outlook.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Outlook</h3>
                            <p class="text-xs text-gray-400 truncate">微软 Outlook 邮箱</p>
                        </div>
                    </div>
                </a>
                <a href="https://proton.me/mail" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/proton.me.svg" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Proton Mail</h3>
                            <p class="text-xs text-gray-400 truncate">保护您隐私的安全电子邮件</p>
                        </div>
                    </div>
                </a>
                <a href="https://360.yandex.com/mail/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/360.yandex.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Yandex Mail</h3>
                            <p class="text-xs text-gray-400 truncate">俄罗斯最大的电子邮件服务商</p>
                        </div>
                    </div>
                </a>
                <a href="https://mail.qiye.163.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/mail.qiye.163.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">网易企业邮箱</h3>
                            <p class="text-xs text-gray-400 truncate">由网易提供的企业邮箱服务</p>
                        </div>
                    </div>
                </a>
            </div>
        </section>
        <section class="my-5">
            <h2>创作</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://www.xiaohongshu.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.xiaohongshu.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">小红书</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="3 亿人的生活经验，都在小红书"></p>
                        </div>
                    </div>
                </a>
                <a href="https://www.douyin.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.douyin.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">抖音</h3>
                            <p class="text-xs text-gray-400 truncate">记录美好生活</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.youtube.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.youtube.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">YouTube</h3>
                            <p class="text-xs text-gray-400 truncate">一款很优质的视频社交平台</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.yuque.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.yuque.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">语雀</h3>
                            <p class="text-xs text-gray-400 truncate">文档协同 知识管理</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.feishu.cn/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/feishu.cn.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">飞书</h3>
                            <p class="text-xs text-gray-400 truncate">先进企业协作与管理平台</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.wolai.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.wolai.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">wolai</h3>
                            <p class="text-xs text-gray-400 truncate">我来 wolai</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.wiz.cn/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.wiz.cn.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">为知笔记</h3>
                            <p class="text-xs text-gray-400 truncate">ONES 旗下笔记工具</p>
                        </div>
                    </div>
                </a>
                <a href="https://fanyi.youdao.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/fanyi.youdao.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">有道翻译</h3>
                            <p class="text-xs text-gray-400 truncate">网易的翻译工具</p>
                        </div>
                    </div>
                </a>
                <a href="https://translate.google.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/translate.google.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Google 翻译</h3>
                            <p class="text-xs text-gray-400 truncate">记录美好生活</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.doubao.com/chat/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.doubao.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">豆包</h3>
                            <p class="text-xs text-gray-400 truncate">字节跳动旗下 AI 智能助手</p>
                        </div>
                    </div>
                </a>
                <a href="https://api2d.com/r/220092" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/api2d.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">API2D</h3>
                            <p class="text-xs text-gray-400 truncate">OpenAI ChatGPT 像用电一样方便</p>
                        </div>
                    </div>
                </a>
                <a href="https://yiyan.baidu.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/yiyan.baidu.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">文心一言</h3>
                            <p class="text-xs text-gray-400 truncate">百度全新一代知识增强大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://tongyi.aliyun.com/" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/tongyi.aliyun.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">通义千问</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="通情、达义，你的全能AI助手"></p>
                        </div>
                    </div>
                </a>
                <a href="https://xinghuo.xfyun.cn/desk" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/xinghuo.xfyun.cn.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">讯飞星火</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="讯飞提供的大模型服务"></p>
                        </div>
                    </div>
                </a>
                <a href="https://duckduckgo.com/?q=DuckDuckGo+AI+Chat&ia=chat&duckai=1" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/duckduckgo.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">AI Chat</h3>
                            <p class="text-xs text-gray-400 truncate" inner_html="GPT-4o mini、Claude 3 以及开源 Llama 3.1 和 Mixtral">
                            </p>
                        </div>
                    </div>
                </a>
            </div>
        </section>
    }
}
