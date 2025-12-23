<script setup lang="ts">
import { ref } from "vue";

interface FormItem {
  ebookDir: string | null;
}

const formItem = ref<FormItem>({
  ebookDir: null,
});
</script>

<template>
  <div>
    <h1>设置</h1>
    <el-divider></el-divider>
    <div>
      <el-form>
        <el-form-item label="示例设置项">
          <el-input
            v-model="formItem.ebookDir"
            placeholder="请输入内容"
          ></el-input>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>
