import { defineStore } from "pinia";
import { ref } from "vue";

export interface NoveChapter {
  title: string;
  content: string;
}

export interface Novel {
  title?: string;
  author?: string;
  description?: string;
  chapters?: Array<NoveChapter>;
}

export const useStore = defineStore(
  "mainStore",
  () => {
    const exist = ref(false);
    const novel = ref<Novel | null>({});
    const updateNoveInfo = (val: Novel | null) => {
      if (val) {
        exist.value = true;
        novel.value = val;
      } else {
        exist.value = false;
        novel.value = null;
      }
    };
    const curChapterIndex = ref(0);
    const updateCurChapterIndex = (index: number) => {
      curChapterIndex.value = index;
    };
    return {
      exist,
      novel,
      updateNoveInfo,
      curChapterIndex,
      updateCurChapterIndex,
    };
  },
  {
    persist: {
      pick: ["exist", "novel"],
    },
  }
);

export default useStore;
