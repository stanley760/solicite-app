<template>
  <a-drawer class="drawer" title="设置" placement="right" :closable="true" :open="openDrawer" :width="drawerWidth"
    :get-container="false" :style="{ position: 'absolute' }" @close="onClose">
    <a-form name="configForm" :model="formConfig" @finish="onConfigFinish">
      <a-tabs v-model:activeKey="activeKey" tab-position="left" @change="onTabChange" :style="{ height: '100%' }">
        <a-tab-pane tab="省份配置" key="1">
          <a-form-item name="channel">
            <a-textarea v-model:value="formConfig.channel" :rows="12" style="resize: none" />
          </a-form-item>
        </a-tab-pane>
        <a-tab-pane tab="分类配置" key="2">
          <a-form-item name="classify">
            <a-textarea v-model:value="formConfig.classify" :rows="12" style="resize: none" />
          </a-form-item>
        </a-tab-pane>
        <a-tab-pane tab="系统配置" key="3">
          <a-form-item name="system">
            <a-textarea v-model:value="formConfig.system" :rows="12" style="resize: none" />
          </a-form-item>
        </a-tab-pane>
      </a-tabs>
      <a-form-item :wrapper-col="{ span: 16, offset: 5 }">
            <a-space>
                <a-button type="primary" html-type="submit"> 保存 </a-button>
            </a-space>
        </a-form-item>
    </a-form>
  </a-drawer>
</template>

<script setup>
import { reactive, ref, onMounted, watch } from 'vue';
import { message } from "ant-design-vue";
import { invoke } from '@tauri-apps/api/core';
import { Window } from '@tauri-apps/api/window';

const props = defineProps({
  openDrawer: Boolean,
  drawerWidth: String,
});

const emit  = defineEmits(['onClose']);

const activeKey = ref('1');
const openDrawer = ref(props.openDrawer);
const drawerWidth = ref(props.drawerWidth);

watch(() => props.openDrawer, (val) => {
  openDrawer.value = val;
});

watch(() => props.drawerWidth, (val) => {
  drawerWidth.value = val;
});

const formConfig = reactive({
  channel: '',
  classify: '',
  system: '',
});

const updateDrawerSize = async () => {
  const innerWidth = await Window.getCurrent().innerSize();
  drawerWidth.value = `${innerWidth}px`;
};

onMounted(() => {
  updateDrawerSize();
  Window.getCurrent().onResized(updateDrawerSize);
  const resizeObserver = new ResizeObserver(entries => {
    for (let entry of entries) {
      // 处理尺寸变化逻辑
      updateDrawerSize();
    }
  });

  // 绑定到你想要观察的元素上
  const observedElement = document.querySelector('.drawer');
  if (observedElement) {
    resizeObserver.observe(observedElement);
  }
});

const onTabChange = async (key) => {
  if (!['1', '2', '3'].includes(key)) {
    message.error(`无效tab`, 2)
    return;
  }
  try {
    const result = await invoke('read_config_by_type', { key: Number(key) });
    if (result.code === 0) {
      if (key === '1') {
        formConfig.channel = result.data;
      } else if (key === '2') {
        formConfig.classify = result.data;
      } else if (key === '3') {
        formConfig.system = result.data;
      }
    } else {
      message.error(`${result.message}`, 2);
    }
  } catch(error) {
    console.error('Error invoking read_config_by_type:', error);
  }
};

const onConfigFinish = async (values) => {
  let submitData = {};
  let k = activeKey.value;
  
  submitData = { content: values.system, key: Number(k) };
  
  const result = await invoke('update_config', submitData);
  if (result.code === 0) {
    message.success(`${result.message}`, 2)
  } else {
    message.error(`${result.message}`, 2);
  }
  onTabChange(k)
};

let onClose = () => {
  openDrawer.value = false;
  emit('onClose');
};

defineExpose({ activeKey, drawerWidth, onTabChange});
</script>