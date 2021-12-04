import 'toastify-js/src/toastify.css'

import {customAlphabet, nanoid} from 'nanoid'
import {$fetch} from 'ohmyfetch'
import queryString from 'query-string'
import Toastify from 'toastify-js'
import {createStorage} from 'unstorage'
import localStorageDriver from 'unstorage/drivers/localstorage'
import NLoadingCirclr from 'virtual:icons/eos-icons/loading'
import IAlien from 'virtual:icons/mdi/alien-outline'
import IPassword from 'virtual:icons/ri/lock-password-line'
import {defineComponent, reactive, ref} from 'vue'
import {useRoute} from 'vue-router'

import NButton from '~/components/login/nbutton'
import Input from '~/components/login/ninput'
import Login from '~/layouts/login'
import {useI18n} from '~/plugins/i18n'

enum LoginState {
	Idle,
	FromClient,
	FromSelf,
	SelfCode,
	ClientCode,
	SelfToken,
	ClientToken
}

export default defineComponent({
	setup() {
		const {i18n} = useI18n()
		const input = reactive({
			username: '',
			passwd: '',
		})
		const route = useRoute()
		const storage = createStorage({
			driver: process.client ? localStorageDriver({base: 'app:'}) : undefined,
		})
		const customed_nanoid = customAlphabet('0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz', 80)
		return () => {
			const is_loading = ref(false)
			const {idoremail, passwd, button, errormsg} = i18n.value.login
			let loginstate = LoginState.Idle
			if (route.query.response_type === 'code') {
				loginstate = LoginState.FromClient
			} else if (route.query.code) {
				const exchange_ff = async function () {
					const code_verifier = await storage.getItem('code_challenge')
					const res = await fetch('http://localhost:3000/api/oauth/token', {
						method: 'POST',
						body: JSON.stringify({
							grant_type: 'authorization_code',
							client_id: process.env['IO_OAUTH_WEB_ID'],
							redirect_uri: 'http://localhost:3000/login',
							code: route.query.code,
							code_verifier: code_verifier,
						})
					})
					console.log(await res.text());
				}
				exchange_ff()
			} else {
				loginstate = LoginState.FromSelf
			}
			return <Login> <div
				w:flex="~ col"
				w:align="items-center"
				w:justify="around"
				w:w='full'
				w:children='w-full p-1'
			>
				<div><span>{idoremail.label}</span></div>
				<Input type="text"
					value={input.username}
					placeholder={idoremail.placeholder}
					onChange={(e) => {
						input.username = e
					}}
				>
					{{
						icon: () => <IAlien />,
					}}
				</Input>
				<div><span>{passwd.lable}</span></div>
				<Input type="text"
					border='none'
					class="text-indent:50px;"
					value={input.passwd}
					placeholder={passwd.placeholder}
					onChange={(e) => {
						input.passwd = e
					}}
				>
					{{
						icon: () => <IPassword />,
					}}
				</Input>
				<NButton 
					class="transform duration-200 hover:scale-103"
					type="button"
					value={button.login}
					w:text="14 white"
					onClick={async () => {
						const state = nanoid()
						const code_challenge = customed_nanoid()
						await storage.setItem('code_challenge', code_challenge)
						is_loading.value = true
						const res = await $fetch('/api/user/login_begin', {
							method: 'POST',
							body: {
								type: 'passwd',
								username: input.username,
								passwd: input.passwd,
							}
						}).catch(e => {
							Toastify({
								text: errormsg.network,
								duration: 2000,
								close: true,
								gravity: 'top',
								position: 'right',
								stopOnFocus: true,
								backgroundColor: 'linear-gradient(to right, #facc15, #fb923c)',
							}).showToast();
							is_loading.value = false
						})
						if (res) {
							if (loginstate === LoginState.FromSelf) {
								const url = queryString.stringifyUrl({
									url: '/api/oauth/authorize', query: {
										response_type: 'code',
										client_id: process.env['IO_OAUTH_WEB_ID'],
										redirect_uri: 'http://localhost:3000/login',
										state,
										code_challenge,
										code_challenge_method: 'plain'
									}
								});
								window.location.href = url
							} else {
								const url = queryString.stringifyUrl({url: '/api/oauth/authorize', query: {
									response_type: 'code',
									client_id: process.env['IO_OAUTH_WEB_ID'],
									redirect_uri: route.query.redirect_url,
									state: route.query.state,
									code_challenge: route.query.code_challenge,
									code_challenge_method: 'plain'
								}});
								window.location.href = url
							}
						}
					}}>
					{is_loading.value ? <NLoadingCirclr/> : ''}
				</NButton>
			</div> </Login>
		}
	},
})
