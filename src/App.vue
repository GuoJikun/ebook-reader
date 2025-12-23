<script setup lang="ts">
import { useStore } from "./store";
import { onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const store = useStore();

onMounted(() => {
  if (store.exist) {
    router.push("detail");
  }
});
</script>

<template>
  <router-view />
</template>

<style lang="scss">
html,
body {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  margin: 0;
  padding: 0;
}
html {
  font-size: 10px;
}
body {
  font-size: 1.4rem;
  font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB",
    "Microsoft YaHei", "微软雅黑", Arial, sans-serif;
}
#app {
  width: 100%;
  height: 100%;
  overflow: auto;
}
*,
*::before,
*::after {
  box-sizing: border-box;
}
h1,
h2,
h3,
h4,
h5,
h6 {
  margin: 0;
}
</style>
