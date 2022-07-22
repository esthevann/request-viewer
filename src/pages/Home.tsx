import { useState } from 'react'
import { invoke, http } from '@tauri-apps/api'
import Request from '../components/Request';


function Home() {

  return (
    <div className='flex flex-col gap-3 mt-3'>
      <div className="flex flex-col items-center gap-3">
        <h1 className='text-3xl font-bold '>Request maker</h1>
        <label htmlFor="" className='flex gap-2'>
          Name:
          <input type="text" className='border' spellCheck={false} />
        </label>
        <button className='bg-blue-500 text-white px-3 py-2 rounded-full'> New Request </button>
      </div>

      <div className='flex flex-col ml-3 gap-3'>
        <h2 className='text-2xl font-bold'>My Requests</h2>
        <Request name={"Test request"} address={"www.google.com"}/>
        
      </div>
    </div>

  )
}

export default Home
