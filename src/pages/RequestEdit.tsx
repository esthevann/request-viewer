import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Link, navigate, usePathParams } from "raviger";
import RequestService from "../services/RequestService";



export default function RequestEditPage(){
    const client = useQueryClient();
    const path = usePathParams('/:requestId/edit');
    
    if (!path || !path.requestId){
        navigate('/')
        return <></>
    }

    const requestId = path.requestId;

    const {data, isLoading} = useQuery(['get_request_by_id', requestId], () => RequestService.get_request_by_id(requestId));

    
    return (
        <div className="flex flex-col items-center mt-3">
            <h1 className='text-3xl font-bold'>Edit Request</h1>
            <Link className="mt-2 mb-2" href="/">Go back</Link>
            <form action="" className="flex flex-col gap-1">
                <label htmlFor="" className="flex gap-1">
                    Name:&nbsp;&nbsp;&nbsp;
                    <input type="text" value={data?.name} className="border px-1" />
                </label>
                <label htmlFor="" className="flex gap-1">
                    Address:
                    <input type="text" value={data?.address || ''} className="border px-1" />
                </label>
                
            </form>
            <div className="pb-3"></div>
            <button className='bg-blue-500 text-white px-3 py-2 rounded-full'> New Request </button>
        </div>
    )
}