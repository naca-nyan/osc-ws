<script setup lang="ts">
import { Client } from "../client";

defineProps<{
  clients: Client[];
}>();
</script>
<template>
  <div v-for="{ user } in clients">
    {{ user }}
  </div>
</template>
