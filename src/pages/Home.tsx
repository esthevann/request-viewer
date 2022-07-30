import { FormEventHandler, useState } from 'react';
import Request from '../components/Request';
import useRequests from '../hooks/useRequests';
import useCreateRequest from '../hooks/useCreateRequest';
import Spinner from '../components/Spinner';

function Home() {
  const [requestName, setRequestName] = useState("");

  const { data, isLoading } = useRequests();
  console.log(data);
  
  const requestMutation = useCreateRequest();

  
  const handleSubmit: FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();
    requestMutation.mutate({name: requestName, address: null}, {
      onSuccess: () => {
        setRequestName("");
      }
    });
  }

  return (
    <div className='flex flex-col gap-3 mt-3'>
      <form onSubmit={handleSubmit} className="flex flex-col items-center gap-3">
        <h1 className='text-3xl font-bold '>Request maker</h1>
        <label htmlFor="" className='flex gap-2'>
          Name:
          <input type="text" value={requestName} onChange={(e) => setRequestName(e.target.value)} className='border text-black px-1' spellCheck={false} />
        </label>
        <button disabled={isLoading || requestMutation.isLoading} className='bg-blue-500 text-white px-3 py-2 rounded-full hover:bg-blue-600'> New Request </button>
      </form>

      <div className='flex flex-col ml-3 gap-3 items-center'>
        <h2 className='text-2xl font-bold'>My Requests</h2>
        {isLoading && <div >
          <div className='pb-20'></div>
          <Spinner />
        </div>}
        {data &&
          <div>
            {data.sort((a, b) => {
              return (new Date(b.created_at)).getTime() - (new Date (a.created_at)).getTime();
            }).map(request => (
              <Request id={request.id} name={request.name} method={request.method} address={request.address || undefined} key={request.id} />
            ))}
          </div>}
      </div>
    </div>
  )
}

export default Home
