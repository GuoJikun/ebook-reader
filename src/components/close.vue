<script lang="ts" setup>
import { Close } from "@element-plus/icons-vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const webviewWindow = getCurrentWebviewWindow();

const closeWindow = () => {
  webviewWindow.close();
};
</script>

<template>
  <div class="btn-close">
    <el-icon @click="closeWindow"><Close /></el-icon>
  </div>
</template>

<style lang="scss" scoped>
.btn-close {
  cursor: pointer;
  &:hover {
    color: red;
  }
}
</style>
