import { Link, navigate, usePathParams } from "raviger";
import useGetRequestById from "../hooks/useGetRequestById";



export default function RequestEditPage(){
    const path = usePathParams('/:requestId/edit');
    
    if (!path || !path.requestId){
        navigate('/')
        return <></>
    }

    const requestId = path.requestId;

    const {data, isLoading} = useGetRequestById(requestId);

    
    return (
        <div className="flex flex-col items-center mt-3">
            <h1 className='text-3xl font-bold'>Edit Request</h1>
            <Link className="mt-2 mb-2" href="/">Go back</Link>
            <form action="" className="flex flex-col gap-1">
                <label htmlFor="" className="flex gap-1">
                    Name:&nbsp;&nbsp;&nbsp;
                    <input type="text" value={data?.name} className="border px-1 text-black" />
                </label>
                <label htmlFor="" className="flex gap-1">
                    Address:
                    <input type="text" value={data?.address || ''} className="border px-1 text-black" />
                </label>
                
            </form>
            <div className="pb-3"></div>
            <button className='bg-blue-500 text-white px-3 py-2 rounded-full'> New Request </button>
        </div>
    )
}