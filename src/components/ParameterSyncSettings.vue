<script setup lang="ts">
import { onUnmounted, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import {
  AvatarParameterConfig,
  Parameter,
  ParameterInfo,
} from "../avatarconfig";

const props = defineProps<{
  syncedNames: string[];
}>();

const emit = defineEmits<{
  (e: "onsend", param: Parameter, value: string): void;
  (e: "onchange", params: ParameterInfo[]): void;
  (e: "onunmounted", syncedNames: string[]): void;
}>();

type ParameterSync = {
  name: string;
  synced: boolean;
} & Parameter;

function send(param: Parameter, value: string) {
  emit("onsend", param, value);
}

const avatarconfig = ref<AvatarParameterConfig | null>(null);

const parameters = ref<ParameterSync[]>([]);

watchEffect(() => {
  const synced = parameters.value.filter((p) => p.synced);
  console.log(synced);
  emit("onchange", synced);
});

onUnmounted(() => {
  emit(
    "onunmounted",
    parameters.value.filter((p) => p.synced).map((p) => p.name)
  );
});

async function getParameters(): Promise<object> {
  const avatar = await invoke("get_state", { key: "/avatar/change" });
  if (!avatar) throw new Error("avatar not detected");
  const [_typ, avatarId] = avatar as [string, string];
  const contents = await invoke("read_avatar_config", { avatarId });
  const json = contents as string;
  const trim = json.trim();
  const data = JSON.parse(trim);
  return data;
}

function getInitInputParameters() {
  const config = avatarconfig.value;
  if (config == null) return [];
  console.log("prop", props.syncedNames);
  return config.parameters
    .map((p) => ({
      name: p.name,
      synced: props.syncedNames.indexOf(p.name) > -1,
      ...p.input,
    }))
    .filter((p): p is ParameterSync => p.type !== undefined);
}

function reload() {
  getParameters()
    .then((data) => (avatarconfig.value = data as AvatarParameterConfig))
    .then(() => (parameters.value = getInitInputParameters()))
    .catch((e) => console.warn(e));
}
reload();
</script>

<template>
  <div v-if="avatarconfig === null">
    <div class="mt-3 text-center fst-italic">
      Avatar config not loaded. Try reset-avatar and <a href=".">F5</a>.
    </div>
    <hr />
  </div>
  <div v-else class="text-center mb-3">
    <h5>Avatar: {{ avatarconfig.name }}</h5>
  </div>
  <div v-for="p in parameters" class="mb-3">
    <div class="row mb-1">
      <div class="col form-label">
        {{ p.name }}
        <span class="badge rounded-pill bg-secondary text-light">
          {{ p.type }}
        </span>
        <div class="float-end">
          <div class="form-switch">
            <label class="pe-5">Sync</label>
            <input
              v-model="p.synced"
              class="form-check-input"
              type="checkbox"
            />
          </div>
        </div>
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
