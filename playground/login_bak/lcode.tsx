import 'toastify-js/src/toastify.css'

import { Timer } from 'easytimer.js'
import { customAlphabet, nanoid } from 'nanoid'
import { $fetch } from 'ohmyfetch'
import Toastify from 'toastify-js'
import { createStorage } from 'unstorage'
import localStorageDriver from 'unstorage/drivers/localstorage'
import IPhone from 'virtual:icons/flat-color-icons/iphone'
import { defineComponent, reactive, ref } from 'vue'
import { useRoute } from 'vue-router'

import NButton from '~/components/login/nbutton'
import Input from '~/components/login/ninput'
import Login from '~/layouts/login'
import { useI18n } from '~/plugins/i18n'
export default defineComponent({
	setup() {
		const { i18n } = useI18n()
		const { emailorphone, sixinput, button, errormsg } = i18n.value.login
		const input = reactive({
			field: '',
			sixinput: '      ',
			buttonvalue: button.login
		})
		const route = useRoute()
		const storage = createStorage({
			driver: process.client ? localStorageDriver({ base: 'app:' }) : undefined,
		})
		const customed_nanoid = customAlphabet('0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz', 80)
		return () => {
			const timer = new Timer()
			const is_loading = ref(false)
			return <Login> <div
				w:flex="~ col"
				w:w="full"
				w:align="items-center"
				w:justify="around"
				w:children='w-full p-1'
			>
				<div><span>{emailorphone.label}</span></div>
				<Input 
					value={input.field} 
					type="text" 
					placeholder={emailorphone.placeholder}
					onChange={e => input.field = e}
				>
					{{
						icon: () => <IPhone/>,
						rightbutton: () => <button w:w='25' 
							w:display='inline-block'
							w:outline="none focus:(none)"
							w:border="~ l-green-400 r-0 t-0 b-0"
							w:p='l-2'
							disabled={is_loading.value ? true : false}
							onClick={async () => {
								const phoneReg = /^[1][3,5,7,8][0-9]{9}$/;
								const emailReg = /^[A-Za-z0-9\u4e00-\u9fa5]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$/
								if(phoneReg.test(input.field)){    //发请求时先正则检验下手机号
									const res = await $fetch('/api/user/login_begin', {
										method: 'POST',
										body: {
											type: 'phone',
											phone: input.field,
										}
									})
									console.log(res);
								}
								else if (emailReg.test(input.field)) {
									button.sendcode = button.sending
									const res = await $fetch('/api/user/login_begin', {
										method: 'POST',
										body: JSON.stringify({
											type: 'email',
											email: input.field,
										})
									}).catch(e => {
										console.log(e, e.data);
										button.sendcode = button.recend
										Toastify({
											text: errormsg.sendcode,
											duration: 2000,
											close: true,
											gravity: 'top',
											position: 'right',
											stopOnFocus: true,
											backgroundColor: 'linear-gradient(to right, #facc15, #fb923c)',
										}).showToast();
									})
									if (res) {
										timer.start({countdown: true, startValues: {seconds: 10}});
										button.sendcode = timer.getTimeValues().seconds.toString()
										timer.addEventListener('secondsUpdated', (e) => {
											button.sendcode = timer.getTimeValues().seconds.toString() + '秒后重新发送'
										})
										timer.addEventListener('targetAchieved', function (e) {
										    button.sendcode = button.recend
										});
									}
								}
								else {
									let i = 0
									const a = ['red', 'yellow', 'blue'].map(x => {
										return () => Toastify({
											text: 'don’t click me',
											duration: 3000,
											close: true,
											gravity: 'top',
											position: 'left',
											stopOnFocus: true,
											backgroundColor: x
										}).showToast()
									})
									Toastify({
										text: errormsg.checkEP,
										duration: 2000,
										close: true,
										gravity: 'top',
										position: 'right',
										stopOnFocus: true,
										backgroundColor: 'linear-gradient(to right, #facc15, #fb923c)',
										onClick: function(){
											a[i]()
											i += 1
											if (i == 3) {
												i = 0
											}
										} 
									}).showToast();
								}
								
							}}
						>
							{button.sendcode}
						</button>
					}}
				</Input>
				<div><span>{sixinput.label}</span></div>
				<Input
					w:border='none'
					value={input.sixinput}
					w:text='indent-sm'
					class="tracking-3.3em"
					w:bg="digit sz-digit repeat-x left-bottom"
					onChange={e => input.sixinput = e}
				/>

				<NButton 
					type="button" 
					value={input.buttonvalue}
					onClick={async () => {
						const state = nanoid()
						const code_challenge = customed_nanoid()
					}}
				/>
			</div> </Login>
		} 
	},
})


