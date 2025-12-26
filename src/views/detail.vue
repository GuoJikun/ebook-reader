<script setup lang="ts">
import { Back, Setting } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import Close from "@/components/close.vue";
import useStore, { Novel } from "@/store";
import { computed, onMounted, ref, useTemplateRef, onBeforeUnmount } from "vue";

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
      color: "#333333",
    },
  set: (value: Record<string, string>) => store.updateConfig(value),
});

const updateConfig = (key: string, value: string | number) => {
  store.updateConfig({ [key]: value } as any);
};

const pagerChapter = (direction: "next" | "prev") => {
  if (!novel.value || !novel.value.chapters) return;
  if (direction === "next") {
    if (curChapIndex.value < novel.value.chapters.length - 1) {
      curChapIndex.value += 1;
    }
  } else {
    if (curChapIndex.value > 0) {
      curChapIndex.value -= 1;
    }
  }
};
const preRef = useTemplateRef<HTMLPreElement>("preRef");
const bind = (e: KeyboardEvent) => {
  if (e.key === "ArrowRight") {
    pagerChapter("next");
  } else if (e.key === "ArrowLeft") {
    pagerChapter("prev");
  }
  preRef.value?.scrollTo(0, 0);
};

onMounted(() => {
  novel.value = store.novel;
  window.addEventListener("keydown", bind);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", bind);
});
</script>

<template>
  <div class="detail">
    <div class="detail-top" data-tauri-drag-region></div>
    <div class="detail-header" data-tauri-drag-region>
      <el-link :icon="Back" @click="goBack"></el-link>
      <span>{{ novel?.title }}</span>
      <div class="detail-header-extra">
        <el-popover trigger="click" width="220">
          <template #reference>
            <el-icon><Setting /></el-icon>
          </template>
          <div>
            <el-form label-position="right" label-width="80px">
              <el-form-item label="字体大小">
                <el-input
                  v-model="config.fontSize"
                  size="small"
                  style="width: 100px"
                  @change="updateConfig('fontSize', $event)"
                ></el-input>
              </el-form-item>
              <el-form-item label="行高">
                <el-input
                  v-model="config.lineHeight"
                  size="small"
                  style="width: 100px"
                  @change="updateConfig('lineHeight', $event)"
                ></el-input>
              </el-form-item>
              <el-form-item label="字体颜色">
                <el-color-picker
                  v-model="config.color"
                  @change="updateConfig('color', $event)"
                ></el-color-picker>
              </el-form-item>
              <el-form-item label="背景颜色">
                <el-color-picker
                  v-model="config.backgroundColor"
                  @change="updateConfig('backgroundColor', $event)"
                ></el-color-picker>
              </el-form-item>
            </el-form>
          </div>
        </el-popover>
        <Close />
      </div>
    </div>
    <div class="detail-main">
      <pre class="detail-main-pre" ref="preRef">
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
    &-extra {
      display: flex;
      align-items: center;
      gap: 12px;
    }
  }

  &-main {
    flex: auto;
    overflow: hidden;
    display: flex;
    flex-direction: column;

    &-pre {
      white-space: pre-wrap;
      word-break: break-word;
      width: 100%;
      height: 100%;
      display: block;
      overflow: auto;
      font-size: v-bind("config.fontSize") + "px";
      line-height: v-bind("config.lineHeight");
      color: v-bind("config.color");
      font-family: Inter, "Helvetica Neue", Helvetica, "PingFang SC",
        "Hiragino Sans GB", "Microsoft YaHei", "微软雅黑", Arial, sans-serif;
      padding: 16px;
    }

    &-btn {
      display: flex;
      justify-content: center;
      padding: 4px 12px;
      width: 100%;
      cursor: pointer;
      &:hover {
        opacity: 0.8;
      }
    }
  }
}
</style>
