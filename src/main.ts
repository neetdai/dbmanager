import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import 'vuetify/styles';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { createVuetify } from 'vuetify';
import { createI18n } from "vue-i18n";


const i18n = createI18n({
    legacy: false,
    locale: 'zh-cn', // 设置默认语言
    fallbackLocale: 'en',
    messages: {

    }
})

const vuetify = createVuetify({
    components,
    directives,
});

createApp(App).use(router).use(i18n).use(vuetify).mount("#app");
