<script lang="ts">
  import { computed, onMounted, ref, watch } from '@vue/runtime-core'
  import { Ref, inject, Component, markRaw } from 'vue';
  export default {
    inheritAttrs: false
  }
</script>

<script lang="ts" setup>
  import { EmojiButton } from '@joeattardi/emoji-button';
  import { dialog, clipboard, fs } from '@tauri-apps/api'
  import FontOverlayVue from './FontOverlay.vue';

  const fontSize = inject<Ref<number>>('fontSize')

  const { id, modelValue } = defineProps<{
    id: string,
    modelValue: {
      text: string,
      showIcon: boolean,
      filePath?: string,
      initialText: string,
    }
  }>()


  const emit = defineEmits(['update:modelValue'])
  const text = computed({
    get: () => modelValue.text,
    set: (value) => {
      modelValue.text = value
      emit('update:modelValue', modelValue)
    }
  })

  const filePath = computed({
    get:() => modelValue.filePath,
    set: (value) => {
      modelValue.filePath = value;
      emit('update:modelValue', modelValue)
    }
  })

  const showIcon = computed({
    get: () => modelValue.showIcon,
    set: (value) => {
      modelValue.showIcon = value;
      emit('update:modelValue', modelValue)
    }
  })

  const initialText = computed({
    get: () => modelValue.initialText,
    set: (value) => {
      modelValue.initialText = value;
      emit('update:modelValue', modelValue)
    }
  })


  const {currentTabId} = inject<{
    currentTabId: Ref<string>
  }>('tabs')!
  
  const textarea = ref(null as HTMLTextAreaElement | null)

  watch(currentTabId, () => {
    if (id == currentTabId.value) {
      console.log('Yes Hi')
      if (document.activeElement instanceof HTMLElement)
        document.activeElement.blur();

      window.focusElement(textarea.value!)
    }
  })

  const contextMenu = ref(null as HTMLDivElement | null)


  function hideMenu() {
    if (contextMenu.value != null) {
      contextMenu.value.style.display = 'none'
    }
  }

  const picker = new EmojiButton({
    theme: 'auto'
  });
  picker.on('emoji', ({emoji}) => {
    text.value += emoji;
  });
  

  function rightClick(e: MouseEvent) {
    e.preventDefault();

    if (contextMenu.value?.style.display == "block")
      hideMenu();
    else {
      if (contextMenu.value){        
        contextMenu.value!.style.display = 'block';
        contextMenu.value!.style.left = e.pageX + "px";
        contextMenu.value!.style.top = e.pageY + "px";
      }
    }
  }
  const isCopyOrCutAvailable = ref(false)
  const isPasteAvailable = ref(false)

  const checkPasteAction = () => {
    clipboard.readText().then(text => {
      if (text == null) {
        isPasteAvailable.value = false
      } else {
        isPasteAvailable.value = text.length > 0
      }
    }).catch(() => isPasteAvailable.value = false)
  }


  function insertAtCursor(myField: HTMLTextAreaElement, myValue: string) {
     if (myField.selectionStart || myField.selectionStart == 0) {
          var startPos = myField.selectionStart;
          var endPos = myField.selectionEnd;
          myField.value = myField.value.substring(0, startPos)
              + myValue
              + myField.value.substring(endPos, myField.value.length);
          text.value = myField.value
      } else {
          myField.value += myValue;
          text.value = myField.value
      }
  }

  const saveFile = async () => {
    if (!filePath.value) {      
      filePath.value = await dialog.save({
        filters: [{
          extensions: ['s', 'par', 'c', 'css', 'csv', 'curl', 'dcurl', 'mcurl', 'scurl', 'flx', 'f', 'gv', 'html', 'ics', '3dml', 'spot', 'jad', 'java', 'json', 'fly', 'n3', 'p', 'dsc', 'rtx', 'etx', 'sgml', 'tsv', 'txt', 't', 'ttl', 'uri', 'uu', 'vcs', 'vcf', 'wml', 'wmls', 'yaml', '*'],
          name: ''
        }],
        title: 'Untitled.txt'
      })
      document.title = filePath.value.split(/[\/\\]/g).last() || 'Untitled'
    }

    const txt = text.value
    
    await fs.writeFile({
      contents: txt,
      path: filePath.value
    })

    initialText.value = txt
    showIcon.value = text.value != initialText.value;
  }

  const handleKeys = (e: KeyboardEvent) => {
    if (e.ctrlKey && (e.key == 's' || e.code == 'KeyS' || e.keyCode == 83) && currentTabId.value == id) {
      e.preventDefault()
      saveFile()
    }
  }

  watch(text, () => {
    isCopyOrCutAvailable.value = text.value.length > 0 && textarea.value?.selectionStart != textarea.value?.selectionEnd;
    console.log(text.value)
    showIcon.value = text.value != initialText.value;
  })

  onMounted(() => {
    if (id == currentTabId.value) { 
      textarea.value?.focus()
    }

    window.addEventListener('keydown', handleKeys)
    checkPasteAction()
    setInterval(checkPasteAction, 2000)
    isCopyOrCutAvailable.value = text.value.length > 0 && textarea.value?.selectionStart != textarea.value?.selectionEnd
    document.addEventListener('click', hideMenu)
    document.addEventListener('selectionchange', () => {
      isCopyOrCutAvailable.value = text.value.length > 0 && document.activeElement == textarea.value && textarea.value?.selectionStart != textarea.value?.selectionEnd;  
    })
  })

  const pasteText = () => {
    clipboard.readText().then(text => {
      if (textarea.value != null && text != null) {
        insertAtCursor(textarea.value, text)
      }
    })
  }

  const cutText = () => {
    if (isCopyOrCutAvailable.value) {
      const textCut = text.value.slice(textarea.value?.selectionStart || 0, textarea.value?.selectionEnd || 0)
      clipboard.writeText(textCut)
      text.value = text.value.slice(0, textarea.value?.selectionStart || 0) + text.value.slice(textarea.value?.selectionEnd);
    }
  }

  const copyText = () => {
    if (isCopyOrCutAvailable.value) {
      const textCopy = text.value.slice(textarea.value?.selectionStart || 0, textarea.value?.selectionEnd || 0)
      clipboard.writeText(textCopy)
    }
  }

  const {showOverlay} = inject<{
    showOverlay: (component: Component) => void
  }>('overlay')!


  const emojiBtn = ref(null as HTMLAnchorElement | null);

  const openEmoji = () => {
    picker.isPickerVisible() && picker.hidePicker();
    picker.showPicker(emojiBtn.value!);
  }

  const changeFontSize = () => {
    showOverlay(markRaw(FontOverlayVue))
  }


