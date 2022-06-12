<script lang="ts">
import { inject } from '@vue/runtime-core'
import { Ref } from 'vue'
export default {
    
}
</script>

<script lang="ts" setup>
    const {hideOverlay} = inject<{hideOverlay: () => void}>('overlay')!
    
    const fontSize = inject<Ref<number>>('fontSize')!

    const overlayClicked = (e: MouseEvent) => {
        if(e.target instanceof HTMLElement) {
            if (e.target.id == 'overlayContent') {
                hideOverlay()
            }
        } else {
            console.log('Hello')
        }
    }
</script>

<template>
    <div id="overlayContent" class="flex flex-col w-full h-full items-center justify-center" @click="overlayClicked">
        <span :style="`font-size: ${fontSize}px`" data-text="Example Text"></span>
        <v-seekbar v-model="fontSize"></v-seekbar>
    </div>
</template>

<style scoped>
    #overlayContent > span::before {
        content: attr(data-text);
    }
</style>