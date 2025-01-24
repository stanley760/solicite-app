<template>
  <div id="editorContainer" style="height: 300px; width: 100%"></div>
</template>

<script>
import * as monaco from 'monaco-editor';	// 全部导入
import { ref, onMounted, toRaw } from 'vue';
export default {
  props: {
    value: String,
    language: String,
    onChange: Function,
  },
  setup(props) {

    const editor = ref(null);

    onMounted(() => {
      editor.value = monaco.editor.create(document.getElementById('editorContainer'), {
        value: props.value || '',
        language: props.language || 'json',
        minimap: {
          enabled: false,
        },
        folding: true,
        colorDecorators: true,		//颜色装饰器
        readOnly: false,			//是否开启已读功能
        theme: "vs",			//主题
        selectOnLineNumbers: true,//显示行号
        roundedSelection: false,
        cursorStyle: 'line', //光标样式
        autoClosingBrackets: 'always', // 是否自动添加结束括号(包括中括号) "always" | "languageDefined" | "beforeWhitespace" | "never"
        autoClosingDelete: 'always', // 是否自动删除结束括号(包括中括号) "always" | "never" | "auto"
        autoClosingOvertype: 'always', // 是否关闭改写 即使用insert模式时是覆盖后面的文字还是不覆盖后面的文字 "always" | "never" | "auto"
        autoClosingQuotes: 'always', // 是否自动添加结束的单引号 双引号 "always" | "languageDefined" | "beforeWhitespace" | "never"
        autoIndent: 'None', // 控制编辑器在用户键入、粘贴、移动或缩进行时是否应自动调整缩进
        automaticLayout: false, //自动布局
        glyphMargin: true, //字形边缘
        useTabStops: false,
        fontSize: 12, //字体大小
        quickSuggestionsDelay: 100, //代码提示延时
      });

      // 监听编辑器内容变化
      editor.value.onDidChangeModelContent(() => {
        // 触发父组件的 change 事件，通知编辑器内容变化
        props.onChange(toRaw(editor.value).getValue());
      });
    });

    return {
      editor,
    };
  }
}
</script>