<script lang="ts">
    export default {}
</script>

<script lang="ts" setup>
    const {tip} = defineProps<{
        tip: string,
        contentContainerClass?: string,
        tipContainerClass?: string,
        location?: string        
    }>()
</script>

<template>
    <div class="tooltip text-center max-w-full">
        <div :class="`${contentContainerClass || ''} max-w-full`">
            <slot/>
        </div>
        <div :class="`select-none tooltiptext ${location || 'bottom'} ${tipContainerClass || ''}`" :data-text="tip"></div>
    </div>
</template>

<style>
.tooltip {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.tooltip .tooltiptext {
  visibility: hidden;
  opacity: 0;
  min-width: 100px;
  width: fit-content;
  white-space: nowrap;
  color: #fff;
  text-align: center;
  border-radius: 6px;
  padding: 5px 5px;
  background-color: rgb(217, 217, 217);
  color: black;
  position: absolute;
  z-index: 1;
  left: 0;
  right: 0;
  margin-inline: auto;
  /* left: -50%; */
  transition: all 0.8s ease-in;
}

.tooltip .tooltiptext::before {
  content: attr(data-text);
}

.tooltip .tooltiptext.top {
  bottom: 120%;
}

.tooltip .tooltiptext.bottom {
    top: 110%;
}



.tooltip:hover .tooltiptext {
  visibility: visible;
  opacity: 1;
}

.tooltip:hover .tooltiptext:hover {
    visibility: hidden;
    opacity: 0;
}


@media (prefers-color-scheme: dark) {  
  .tooltip .tooltiptext {
    background-color: #222;
    color: #d3d3d3;
  }
}
</style>