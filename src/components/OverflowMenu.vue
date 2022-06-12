<script lang="ts">
export default {}
</script>

<script lang="ts" setup>
  import { ref } from 'vue'
  const {menuItems, className, menuClassName} = defineProps<{
    menuItems: {
      title: string,
      id: string,
      onClick: () => void
    }[],
    className?: string,
    menuClassName?: string
  }>()

  const isOverflowOpen = ref(false)
</script>

<template>
  <button :class="className" @click="isOverflowOpen = !isOverflowOpen" v-ripple>
    <slot/>
  </button>
  <div :class="`menu pt-1 rounded-md z-40 overflow-x-hidden overflow-y-auto flex flex-col items-center max-h-72 absolute top-11 right-10 bg-orange-200 dark:bg-stone-600 text-black dark:text-white ${menuClassName || ''}`" v-if="isOverflowOpen">
    <div class="menu-item w-full hover:bg-orange-300 dark:hover:bg-stone-700 text-center border-b border-slate-800" v-ripple v-for="menuItem in menuItems" :key="menuItem.id">
      <button class="mx-2" @click="() => {
        menuItem.onClick()
        isOverflowOpen = false
      }"
      :data-text="menuItem.title">
        
      </button>
    </div>
  </div>
</template>


<style scoped>
.menu-item {

  min-width: 80px;
}

.menu-item button::before {
  content: attr(data-text)
}
</style>