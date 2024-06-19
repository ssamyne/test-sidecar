import { useEffect, useLayoutEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
// import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

import { Command } from '@tauri-apps/api/shell'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')
  const [count, setCount] = useState<string>('0')

  async function greet() {
    const command = Command.sidecar('bin/python/test')
    const output = await command.execute()
    const { stdout } = output

    setGreetMsg(`Hello ${stdout} ${name}`)
  }

  useLayoutEffect(() => {
    async function callTestAppHandle() {
      try {
        await listen<Payload>('event-name', (event) => {
          setCount(event.payload.message)
        })
      } catch (error) {
        console.error('Failed to invoke command:', error)
      }
    }
    callTestAppHandle()
  }, [])

  const openThread = async () => {
    await invoke('init_process', {
      serialPort: 'COM1',
      baudRate: '9600',
      byteSize: '8',
      parity: '10',
      stopBits: '9',
      timeout: '1000',
    })
  }

  const stopThread = async () => {
    await invoke('stop_process')
  }
  // same type as payload
  type Payload = {
    message: string
  }

  async function startSerialEventListener() {
    console.log('run')
  }

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke('greet', { name }))
  // }

  return (
    <div className='container'>
      <h1>Welcome to Tauri!</h1>

      <div className='row'>
        <a href='https://vitejs.dev' target='_blank'>
          <img src='/vite.svg' className='logo vite' alt='Vite logo' />
        </a>
        <a href='https://tauri.app' target='_blank'>
          <img src='/tauri.svg' className='logo tauri' alt='Tauri logo' />
        </a>
        <a href='https://reactjs.org' target='_blank'>
          <img src={reactLogo} className='logo react' alt='React logo' />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <p>app online for : {count}</p>
      <form
        className='row'
        onSubmit={(e) => {
          e.preventDefault()
          greet()
        }}
      >
        <input
          id='greet-input'
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder='Enter a name...'
        />
        <button type='submit'>Greet</button>
      </form>
      <button onClick={() => openThread()}>start</button>
      <button onClick={() => stopThread()}>stop</button>
      <p>{greetMsg}</p>
    </div>
  )
}

export default App
