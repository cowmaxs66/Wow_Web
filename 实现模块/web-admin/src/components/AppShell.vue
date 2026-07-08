<script setup lang="ts">
import {
  Bolt,
  FileCode2,
  LayoutDashboard,
  MonitorCheck,
  ScrollText,
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
  serverLabel?: string;
  serverUrl?: string;
}>();

defineEmits<{
  navigate: [view: string];
}>();

const navItems: NavItem[] = [
  { id: "overview", label: "總覽", icon: LayoutDashboard },
  { id: "clients", label: "客戶端", icon: MonitorCheck },
  { id: "scripts", label: "腳本", icon: FileCode2 },
  { id: "operations", label: "遠程操作", icon: Bolt },
  { id: "logs", label: "日誌", icon: ScrollText },
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
          <small>Desktop Console</small>
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
      <footer class="shell-status">
        <span>{{ serverUrl || "Management Server" }}</span>
        <strong>{{ serverLabel || "未知" }}</strong>
      </footer>
    </aside>

    <main class="workspace">
      <slot />
    </main>
  </div>
</template>

<style scoped>
.shell {
  display: grid;
  grid-template-columns: 236px minmax(0, 1fr);
  min-height: 100vh;
  background: var(--color-page);
}

.sidebar {
  position: sticky;
  top: 0;
  align-self: start;
  height: 100vh;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr) auto;
  border-right: 1px solid rgba(148, 163, 184, 0.2);
  background: var(--color-sidebar);
  color: var(--color-sidebar-text);
  padding: var(--space-4);
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-5);
}

.brand-mark {
  display: grid;
  width: 38px;
  height: 38px;
  place-items: center;
  border-radius: var(--radius-control);
  background: linear-gradient(135deg, var(--color-accent) 0%, #0f766e 100%);
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
  color: var(--color-sidebar-muted);
  font-size: 12px;
}

nav {
  display: grid;
  align-content: start;
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
  color: var(--color-sidebar-muted);
  padding: 10px var(--space-3);
  text-align: left;
  font-size: 14px;
  font-weight: 700;
  transition:
    background 140ms ease,
    color 140ms ease;
}

.nav-item.active,
.nav-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #ffffff;
}

.nav-item:focus {
  outline: none;
}

.nav-item:focus-visible {
  outline: 2px solid rgba(255, 255, 255, 0.42);
  outline-offset: 2px;
}

.shell-status {
  display: grid;
  gap: 2px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: var(--radius-control);
  background: rgba(255, 255, 255, 0.06);
  padding: var(--space-3);
}

.shell-status span {
  overflow: hidden;
  color: var(--color-sidebar-muted);
  font-size: 11px;
  font-weight: 760;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.shell-status strong {
  color: #d1fae5;
  font-size: 13px;
}

.workspace {
  min-width: 0;
  width: min(100%, 1840px);
  padding: var(--space-5);
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
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-2);
  }

  .nav-item {
    width: 100%;
    min-width: 0;
    justify-content: center;
    gap: var(--space-2);
    padding-inline: var(--space-2);
    font-size: 12px;
    white-space: nowrap;
  }

  .workspace {
    padding: var(--space-4);
  }

  .shell-status {
    display: none;
  }
}
</style>
