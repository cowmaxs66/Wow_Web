<script setup lang="ts">
import type { Component } from "vue";

defineProps<{
  label: string;
  value: string;
  note: string;
  icon: Component;
  tone?: "default" | "success" | "warning";
}>();
</script>

<template>
  <section class="metric" :class="tone ?? 'default'">
    <div class="icon-box">
      <component :is="icon" :size="18" :stroke-width="2" />
    </div>
    <div class="metric-copy">
      <p>{{ label }}</p>
      <strong>{{ value }}</strong>
      <span>{{ note }}</span>
    </div>
  </section>
</template>

<style scoped>
.metric {
  display: grid;
  grid-template-columns: 42px 1fr;
  gap: var(--space-3);
  min-width: 0;
  padding: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
}

.icon-box {
  display: grid;
  width: 42px;
  height: 42px;
  place-items: center;
  border-radius: var(--radius-control);
  color: var(--color-accent);
  background: var(--color-accent-soft);
}

.success .icon-box {
  color: var(--color-teal);
  background: var(--color-teal-soft);
}

.warning .icon-box {
  color: var(--color-warning);
  background: #fff5db;
}

.metric-copy {
  min-width: 0;
}

p {
  margin: 0 0 var(--space-1);
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 700;
}

strong {
  display: block;
  overflow: hidden;
  color: var(--color-text);
  font-size: 22px;
  font-weight: 760;
  line-height: 1.15;
  text-overflow: ellipsis;
  white-space: nowrap;
}

span {
  display: block;
  margin-top: var(--space-1);
  overflow: hidden;
  color: var(--color-faint);
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
