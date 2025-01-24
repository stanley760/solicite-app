<template>
  <a-flex gap="middle" vertical>
    <a-typography-title :level="3" align="center" :style="{ height: '50px', width: '100%', alignContent:'center'}">{{ new Date().getFullYear() }}年省份数据统计工具</a-typography-title>
  </a-flex>
  <Form ref="formRef"></Form>
</template>
<script>
import Drawer from '@/components/Drawer/index.vue'
// import CodeEditor from '@/components/CodeEditor/index.vue';
import Form from '@/components/Form/index.vue';
import { defineComponent } from 'vue';
export default defineComponent({
  components: {
    Drawer,
    // CodeEditor,
    Form,
  },
});
</script>