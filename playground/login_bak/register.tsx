import { $fetch } from 'ohmyfetch'
import IAlien from 'virtual:icons/mdi/alien-outline'
import IEmail from 'virtual:icons/mdi/email'
import IPassword from 'virtual:icons/ri/lock-password-line'
import IKey from 'virtual:icons/wpf/password1'
import { defineComponent, reactive} from 'vue'

import Link from '~/components/link'
import NButton from '~/components/login/nbutton'
import NInput from '~/components/login/ninput'
import { useI18n } from '~/plugins/i18n'
export default defineComponent({

	setup() {
		const {i18n} = useI18n()
		const input = reactive({
			username: '',
			password: '',
			password2: '',
			email: '',
			sixinput: '',
		})
		const phoneReg = /^[1][3,5,7,8][0-9]{9}$/;
		const emailReg = /^[A-Za-z0-9\u4e00-\u9fa5]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$/
		
		return () => {
			const {button, id, email, passwd, ensurepasswd, sixinput, hintinfo} = i18n.value.login
			return <div
				w:w="100"
				w:p="4"
				w:border="1 rounded"
			>
				<div w:text="center"><span w:text="2xl stroke-1">{button.register}</span></div>
				<div
					w:flex="~ col"
					w:w="full"
					w:align="items-center"
					w:justify="around"
					w:children='p-1 w-full'
				>
					<div><span>{id.label}</span></div>
					<NInput type="text" 
						value={input.username}
						placeholder={id.placeholder}
						onChange={(e) => {
							input.username = e
						}}
					>
						{{
							icon: () => <IAlien/>
						}}
					</NInput>
					<div><span>{passwd.lable}</span></div>
					<NInput type="text"
						value={input.password}
						placeholder={passwd.placeholder} 
						onChange={(e) => {
							input.password = e
						}}
					>
						{{
							icon: () => <IPassword/>
						}}
					</NInput>
					<div><span>{ensurepasswd.lable}</span></div>
					<NInput type="text" 
						value={input.password2}
						placeholder={ensurepasswd.placeholder} 
						onChange={(e) => {
							input.password2 = e
						}}
					>
						{{
							icon: () => <IKey/>
						}}
					</NInput>
					<span>{email.label}</span>
					<NInput 
						value={input.email}
						type="text" 
						placeholder={email.placeholder}
						onChange={(e) => {
							input.email = e
						}}
					>
						{{
							icon: () => <IEmail/>,
							rightbutton: () => <button w:w='25' 
								w:outline="none focus:(none)"
								w:border="~ l-green-400 r-0 t-0 b-0"
								w:p='l-2'
							>{button.sendcode}</button>
						}}
					</NInput>
					<span>{sixinput.label}</span>
					<NInput
						value={input.sixinput}
						class="tracking-2.5em"
						w:bg="digit sz-digit repeat-x bottom"
						onChange={e => input.sixinput = e}
					/>
					<NButton 
						type="button" 
						value={button.register}
						onClick={async () => {
							const body = {
								username: input.username,
								passwd: input.password,
								email: input.email
							}
							const res = await $fetch('/api/user/signup', {
								method: 'POST',
								body
							})
							console.log(res);
						}}
					/>
				</div>
				<div w:text="center">
					<span w:text="xs gray-500">{hintinfo.guide.pushlogin}</span>
					<span w:text='xs green-800' w:cursor='pointer'><Link to='/login/lpassword'>Login</Link></span>
				</div>
			</div>
		}
	}
})



