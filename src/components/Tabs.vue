<script lang="ts">
  
  export default {}
</script>

<script lang="ts" setup>
  import { inject, onMounted, Ref, shallowRef } from 'vue'
  import type { Component } from  'vue'
  import Tooltip from './Tooltip.vue';
  import OverflowMenu from './OverflowMenu.vue';
import { dialog } from '@tauri-apps/api';
  const {currentTabId,tabs: tabsRef, onNewTab, maxTabsDisplay} = inject<{
    tabs: Ref<{
      name: string,
      id: string,
      onClose: (index: number) => void,
      component: Component,
      props: {
        [name: string]: any
      }
    }[]>,
    maxTabsDisplay: {
      readonly maxTabs: number
    },
    onNewTab: () => void,
    currentTabId: Ref<string>
  }>('tabs')!

  const tabs = shallowRef(tabsRef)

  const changeTabSelection = (e: MouseEvent, id: string) => {
    if (e.target instanceof HTMLElement) {
      if (!e.target.matches('div.close-btn button.close')) {
        currentTabId.value = id
      }
    }
  }

  const closeTab = async (index: number, tab: {
    name: string;
    id: string;
    onClose: (index: number) => void;
    component: Component;
    props: {
      [name: string]: any
    };
  }) => {
    if (!tab.props.text.showIcon) {
      tab.onClose(index)
    } else {
      const flag = await dialog.confirm('Do you really want to close the tab? You have unsaved changes, if you close the tab, the changes will be lost!')
      if (flag) {
        tab.onClose(index);
      }
    }
  }

  const handleKeys = (e: KeyboardEvent) => {
    if (e.ctrlKey && (e.key == 'w' || e.code == 'KeyW' || e.keyCode == 87)) {
      e.preventDefault()
      const index = tabsRef.value.findIndex(tab => tab.id == currentTabId.value)
      if (index > -1) {
        closeTab(index, tabsRef.value[index])
        // tabsRef.value[index].onClose(index)
      }
    } else if (e.ctrlKey && (e.key == 't' || e.code == 'KeyT' || e.keyCode == 84)) {
      e.preventDefault()
      console.log('Yes')
      onNewTab()
    }
  }

  const overflowClick = (id: string) => {
    const tabIndex = tabsRef.value.findIndex(a => id == a.id);
    if (tabIndex > -1) {
      const [tab] = tabsRef.value.splice(tabIndex, 1)
      tabsRef.value.unshift(tab)
      currentTabId.value = tab.id
    }
  }



  onMounted(() => {
    window.addEventListener('keydown', handleKeys)
  })

  const openFile = inject<() => void>('openFile')!

</script>

<template>
  <div>
    <div class="flex w-full text-gray-500 border-b border-gray-200 relative dark:text-gray-400 dark:border-gray-700">
      <div class="relative">
        <OverflowMenu className="menu-overflow-left" menuClassName="menu-left" :menuItems="[
          {
            id: 'new-file', 
            onClick: onNewTab,
            title: 'New Text File'
          },
          {
            id: 'open-file', 
            onClick: openFile,
            title: 'Open File'
          }
        ]">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-4 fill-black dark:fill-white ml-3">
            <path d="M0 96C0 78.33 14.33 64 32 64H416C433.7 64 448 78.33 448 96C448 113.7 433.7 128 416 128H32C14.33 128 0 113.7 0 96zM0 256C0 238.3 14.33 224 32 224H416C433.7 224 448 238.3 448 256C448 273.7 433.7 288 416 288H32C14.33 288 0 273.7 0 256zM416 448H32C14.33 448 0 433.7 0 416C0 398.3 14.33 384 32 384H416C433.7 384 448 398.3 448 416C448 433.7 433.7 448 416 448z"/>
          </svg>
        </OverflowMenu>
      </div>
      <ul class="topbar text-sm font-medium text-center flex tabs pl-3 pt-0.5 select-none gap-0.5">
        <li class="relative w-31 h-10 flex gap-0.5 rounded-t-lg items-center justify-center hover:text-gray-300 tab-item" v-ripple :selected="tab.id == currentTabId" v-for="(tab,i) in tabsRef.slice(0, maxTabsDisplay.maxTabs)" :key="tab.id" @click="e => changeTabSelection(e,tab.id)">
          <Tooltip :tip="tab.props.text.filePath?.split('/')?.last() || tab.name" contentContainerClass="overflow-hidden" class="min-h-full min-w-full">
            <div class="flex items-center justify-center gap-2">
              <div class="unfinished" v-if="tab.props.text.showIcon">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-3 h-3 fill-green-400">
                    <path d="M512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256z"/>
                </svg>
              </div>
              <a href="javascript:void(0);" draggable="false" class="tab-title cursor-default whitespace-nowrap dark:hover:text-gray-300" :data-text="tab.props.text.filePath?.split(/[\\\/]/g)?.last() || tab.name"></a>
            </div>
          </Tooltip>
          <div class="close-btn absolute right-3.5 w-2 top-1">
            <Tooltip :tip="`Close File ${tab.id == currentTabId ? '(Ctrl+W)': ''}`">
              <button @click="closeTab(i, tab)" v-ripple class="p-2 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-full cursor-default close"/>
            </Tooltip>
          </div>
        </li>
      </ul>
      <Tooltip tip="New File" class="ml-3" @click="onNewTab()">
        <button class="px-3 py-2.5 mt-0.5 rounded-full hover:bg-orange-200 dark:hover:bg-zinc-700 cursor-default fill-black dark:fill-white" v-ripple>
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4" viewBox="0 0 448 512"><path d="M432 256c0 17.69-14.33 32.01-32 32.01H256v144c0 17.69-14.33 31.99-32 31.99s-32-14.3-32-31.99v-144H48c-17.67 0-32-14.32-32-32.01s14.33-31.99 32-31.99H192v-144c0-17.69 14.33-32.01 32-32.01s32 14.32 32 32.01v144h144C417.7 224 432 238.3 432 256z"/></svg>
        </button>
      </Tooltip>
      
      <OverflowMenu :menuItems="tabs.slice(maxTabsDisplay.maxTabs).map(tab => {
        return {
          id: tab.id,
          title: tab.name,
          onClick: () => overflowClick(tab.id)
        }
      })" className="px-3 py-2.5 mt-0.5 rounded-full hover:bg-orange-200 dark:hover:bg-zinc-700 cursor-default fill-black dark:fill-white" v-if="tabs.length > maxTabsDisplay.maxTabs">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4" viewBox="0 0 384 512"><path d="M192 384c-8.188 0-16.38-3.125-22.62-9.375l-160-160c-12.5-12.5-12.5-32.75 0-45.25s32.75-12.5 45.25 0L192 306.8l137.4-137.4c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25l-160 160C208.4 380.9 200.2 384 192 384z"/></svg>
      </OverflowMenu>
    </div>
    <div class="tab-components">
      <div class="component" v-for="tab in tabs" :key="tab.id" :class="tab.id != currentTabId ? 'hidden': ''">
        <component :is="tab.component" v-model="tab.props.text" v-bind="tab.props"></component>
      </div>
    </div>
  </div>
  
