<template>
    <a-form ref="formRef" :model="formState" :rules="rules" name="tableForm" type="flex" justify="center"
        v-bind="layout" @finish="onFinish">
        <a-form-item has-feedback label="统计类型" name="type">
            <a-radio-group v-model:value="formState.type" style="width:380px" @change="clearContentByType">
                <a-radio :value="1">省份数量</a-radio>
                <a-radio :value="2">分类统计</a-radio>
            </a-radio-group>
        </a-form-item>

        <a-form-item has-feedback label="选择文件" name="file">
            <a-space>
                <a-input v-model:value="formState.fileName" style="width: 290px" readonly>
                    <template #prefix>
                        <FileExcelOutlined class="site-form-item-icon" />
                    </template>
                </a-input>
                <a-button @click="openFileDialog">
                    <upload-outlined></upload-outlined>
                    选择
                </a-button>
            </a-space>
        </a-form-item>

        <a-form-item has-feedback label="文件密码" name="password">
            <a-input-password v-model:value="formState.password" style="width: 380px">
                <template #prefix>
                    <LockOutlined class="site-form-item-icon" />
                </template>
            </a-input-password>
        </a-form-item>

        <a-form-item has-feedback label="工作薄" name="sheets" v-show="formState.sheetsList.length > 0">
            <a-checkbox-group v-model:value="formState.sheets" style="width: 380px">
                <a-col :span="4" v-for="(sheet, index) in formState.sheetsList" :key="index">
                    <a-checkbox :value="sheet">{{ sheet }}</a-checkbox>
                </a-col>
            </a-checkbox-group>
        </a-form-item>

        <a-form-item has-feedback label="时间范围" name="timerange">
            <a-range-picker v-model:value="formState.timerange" style="width: 380px" :locale="locale"
                :presets="rangePresets" :show-time="{
                    hideDisabledOptions: true,
                    defaultValue: [
                        dayjs('09:00:00', 'HH:mm:ss'),
                        dayjs('09:00:01', 'HH:mm:ss'),
                    ],
                }" format="YYYY-MM-DD HH:mm:ss" @change="onRangeChange" />
        </a-form-item>
        <a-form-item :wrapper-col="{ span: 16, offset: 7 }">
            <a-progress :percent="process" :steps="20" :stroke-color="['#52c41a']" />
        </a-form-item>

        <a-form-item :wrapper-col="{ span: 16, offset: 6 }">
            <a-space>
                <a-button type="primary" html-type="submit" :loading="iconLoading"> 提交 </a-button>
                <a-button type="default" html-type="reset" @click="resetForm">重置</a-button>
            </a-space>
        </a-form-item>
    </a-form>
    <a-float-button shape="circle" type="primary" @click="openSetting" :style="{ right: '60px' }">
        <template #icon>
            <SettingOutlined class="site-form-item-icon" />
        </template>
    </a-float-button>
    <Drawer ref="drawer" :openDrawer="openDrawer" :drawerWidth="drawerWidth" @onClose="closeDrawer"></Drawer>
</template>

<script setup>
import dayjs from 'dayjs';
import 'dayjs/locale/zh-cn';
import locale from 'ant-design-vue/es/date-picker/locale/zh_CN';
import { reactive, ref } from 'vue';
import { FileExcelOutlined, UploadOutlined, LockOutlined, SettingOutlined } from '@ant-design/icons-vue';
import { message } from "ant-design-vue";
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import Drawer from '@/components/Drawer/index.vue'
import { parseISO, format } from 'date-fns';
dayjs.locale('zh-cn');


const formRef = ref();
const drawer = ref();
const openDrawer = ref(false);
const drawerWidth = ref('100%');
const iconLoading = ref(false);
const process = ref(0);
let intervalId = null;
// 验证文件
let validateFile = async (_rule, value) => {
    if (!value) {
        return Promise.reject('请选择excel文件');
    } else if (value.split('.').pop() !== 'xlsx') {
        return Promise.reject('请选择类型为 .xlsx 文件');
    } else {
        return Promise.resolve();
    }
};
// 验证密码
let validatePassword = async (_rule, value) => {
    if (!formState.file) {
        return Promise.reject('请先选择文件');
    }
    const result = await invoke('read_excel_sheet_names', { path: formState.file, password: value });
    if (result.code === 0) {
        formState.sheetsList = result.data;
        return Promise.resolve();
    } else if (result.code === 104) {
        if (!value) {
            return Promise.reject('文件受保护, 请输入密码');
        }
        return Promise.reject('密码错误');
    } else {
        return Promise.reject(`${result.message}`);
    }
};

