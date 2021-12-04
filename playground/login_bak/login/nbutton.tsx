import type {PropType} from 'vue'

import {defineComponent} from 'vue'
export default defineComponent({
	props: {
		value: {
			type: String,
			default: '',
		},
		type: String,
		onClick: Function as PropType<(ev: Event) => void>,
	},
	setup(props, {slots}) {
		return () => <button
			class="hover:(scale-103) "
			w:bg="gradient-to-r active:(gradient-to-l)"
			w:gradient="from-green-400 via-cyan-400 to-blue-400"
			w:opacity='90'
			w:border="~ rounded-1xl light-blue-400"
			w:text="14 white"
			w:outline="none focus:(none)"
			onClick={props.onClick}
		>
			<div
				w:flex="~ row"
				w:text="center"
				w:align="items-center"
				w:justify="center"
			>
				<span>
					{slots.default ? slots.default() : null}
				</span>
				<span>
					{props.value}
				</span>
			</div>
		</button>

	},
})
