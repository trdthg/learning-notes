import type { PropType } from 'vue'

import { defineComponent } from 'vue'
export default defineComponent({
	props: {
		value: {
			type: String,
			required: true,
		},
		type: {
			type: String,
			default: 'text',
		},
		placeholder: {
			type: String,
			default: '',
		},
		border: String,
		'class': String,
		'w:bg': String,
		'w:gradient': String,
		'w:text': String,
		'w:maxlength': String,
		'w:p': String,
		onChange: Function as PropType<(ev: string) => void>,
	},
	setup(props, {slots}) {
		return () => {
			return <div
				w:flex="~ row"
				w:border={ props.border ? props.border : '~ rounded hover:(green-300) focus:(2)'}
				w:transform="duration-200 hover:(border-green-300)"
				w:align="items-center"
			>
				<span>{slots.icon ? slots.icon() : null}</span>
				<input
					w:w='full'
					w:p={props['w:p']}
					maxlength={props['w:maxlength']}
					w:text={props['w:text']}
					w:outline="none"
					type={props.type}
					value={props.value}
					placeholder={props.placeholder}
					class={props['class']}
					w:bg={props['w:bg']}
					w:gradient={props['w:gradient']}
					onChange={e => props.onChange?.((e.target as HTMLInputElement).value)}
				/>
				<span>
					{slots.rightbutton ? slots.rightbutton() : null}
				</span>
			</div>
		}
	},
})