const rules = {
    file: [{
        required: true,
        validator: validateFile,
        trigger: 'change',
    }],
    sheets: [{
        min: 1,
        type: 'array',
        required: true,
        message: "请选择至少一项工作薄",
        trigger: 'change',
    }],
    timerange: [{
        required: true,
        message: '请选择时间范围',
        trigger: 'change',
    }],
    password: [{
        required: false,
        validator: validatePassword,
        trigger: 'change',
    }],
    type: [{
        required: true,
        message: '请选择统计类型',
        trigger: 'change',
    }]
};

const layout = {
    labelCol: {
        span: 7,
    },
    wrapperCol: {
        span: 17,
    },
};

const formState = reactive({
    file: '',
    fileName: '',
    password: '',
    timerange: [],
    sheets: [],
    sheetsList: [],
    type: ''
});



const onFinish = async (values) => {
    const formattedArr = values.timerange.value.map(time =>
        format(parseISO(time), 'yyyy-MM-dd HH:mm:ss')
    );
    process.value = 0;
    intervalId = setInterval(() => {
        if (process.value <= 90) {
            process.value += 10; // 模拟进度增加
        } else {
            clearInterval(intervalId);
        }
    }, 1000);
    iconLoading.value = true;
    // todo 调用rust后台读取excel并更新excel的逻辑功能
    invoke('handle_excel', {
        params: {
            type: values.type,
            path: values.file,
            password: values.password,
            timeRange: formattedArr,
            sheetName: values.sheets[0],
        }
    }).then((msg) => {
        message.success("写入成功", 2);
        iconLoading.value = false;
        process.value = 100;
    }).catch((e) => {
        message.error(e.toString(), 2);
        iconLoading.value = false;
        process.value = 0;
    }).finally(
        () => {
            clearInterval(intervalId);
            intervalId = null;
        }
    )

};



let openSetting = () => {
    openDrawer.value = true;
    drawer.value.activeKey = '1';
    drawer.value.onTabChange('1');
};

let closeDrawer = (val) => {
    openDrawer.value = val;
};

const resetForm = () => {
    if (intervalId) {
        clearInterval(intervalId);
        intervalId = null;
    }
    if (formRef.value) {
        formRef.value.resetFields();
        formState.file = '';
        formState.fileName = '';
        formState.sheets = [];
        formState.sheetsList = [];
        formState.type = '';
        process.value = 0;
    }
};

const rangePresets = ref([
    {
        label: '一天前',
        value: [
            dayjs().add(-1, 'd').hour(9).minute(0).second(1),
            dayjs().hour(9).minute(0).second(0)
        ],
    },
    {
        label: '一周前',
        value: [
            dayjs().add(-7, 'd').hour(9).minute(0).second(1),
            dayjs().hour(9).minute(0).second(0)
        ],
    },
    {
        label: '一个月前',
        value: [
            dayjs().add(-30, 'd').hour(9).minute(0).second(1),
            dayjs().hour(9).minute(0).second(0)
        ],
    },
    {
        label: '明天前',
        value: [
            dayjs().hour(9).minute(0).second(1),
            dayjs().add(1, 'd').hour(9).minute(0).second(0)
        ],
    },
]);

const openFileDialog = async () => {
    const selected = await open({
        filters: [
            { name: 'Excel Files', extensions: ['xlsx'] }
        ]
    });
    if (!selected) {
        message.error('请选择文件', 2);
        return;
    }
    formState.file = selected;
    formState.fileName = selected.split('/').pop();

    const fileExtension = selected.split('.').pop().toLowerCase();
    if (fileExtension === 'xlsx') {

        const result = await invoke('read_excel_sheet_names', { path: selected, password: formState.password });

        if (result.code === 0) {
            formState.sheetsList = result.data;
        } else {
            if (result.code === 104 && formState.password) {
                message.error(`密码错误`, 2);
                formState.password = '';
            } else if (result.code === 104 && (!formState.password)) {
                formRef.value.validateFields("password");
            } else {
                message.error(`${result.message}`, 2);
            }
        }

    } else {
        message.error('只能上传 .xlsx 文件!', 2);
        formState.file = '';
    }
};

const onRangeChange = (dates, dateStrings) => {
    if (dates) {
        formState.timerange.value = dateStrings;
    }
};
const clearContentByType = value => {
    resetForm();
    formState.type = value.target.value;
}
</script>
