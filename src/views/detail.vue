<script setup lang="ts">
import { Back } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import Close from "@/components/close.vue";
import useStore, { Novel } from "@/store";
import { computed, onMounted, ref } from "vue";

const router = useRouter();
const store = useStore();

const goBack = () => {
  router.push("/");
};

const novel = ref<Novel | null>(null);
const curChapIndex = computed({
  get: () => store.curChapterIndex,
  set: (value: number) => store.updateCurChapterIndex(value),
});

const curContent = computed(() => {
  if (novel.value && novel.value?.chapters?.length) {
    return novel.value.chapters?.[curChapIndex.value].content;
  }
  return "";
});

const config = computed({
  get: () =>
    store?.config ?? {
      fontSize: 14,
      lineHeight: 1.6,
      backgroundColor: "#ffffff",
    },
  set: (value: Record<string, string>) => store.updateConfig(value),
});

onMounted(() => {
  novel.value = store.novel;
  console.log("Novel Info on Detail Page:", novel.value);
});
</script>

<template>
  <div class="detail">
    <div class="detail-top" data-tauri-drag-region></div>
    <div class="detail-header" data-tauri-drag-region>
      <el-link :icon="Back" @click="goBack"></el-link>
      <span>{{ novel?.title }}</span>
      <div><Close /></div>
    </div>
    <div class="detail-main">
      <pre
        style="
          white-space: pre-wrap;
          word-break: break-word;
          width: 100%;
          height: 100%;
          display: block;
          overflow: auto;
        "
      >
        {{ curContent }}
      </pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.detail {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background-color: v-bind("config.backgroundColor");
  &-header {
    height: 40px;
    padding: 4px 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  &-main {
    flex: auto;
    overflow: hidden;
    &-pre {
      font-size: v-bind("config.fontSize") + "px";
      line-height: v-bind("config.lineHeight");
      padding: 16px;
    }
  }
}
</style>
