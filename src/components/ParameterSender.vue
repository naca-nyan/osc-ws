<script setup lang="ts">
import { Parameter } from "../avatarconfig";

defineProps<{
  parameters: (Parameter & { name: string })[];
}>();

const emit = defineEmits<{
  (e: "onsend", param: Parameter, value: string): void;
}>();

function send(param: Parameter, value: string) {
  emit("onsend", param, value);
}
</script>

<template>
  <div v-for="p in parameters" class="mb-3">
    <div class="row mb-1">
      <div class="col form-label">
        {{ p.name }}
        <span class="badge rounded-pill bg-secondary text-light">
          {{ p.type }}
        </span>
      </div>
    </div>
    <div class="row">
      <div v-if="p.type === 'Int'" class="btn-group">
        <button
          v-for="v in ['0', '1', '2', '3', '4', '5', '6']"
          @click="send(p, v)"
          class="btn btn-outline-primary"
        >
          {{ v }}
        </button>
      </div>
      <div v-else-if="p.type === 'Bool'" class="btn-group">
        <button @click="send(p, 'true')" class="btn btn-outline-primary">
          True
        </button>
        <button @click="send(p, 'false')" class="btn btn-outline-primary">
          False
        </button>
      </div>
      <div v-else-if="p.type === 'Float'" class="btn-group">
        <button
          v-for="v in [
            '0.0',
            '0.1',
            '0.2',
            '0.3',
            '0.4',
            '0.5',
            '0.6',
            '0.7',
            '0.8',
            '0.9',
            '1.0',
          ]"
          @click="send(p, v)"
          class="btn btn-outline-primary"
        >
          {{ v }}
        </button>
      </div>
    </div>
    <hr />
  </div>
</template>
