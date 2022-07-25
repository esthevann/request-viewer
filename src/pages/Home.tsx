import { FormEvent, FormEventHandler, useState } from 'react';
import { invoke, http } from '@tauri-apps/api';
import { useQueryClient, useQuery } from '@tanstack/react-query';
import Request from '../components/Request';
import RequestService from '../services/RequestService'

function Home() {
  const client = useQueryClient();

  const {data, isLoading} = useQuery(['requests'], RequestService.get_all_requests);

  const [requestName, setRequestName] = useState("");
  const handleSubmit: FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault(); 
    invoke("create_request_record", { args: { name: requestName, address: null } })
      .then(data => {
        console.log(data);
        client.invalidateQueries(['requests'])
      })
      .catch(e => console.log(e))
    
  }

  return (
    <div className='flex flex-col gap-3 mt-3'>
      <form onSubmit={handleSubmit} className="flex flex-col items-center gap-3">
        <h1 className='text-3xl font-bold '>Request maker</h1>
        <label htmlFor="" className='flex gap-2'>
          Name:
          <input type="text" value={requestName} onChange={(e) => setRequestName(e.target.value)} className='border' spellCheck={false} />
        </label>
        <button className='bg-blue-500 text-white px-3 py-2 rounded-full'> New Request </button>
      </form>

      <div className='flex flex-col ml-3 gap-3'>
        <h2 className='text-2xl font-bold'>My Requests</h2>
        {isLoading && <div>
            Loading
          </div>}
        {data && 
          <div>
            {data.map(request => (
              <Request name={request.name} address={request.address || undefined} key={request.id}/>
            ))}
          </div>}
      </div>
    </div>

  )
}

export default Home