</script>

<template>
  <textarea @contextmenu="rightClick" v-model="text" tabindex="0" ref="textarea" class="cursor-auto dark:bg-stone-800 bg-opacity-50 resize-none w-full p-2 outline-none" spellcheck="false" :style="`font-size: ${fontSize}px`"></textarea>

  <div id="contextMenu" ref="contextMenu" class="fill-black dark:fill-white context-menu select-none bg-slate-200 dark:bg-slate-700 text-black dark:text-white" 
  style="display:none">
    <ul tabindex="-1">
        
        <li disabled="false" ref="emojiBtn" @click="openEmoji">
          <a href="javascript:void(0);" tabindex="-1" draggable="false">
            
          <svg class="w-4 fill-black dark:fill-white" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d='M12,1.73A10.27,10.27,0,1,0,22.24,12,10.25,10.25,0,0,0,12,1.73ZM21,12a9,9,0,1,1-9-9A9,9,0,0,1,21,12Z' />
            <path d='M8.8,11.05a1.55,1.55,0,1,0-1.51-1.5A1.56,1.56,0,0,0,8.8,11.05Z' />
            <path d='M15.44,11.05a1.55,1.55,0,1,0,0-3.09,1.53,1.53,0,0,0-1.51,1.59A1.51,1.51,0,0,0,15.44,11.05Z' />
            <path d='M12.19,16.35A6.58,6.58,0,0,1,6.9,13.5a5.71,5.71,0,0,0,5.3,4,5.54,5.54,0,0,0,5.31-4A6.27,6.27,0,0,1,12.19,16.35Z' />
          </svg>
          <span data-text="Emoji"></span>

          </a>
        </li>
        
        <li :disabled="!isCopyOrCutAvailable" @click="copyText">
          <a href="javascript:void(0);" tabindex="-1" draggable="false"><svg xmlns="http://www.w3.org/2000/svg" class="w-4 fill-black dark:fill-white" viewBox="0 0 512 512"><path d="M384 96L384 0h-112c-26.51 0-48 21.49-48 48v288c0 26.51 21.49 48 48 48H464c26.51 0 48-21.49 48-48V128h-95.1C398.4 128 384 113.6 384 96zM416 0v96h96L416 0zM192 352V128h-144c-26.51 0-48 21.49-48 48v288c0 26.51 21.49 48 48 48h192c26.51 0 48-21.49 48-48L288 416h-32C220.7 416 192 387.3 192 352z"/></svg>Copy</a>
        </li>

        <li :disabled="!isCopyOrCutAvailable" @click="cutText">
          <a href="javascript:void(0);" tabindex="-1" draggable="false">            
            <svg class="w-4 fill-black dark:fill-white" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M7.8294 2.43983C7.52083 1.98179 6.89937 1.86062 6.44132 2.16919C5.98328 2.47776 5.86211 3.09922 6.17068 3.55726L10.9634 10.6715L8.63676 14.3491C8.13715 14.1248 7.58315 14 7 14C4.79086 14 3 15.7909 3 18C3 20.2091 4.79086 22 7 22C9.20914 22 11 20.2091 11 18C11 17.1033 10.705 16.2756 10.2067 15.6085L12.1839 12.4832L14.0676 15.2795C13.4051 15.9932 13 16.9493 13 18C13 20.2091 14.7909 22 17 22C19.2091 22 21 20.2091 21 18C21 15.7909 19.2091 14 17 14C16.5639 14 16.1441 14.0698 15.7511 14.1988L12.1319 8.82451L12.1313 8.8255L7.8294 2.43983ZM5 18C5 16.8954 5.89543 16 7 16C8.10457 16 9 16.8954 9 18C9 19.1046 8.10457 20 7 20C5.89543 20 5 19.1046 5 18ZM15 18C15 16.8954 15.8954 16 17 16C18.1046 16 19 16.8954 19 18C19 19.1046 18.1046 20 17 20C15.8954 20 15 19.1046 15 18Z"/>
            <path d="M14.5202 8.79027L17.8452 3.53461C18.1404 3.06789 18.0014 2.45017 17.5347 2.15489C17.068 1.85962 16.4503 1.99861 16.155 2.46534L13.2998 6.97841L14.5202 8.79027Z"/>
            </svg>
            <span data-text="Cut"></span>
          </a>
        </li>
        <li :disabled="!isPasteAvailable" @click="isPasteAvailable && pasteText()">
          <a href="javascript:void(0);" tabindex="-1" draggable="false">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-4 fill-black dark:fill-white"><path d="M320 96V80C320 53.49 298.5 32 272 32H215.4C204.3 12.89 183.6 0 160 0S115.7 12.89 104.6 32H48C21.49 32 0 53.49 0 80v320C0 426.5 21.49 448 48 448l144 .0013L192 176C192 131.8 227.8 96 272 96H320zM160 88C146.8 88 136 77.25 136 64S146.8 40 160 40S184 50.75 184 64S173.3 88 160 88zM416 128v96h96L416 128zM384 224L384 128h-112C245.5 128 224 149.5 224 176v288c0 26.51 21.49 48 48 48h192c26.51 0 48-21.49 48-48V256h-95.99C398.4 256 384 241.6 384 224z"/></svg>
            <span data-text="Paste"></span>
          </a>
        </li>

        
       <li disabled="false" @click="changeFontSize">
          <a href="javascript:void(0);" tabindex="-1" draggable="false">
            <svg version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
                viewBox="0 0 496.188 496.188" class="w-4 fill-black dark:fill-white" style="enable-background:new 0 0 496.188 496.188;" xml:space="preserve">
              <g>
                <path d="M196.698,108.246c-5.006-16.197-19.999-27.241-36.96-27.241h-41.996c-17.006,0-32.037,11.116-37.01,27.396L1.629,367.257
                  c-3.45,11.247-1.355,23.452,5.657,32.902c6.992,9.447,18.05,15.025,29.811,15.025c16.67,0,31.287-11.116,35.747-27.167
                  l16.263-58.612h95.689l17.514,58.246c4.91,16.345,19.953,27.533,37.051,27.533c12.307,0,23.871-5.846,31.156-15.742
                  c7.301-9.904,9.448-22.679,5.809-34.42L196.698,108.246z M100.018,272.891l19.808-70.913c5.446-19.337,10.426-44.609,15.398-64.451
                  h0.946c4.974,19.842,10.959,44.625,16.881,64.451l20.832,70.913H100.018z"/>
                <path d="M495.093,362.394v-69.026c0-46.676-21.128-85.658-87.331-85.658c-24.577,0-45.017,4.506-59.917,9.814
                  c-9.902,3.531-15.35,14.109-12.473,24.217l0.197,0.665c1.511,5.319,5.156,9.783,10.065,12.352c4.895,2.571,10.62,3.025,15.87,1.253
                  c10.85-3.661,23.292-6.082,35.309-6.082c30.462,0,36.154,15.028,36.154,25.582v2.853c-70.256-0.415-116.538,24.346-116.538,75.915
                  c0,31.68,23.953,60.897,64.171,60.897c23.534,0,43.813-8.538,56.824-24.37h1.198l1.596,8.685
                  c1.207,6.496,6.864,11.221,13.482,11.221h28.767c3.824,0,7.467-1.587,10.054-4.384c2.602-2.797,3.919-6.554,3.626-10.358
                  C495.4,386.195,495.093,374.461,495.093,362.394z M434.967,338.44c0,3.649-0.389,7.311-1.218,10.554
                  c-4.053,12.588-16.639,22.736-32.072,22.736c-13.792,0-24.36-7.71-24.36-23.541c0-23.955,25.188-31.664,57.65-31.267V338.44z"/>
              </g>
              </svg>
            <span class="text-sm" data-text="Change Font Size"></span>
          </a>
        </li>

        
    </ul>
  </div>

