<script setup lang="ts">
import {
  Bolt,
  FileCode2,
  LayoutDashboard,
  MonitorCheck,
  Settings,
} from "@lucide/vue";
import type { Component } from "vue";

interface NavItem {
  id: string;
  label: string;
  icon: Component;
}

defineProps<{
  activeView: string;
}>();

defineEmits<{
  navigate: [view: string];
}>();

const navItems: NavItem[] = [
  { id: "overview", label: "總覽", icon: LayoutDashboard },
  { id: "clients", label: "客戶端", icon: MonitorCheck },
  { id: "scripts", label: "腳本", icon: FileCode2 },
  { id: "operations", label: "遠程操作", icon: Bolt },
  { id: "settings", label: "設定", icon: Settings },
];
</script>

<template>
  <div class="shell">
    <aside class="sidebar" aria-label="主導航">
      <div class="brand">
        <span class="brand-mark">W</span>
        <div>
          <strong>WoW Control</strong>
          <small>Agent 管理端</small>
        </div>
      </div>
      <nav>
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: item.id === activeView }"
          type="button"
          @click="$emit('navigate', item.id)"
        >
          <component :is="item.icon" :size="18" :stroke-width="2" />
          <span>{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <main class="workspace">
      <slot />
    </main>
  </div>
</template>

<style scoped>
.shell {
  display: grid;
  grid-template-columns: 220px minmax(0, 1fr);
  min-height: 100vh;
}

.sidebar {
  position: sticky;
  top: 0;
  align-self: start;
  height: 100vh;
  border-right: 1px solid var(--color-border);
  background: #ffffff;
  padding: var(--space-4);
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-8);
}

.brand-mark {
  display: grid;
  width: 38px;
  height: 38px;
  place-items: center;
  border-radius: var(--radius-control);
  background: var(--color-text);
  color: #ffffff;
  font-size: 18px;
  font-weight: 800;
}

.brand strong,
.brand small {
  display: block;
}

.brand strong {
  font-size: 15px;
}

.brand small {
  color: var(--color-muted);
  font-size: 12px;
}

nav {
  display: grid;
  gap: var(--space-2);
}

.nav-item {
  display: flex;
  width: 100%;
  align-items: center;
  gap: var(--space-3);
  border: 0;
  border-radius: var(--radius-control);
  background: transparent;
  color: var(--color-muted);
  padding: 10px var(--space-3);
  text-align: left;
  font-size: 14px;
  font-weight: 700;
}

.nav-item.active,
.nav-item:hover {
  background: var(--color-accent-soft);
  color: var(--color-accent);
}

.workspace {
  min-width: 0;
  width: min(100%, 1760px);
  padding: var(--space-5) var(--space-6) var(--space-6);
}

@media (max-width: 880px) {
  .shell {
    grid-template-columns: 1fr;
  }

  .sidebar {
    position: sticky;
    top: 0;
    z-index: 10;
    height: auto;
    display: grid;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
  }

  .brand {
    margin-bottom: 0;
  }

  nav {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: var(--space-2);
  }

  .nav-item {
    min-width: 0;
    justify-content: center;
    gap: var(--space-2);
    padding-inline: var(--space-2);
    font-size: 12px;
  }

  .workspace {
    padding: var(--space-4);
  }
}
</style>
