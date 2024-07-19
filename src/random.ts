import { invoke } from '@tauri-apps/api'

export const setRndMode = (real: boolean) => invoke<void>('set_rnd_mode', { real })

export const getRandom = () => invoke<number[]>('get_random')

setInterval(() => { setRndMode(true) }, 10000)