</template>

<style scoped>
textarea::selection {
  background-color: gray;
  color: white;
}

textarea {
  overflow-x: scroll;
  white-space: nowrap;
  height: 92vh;
  background-color: white;
}

.context-menu {
  position: absolute;
  overflow-x: hidden;
  border-radius: 10px;
  text-align: center;
  /* background: white; */
  border: 1px solid black;
}

.context-menu ul {
  padding: 0px;
  margin: 0px;
  min-width: 150px;
  list-style: none;
}

.context-menu ul li {
  padding-bottom: 7px;
  padding-top: 7px;
  border-bottom: 1px solid black;
  border-top: 1px solid black;
}
.context-menu ul li[disabled="true"] {
  opacity: 0.3;
}


.context-menu ul li a {
  text-decoration: none;
  /* color: black; */
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 5px;
}
.context-menu ul li a span::before {
  content: attr(data-text);
}

.context-menu ul li[disabled="true"], .context-menu ul li[disabled="true"] a {
  cursor: not-allowed;
}

.context-menu ul li[disabled="false"]:hover {
  background: darkgray;
}

textarea::-webkit-scrollbar-track {
  box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  /* background-color: #fff; */
  border-radius: 12px;
}

::-webkit-scrollbar {
  cursor: pointer !important;
}

