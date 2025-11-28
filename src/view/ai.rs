use leptos::*;
use leptos_meta::*;

#[component]
pub fn Ai() -> impl IntoView {
    view! {
        <Title text="itit.work - AI 大模型导航" />
        <section class="my-5 bg-white shadow-md p-6">
            <h2>国内公共大模型</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://api2d.com/r/220092" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/api2d.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">API2D</h3>
                            <p class="text-xs text-gray-400 truncate">大模型像用电一样方便</p>
                        </div>
                    </div>
                </a>
                <a href="https://qianwen.aliyun.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/tongyi.aliyun.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">通义千问</h3>
                            <p class="text-xs text-gray-400 truncate">阿里云大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://hunyuan.tencent.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/hunyuan.tencent.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">腾讯混元</h3>
                            <p class="text-xs text-gray-400 truncate">腾讯大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://yiyan.baidu.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/yiyan.baidu.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">文心一言</h3>
                            <p class="text-xs text-gray-400 truncate">百度大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.doubao.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/www.doubao.com.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">豆包</h3>
                            <p class="text-xs text-gray-400 truncate">字节跳动大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://chatglm.cn" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/chatglm.cn.svg" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">智谱清言</h3>
                            <p class="text-xs text-gray-400 truncate">智谱AI商业大模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://kimi.moonshot.cn" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/kimi.moonshot.cn.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Kimi</h3>
                            <p class="text-xs text-gray-400 truncate">月之暗面大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://cloud.baidu.com/product/wenxinworkshop" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/qianfan.cloud.baidu.com.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">百度千帆</h3>
                            <p class="text-xs text-gray-400 truncate">百度智能云AI平台</p>
                        </div>
                    </div>
                </a>
                <a href="https://xinghuo.xfyun.cn" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/xinghuo.xfyun.cn.ico" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">讯飞星火</h3>
                            <p class="text-xs text-gray-400 truncate">科大讯飞大语言模型</p>
                        </div>
                    </div>
                </a>
            </div>
        </section>
        <section class="my-5 bg-white shadow-md p-6">
            <h2>国际公共大模型</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://chat.openai.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/chat.openai.com.svg" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">ChatGPT</h3>
                            <p class="text-xs text-gray-400 truncate">OpenAI GPT-4/3.5模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://claude.ai" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/claude.ai.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Claude</h3>
                            <p class="text-xs text-gray-400 truncate">Anthropic Claude 3系列模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://gemini.google.com" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/gemini.google.com.svg" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Gemini</h3>
                            <p class="text-xs text-gray-400 truncate">Google Gemini系列模</p>
                        </div>
                    </div>
                </a>
                <a href="https://www.perplexity.ai" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/perplexity.ai.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Perplexity</h3>
                            <p class="text-xs text-gray-400 truncate">基于多模型的AI助手</p>
                        </div>
                    </div>
                </a>
                <a href="https://mistral.ai" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/mistral.ai.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Mistral AI</h3>
                            <p class="text-xs text-gray-400 truncate">Mistral系列商业模型</p>
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
        <section class="my-5 bg-white shadow-md p-6">
            <h2>本地私有大模型</h2>
            <div
                class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                <a href="https://github.com/facebookresearch/llama" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/llama.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Llama 3</h3>
                            <p class="text-xs text-gray-400 truncate">Meta最新开源大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://github.com/QwenLM/Qwen" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/qwen.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Qwen2</h3>
                            <p class="text-xs text-gray-400 truncate">阿里通义千问开源版本</p>
                        </div>
                    </div>
                </a>
                <a href="https://github.com/01-ai/Yi" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/01ai.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">01.AI</h3>
                            <p class="text-xs text-gray-400 truncate">零一万物开源大语言模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://github.com/THUDM/ChatGLM3" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/chatglm3.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">ChatGLM3</h3>
                            <p class="text-xs text-gray-400 truncate">清华开源双语大模型</p>
                        </div>
                    </div>
                </a>
                <a href="https://github.com/baichuan-inc" target="_blank">
                    <div
                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                        <img class="w-10 h-10" src="/images/baichuan.png" alt="" />
                        <div class="ml-3 truncate">
                            <h3 class="text-sm truncate">Baichuan</h3>
                            <p class="text-xs text-gray-400 truncate">百川智能开源大语言模型</p>
                        </div>
                    </div>
                </a>
            </div>
        </section>
    }
}
