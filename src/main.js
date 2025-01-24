import { createApp } from "vue";
import Antd from 'ant-design-vue';
import App from "./App.vue";
import 'ant-design-vue/dist/reset.css';

createApp(App).use(Antd).mount("#app");

document.onkeydown = function () {
    if (window.event && window.event.keyCode == 123) {
        event.keyCode = 0;
        event.returnValue = false;
    }
    if (window.event && window.event.keyCode == 13) {
        window.event.keyCode = 505;
    }
};

//屏蔽右键菜单
document.oncontextmenu = function (event) {
    if (window.event) {
        event = window.event;
    } try {
        var the = event.srcElement;
        if (!((the.tagName == "INPUT" && the.type.toLowerCase() == "text") || the.tagName == "TEXTAREA")) {
            return false;
        }
        return true;
    } catch (e) {
        return false;
    }
};