textarea::-webkit-scrollbar { 
  cursor: pointer;
  width: 12px;
}

textarea::-webkit-scrollbar-thumb {
  background-color: #AAA;
  border-radius: 12px;
  background-image: -webkit-linear-gradient(90deg,
      rgba(0, 0, 0, .3) 25%,
      transparent 25%,
      transparent 50%,
      rgba(0, 0, 0, .3) 50%,
      rgba(0, 0, 0, .3) 75%,
      transparent 75%,
      transparent
  )
}

  textarea::-webkit-scrollbar:horizontal {
    height: 10px;
    background-color: transparent;
  }

  textarea::-webkit-scrollbar-track:horizontal {
    background-color: transparent;
  }

  textarea::-webkit-scrollbar-thumb:horizontal {
    background-image: -webkit-linear-gradient(180deg,
        rgba(0, 0, 0, .3) 25%,
        transparent 25%,
        transparent 50%,
        rgba(0, 0, 0, .3) 50%,
        rgba(0, 0, 0, .3) 75%,
        transparent 75%,
        transparent
    )
  }


@media (prefers-color-scheme: dark) {
  textarea::selection {
    background-color: #333;
    color: white;
  }

  textarea {
    --tw-bg-opacity: 1;
    background-color: rgb(20, 18, 17);
  }

  textarea::-webkit-scrollbar {
    background-color: #363636;
  }


  textarea::-webkit-scrollbar-track {
    background-color: #363636;
  }


  textarea::-webkit-scrollbar-thumb {
    background-image: -webkit-linear-gradient(90deg,
        rgba(0, 0, 0, .5) 25%,
        gray 25%,
        gray 50%,
        rgba(0, 0, 0, .5) 50%,
        rgba(0, 0, 0, .5) 75%,
        gray 75%,
        gray)
  }
  
  textarea::-webkit-scrollbar-thumb:horizontal {
    background-image: -webkit-linear-gradient(180deg,
        rgba(0, 0, 0, .5) 25%,
        gray 25%,
        gray 50%,
        rgba(0, 0, 0, .5) 50%,
        rgba(0, 0, 0, .5) 75%,
        gray 75%,
        gray)
  }

}
</style>