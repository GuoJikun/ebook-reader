<script lang="ts" setup>
import { Close } from "@element-plus/icons-vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const props = defineProps({
  size: {
    type: [Number, String],
    default: 16,
  },
});

const webviewWindow = getCurrentWebviewWindow();

const closeWindow = () => {
  webviewWindow.close();
};
</script>

<template>
  <div class="btn-close">
    <el-icon @click="closeWindow" :size="`${props.size}px`"><Close /></el-icon>
  </div>
</template>

<style lang="scss" scoped>
.btn-close {
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  &:hover {
    color: red;
  }
}
</style>
