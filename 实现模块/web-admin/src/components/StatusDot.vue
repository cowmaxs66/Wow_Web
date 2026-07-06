<script setup lang="ts">
defineProps<{
  tone: "online" | "offline" | "idle";
  label: string;
}>();
</script>

<template>
  <span class="status-dot" :class="tone">
    <span class="dot" aria-hidden="true"></span>
    <span>{{ label }}</span>
  </span>
</template>

<style scoped>
.status-dot {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--color-muted);
  font-size: 13px;
  font-weight: 650;
  white-space: nowrap;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
  background: var(--color-faint);
}

.online .dot {
  background: var(--color-success);
  box-shadow: 0 0 0 3px rgba(21, 128, 61, 0.12);
}

.offline .dot {
  background: var(--color-danger);
  box-shadow: 0 0 0 3px rgba(180, 35, 24, 0.12);
}
</style>
