<script lang="ts" setup>
  import { computed, markRaw, onMounted, provide, reactive, readonly, ref, watch, watchEffect } from "vue";
  import type {Component} from 'vue'
  import TabItemVue from "./components/TabItem.vue";
  import Tabs from "./components/Tabs.vue";
  import {nanoid} from 'nanoid'
  import { dialog, fs } from "@tauri-apps/api";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from '@tauri-apps/api/tauri'

  const maxTabsDisplay = reactive({
    maxTabs: 0
  });

  const tabs = ref<
    {
      name: string;
      id: string;
      onClose: (index: number, id: string) => void;
      component: Component;
      props: {
        [name: string]: any;
      };
    }[]
  >([]);

  const createTabIfEmpty = () => {
    if (tabs.value.length == 0) {
      addTab();
    }
  };

  const addTab = (initialText?: string, filePath?: string, addAtStart: boolean = false) => {
    const id = nanoid()
    const obj = {
      component: markRaw(TabItemVue),
      name: "Untitled",
      id,
      onClose: (index: number) => {
        const [tab] = tabs.value.splice(index, 1);
        if (tab.id == currentTabId.value) {
          if(index != tabs.value.length) {
            currentTabId.value = tabs.value[index]?.id || ''
          } else {
            currentTabId.value = tabs.value[index - 1]?.id || ''
          }
        }
        createTabIfEmpty();
      },
      props: {
        id,
        text: reactive({
          showIcon: false,
          text: initialText || '',
          initialText: initialText || '',
          filePath
        })
      },
    }
    if (addAtStart) {
      tabs.value.unshift(obj)
    } else {
      if ((tabs.value.length + 1) > maxTabsDisplay.maxTabs) {
        tabs.value.unshift(obj)
      } else {
        tabs.value.push(obj);
      }
    }
    currentTabId.value = id
  };

  
  listen('open-file', async e => {
    if (typeof e.payload === "object" && e.payload) {
      const text = await fs.readTextFile(e.payload["path"])
      addTab(text, e.payload["path"], true)
    }
  })

  const overlay = ref(null as HTMLDivElement | null)

  const overlayComponent = ref(null as Component | null)

  const showOverlay = (component: Component) => {
    overlayComponent.value = component
    overlay.value?.classList?.remove('hidden')
  }

  const hideOverlay = () => {
      overlayComponent.value = null
      overlay.value?.classList?.add('hidden')
  }


  const fontSize = ref(16)
  provide('fontSize', fontSize)

  provide('overlay', {
    showOverlay,
    hideOverlay
  })

  const currentTabId = ref<string>("");
  const calcMaxTabs = () => {
    maxTabsDisplay.maxTabs = parseInt(`${document.body.clientWidth / 128}`) - 1
  }

  const openFile = async () => {
    const pathE = await dialog.open({
      multiple: false,
      filters: [
        {
          extensions: ['s', 'par', 'c', 'css', 'csv', 'curl', 'dcurl', 'mcurl', 'scurl', 'flx', 'f', 'gv', 'html', 'ics', '3dml', 'spot', 'jad', 'java', 'json', 'fly', 'n3', 'p', 'dsc', 'rtx', 'etx', 'sgml', 'tsv', 'txt', 't', 'ttl', 'uri', 'uu', 'vcs', 'vcf', 'wml', 'wmls', 'yaml', '*'],
          name: ''
        }
      ]
    })
    const path = pathE instanceof Array ? pathE[0]: pathE
    if (path != null) {
      const txt = await fs.readTextFile(path)
      addTab(txt, path)
    }
  }

  provide('openFile', openFile)
  onMounted(() => {
    createTabIfEmpty()
    calcMaxTabs()
    document.title = tabs.value.find(tab => tab.id == currentTabId.value)?.props?.text?.filePath || 'Untitled'
    document.body.onresize = calcMaxTabs
    window.addEventListener('keydown', async (e) => {
      if (e.ctrlKey && (e.key == 'f' || e.code == 'KeyF' || e.keyCode == 70)) {
        if(overlayComponent.value != null || !overlay.value?.classList?.contains('hidden')) {
          e.preventDefault()
        }
      } else if (e.ctrlKey && (e.key == 'r' || e.code == 'KeyR' || e.keyCode == 82)) {
        e.preventDefault()
      } else if (e.ctrlKey && (e.key == 'o' || e.code == 'KeyO' || e.keyCode == 79)) {
        e.preventDefault()
        await openFile()
      }
    })

    window.addEventListener('load', () => {
      setTimeout(() => {
        invoke('close_splashscreen')
      }, 1000);
    })
  });

  watch(tabs, () => createTabIfEmpty());

  provide("tabs", {
    tabs,
    currentTabId,
    maxTabsDisplay: readonly(maxTabsDisplay),
    onNewTab: addTab,
  });

  watch(currentTabId, () => {
    document.title = tabs.value.find(tab => tab.id == currentTabId.value)?.props?.text?.filePath || 'Untitled'
  })
</script>


<template>
  <div ref="overlay" class="overlay w-full h-screen hidden">
    <component :is="overlayComponent"></component>
  </div>
  <div class="content">
    <Tabs />
  </div>
</template>