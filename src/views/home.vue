<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "@element-plus/icons-vue";
import Header from "@/components/home-header.vue";
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useRouter } from "vue-router";
import { ElMessage } from "element-plus";
import useStore from "@/store";

const store = useStore();
const router = useRouter();
const loading = ref(false);

const openFile = async () => {
  loading.value = true;
  try {
    const result = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Text Files",
          extensions: ["txt"],
        },
      ],
      title: "选择小说文件",
    });
    if (result) {
      const novel = await invoke("read_txt", { path: result as string });
      console.log("Novel content:", novel);
      if (!novel || (novel as any).chapters.length === 0) {
        ElMessage.error("无法解析该文件，请选择有效的小说文本文件。");
        return;
      } else {
        store.updateNoveInfo(novel as any);
        router.push("detail");
      }
    }
    console.log("Selected file:", result);
  } catch (error) {
    console.error("Error opening file dialog:", error);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="wrap" v-loading="loading">
    <Header class="header" data-tauri-drag-region />
    <div>
      <el-button :icon="Plus" @click="openFile">打开</el-button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.wrap {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  height: 100%;
  width: 100%;
  .header {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 40px;
    z-index: 1;
  }
}
</style>