</template>

<style>
.menu-left {
  right: auto !important;
  left: 10px !important;
  white-space: nowrap !important;
}

.menu-overflow-left {
  margin-top: 13px;
}
</style>

<style scoped>
.w-31 {
  width: 7.6rem;
}
.tab-title::before {
  content: attr(data-text)
}
div.flex.w-full.text-gray-500.border-b.border-gray-200.dark\:text-gray-400.dark\:border-gray-700 {
  background: rgb(227 219 215);
}

.topbar {
  max-width: 90%;
}

.topbar li, .topbar li a {
  transition: all 0.4s ease-in-out;
}

.topbar::-webkit-scrollbar {
  display: none;
}

button.close {
    background-image: url("data:image/svg+xml, %3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 512'%3E%3Cpath d='M310.6 361.4c12.5 12.5 12.5 32.75 0 45.25C304.4 412.9 296.2 416 288 416s-16.38-3.125-22.62-9.375L160 301.3L54.63 406.6C48.38 412.9 40.19 416 32 416S15.63 412.9 9.375 406.6c-12.5-12.5-12.5-32.75 0-45.25l105.4-105.4L9.375 150.6c-12.5-12.5-12.5-32.75 0-45.25s32.75-12.5 45.25 0L160 210.8l105.4-105.4c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25l-105.4 105.4L310.6 361.4z' fill='%23000' /%3E%3C/svg%3E%0A");
  background-repeat: no-repeat;
  background-position-x: center;
  background-size: 10px;
}

@media (prefers-color-scheme: dark) {
  button.close {
      background-image: url("data:image/svg+xml, %3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 512'%3E%3Cpath d='M310.6 361.4c12.5 12.5 12.5 32.75 0 45.25C304.4 412.9 296.2 416 288 416s-16.38-3.125-22.62-9.375L160 301.3L54.63 406.6C48.38 412.9 40.19 416 32 416S15.63 412.9 9.375 406.6c-12.5-12.5-12.5-32.75 0-45.25l105.4-105.4L9.375 150.6c-12.5-12.5-12.5-32.75 0-45.25s32.75-12.5 45.25 0L160 210.8l105.4-105.4c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25l-105.4 105.4L310.6 361.4z' fill='%23fff' /%3E%3C/svg%3E%0A");
  }
}


ul>li[selected="true"] {
  background-color: white;
  color: black;
}

ul>li[selected="false"]:hover,
ul>li[selected="false"] a[href="javascript:void(0);"]:hover {
  background-color: rgb(234, 234, 234);
  color: #333;
}


@media (prefers-color-scheme: dark) {
  ul>li {
    color: gray;
  }

  ul>li[selected="true"] {
    background-color: rgb(63 63 70);
    color: rgb(229 231 235);
  }

  div.flex.w-full.text-gray-500.border-b.border-gray-200.dark\:text-gray-400.dark\:border-gray-700 {
    background: transparent;
  }

  ul>li[selected="false"]:hover,
  ul>li[selected="false"] a[href="javascript:void(0);"]:hover {
    background-color: rgb(39 39 42);
    color: gray;
  }
}
</style>