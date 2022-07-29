import { Link, navigate, usePathParams } from "raviger";
import { FormEventHandler, useEffect, useState } from "react";
import Spinner from "../components/Spinner";
import useGetRequestById from "../hooks/useGetRequestById";
import useFetchRequestData from "../hooks/useFetchRequestData";
import useUpdateRequest from "../hooks/useUpdateRequest";



export default function RequestEditPage(){
    const path = usePathParams('/:requestId/edit');
    const requestId = path?.requestId || "";
    const {name, address, method, setName, setAdress, setMethod, isLoading} = useFetchRequestData(requestId);
    const requestUpdater = useUpdateRequest();
    
    if (!path || !path.requestId){
        navigate('/')
        return <></>
    }

    if (isLoading || name == "") {
        return (
            <div className="flex flex-col h-screen items-center justify-center mt-3">
                <Spinner />
            </div>
        )
    }


    const handleSubmit: FormEventHandler<HTMLFormElement> = (e) => {
        e.preventDefault();
        requestUpdater.mutate({ id: requestId, name, address}, {
            onSuccess: () => {
                navigate('/')
            }
        });
    }
    
    return (
        <div className="flex flex-col items-center mt-3">
            <h1 className='text-3xl font-bold'>Edit Request</h1>
            <Link className="mt-2 mb-2" href="/">Go back</Link>
            <form id="editForm" action=""  onSubmit={handleSubmit} className="flex flex-col gap-1">
                <label htmlFor="" className="flex gap-1">
                    Name:&nbsp;&nbsp;&nbsp;
                    <input type="text" value={name} onChange={(e) => setName(e.target.value)} className="border px-1 text-black" />
                </label>
                <label htmlFor="" className="flex gap-1">
                    Address:
                    <input type="text" value={address} onChange={(e) => setAdress(e.target.value)} className="border px-1 text-black" />
                </label>
                <label htmlFor="">
                    Method:&nbsp;
                    <select name="method" id="method" className="text-black" onChange={(e) => setMethod(e.target.value)}>
                        <option value="GET">GET</option>
                        <option value="POST">POST</option>
                        <option value="PUT">PUT</option>
                        <option value="PATCH">PATCH</option>
                    </select>
                </label>
            </form>
            <div className="pb-3"></div>
            <button form="editForm"  className='bg-blue-500 text-white px-3 py-2 rounded-full'> New Request </button>
        </div>
    )
}