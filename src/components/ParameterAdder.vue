<script setup lang="ts">
import { computed, ref } from "vue";

const emit = defineEmits<{
  (e: "add", addr: string, typ: string): void;
}>();

const addrBase = ref("/avatar/parameters/");
const paramDefault = "VRCEmotes";
const parameter = ref("");
const typ = ref("Int");
const addr = computed(() => {
  const param = parameter.value ? parameter.value : paramDefault;
  return addrBase.value + param;
});
</script>

<template>
  <div class="row">
    <div class="col-8">
      <label class="visually-hidden">Parameter</label>
      <div class="input-group">
        <input v-model="addrBase" class="input-group-text" />
        <input
          v-model="parameter"
          class="form-control"
          placeholder="VRCEmotes"
        />
      </div>
    </div>
    <div class="col-2">
      <label for="type" class="visually-hidden">type</label>
      <select name="type" id="type" v-model="typ" class="form-select">
        <option>Int</option>
        <option>Bool</option>
        <option>Float</option>
      </select>
    </div>
    <button @click="emit('add', addr, typ)" class="btn btn-primary col-2">
      Add
    </button>
  </div>
</template>
