<script setup lang="ts">
import { Close } from "@element-plus/icons-vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const webviewWindow = getCurrentWebviewWindow();

const closeWindow = () => {
  webviewWindow.close();
};
</script>

<template>
  <div class="header" data-tauri-drag-region>
    <div></div>
    <div>
      <el-icon @click="closeWindow"><Close /></el-icon>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.header {
  height: 40px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 16px;
}
</style>
