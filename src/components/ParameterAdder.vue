<script setup lang="ts">
import { computed, ref } from "vue";
import { parameterTypes, ParameterType, Parameter } from "../parameter";

const emit = defineEmits<{
  (e: "add", param: { address: string; type: string }): void;
}>();

const addrBase = ref("/avatar/parameters/");
const paramDefault = "VRCEmotes";
const parameter = ref("");
const typ = ref<ParameterType>(parameterTypes[0]); // "Int"
const addr = computed(() => {
  const param = parameter.value ? parameter.value : paramDefault;
  return addrBase.value + param;
});

const param = computed(() => ({
  address: addr.value,
  type: typ.value,
}));
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
          :placeholder="paramDefault"
        />
      </div>
    </div>
    <div class="col-2">
      <label for="type" class="visually-hidden">type</label>
      <select name="type" id="type" v-model="typ" class="form-select">
        <option v-for="t in parameterTypes">{{ t }}</option>
      </select>
    </div>
    <button @click="emit('add', param)" class="btn btn-primary col-2">
      Add
    </button>
  </div>
</template>
