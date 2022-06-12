import { createApp } from 'vue'
import App from './App.vue'
import Ripple from 'vue3-whr-ripple-directive'
import './index.css'
import SeekbarVue from './components/Seekbar.vue'


// Note: The ripple directive will work better if the element where you attach it is relative positioned

declare global {
    interface Array<T> {
        last: () => T | undefined
    }
    interface Window { focusElement: (element: HTMLElement) => void; }
}

Array.prototype.last = function () {
    return this.at(this.length-1)
}

window.focusElement = window.focusElement || ((element) => {
    // element.focus()
    document['focusElement']!(element)
})

createApp(App).component('v-seekbar', SeekbarVue).directive('ripple', Ripple).mount('#app')
